use futures::{Stream, StreamExt};
use std::{
	fmt::Debug,
	ops::Deref,
	pin::Pin,
	sync::{Arc, Mutex},
	task::Poll,
};
use tokio::sync::watch;
use tokio_stream::wrappers::WatchStream;

#[derive(Debug)]
/// Keeps the most recent value of a stream and acts as stream itself.
pub struct RecentStream<T: Send + Sync + 'static> {
	pub value: Arc<watch::Receiver<Option<T>>>,
	// Arc<Mutex<Option<Option<T>>>>,
}

impl<T: Send + Sync + 'static + Clone + Debug> RecentStream<T> {
	pub fn new(mut stream: impl Stream<Item = T> + Send + Unpin + 'static) -> Self {
		let (tx, rx) = watch::channel(None);
		tokio::spawn(async move {
			while let Some(v) = dbg!(stream.next().await) {
				tx.send(Some(v)).unwrap();
			}
		});
		Self { value: Arc::new(rx) }
	}
}

impl<T: Clone + 'static + Send + Sync + Debug> Stream for RecentStream<T> {
	type Item = T;

	fn poll_next(
		self: Pin<&mut Self>,
		cx: &mut std::task::Context<'_>,
	) -> Poll<Option<Self::Item>> {
		if self.value.has_changed().is_err() {
			return Poll::Ready(None)
		}
		let v = WatchStream::new((*self.value).clone());
		tokio::pin!(v);
		match v.poll_next(cx) {
			Poll::Ready(Some(None)) => Poll::Pending,
			Poll::Pending => Poll::Pending,
			Poll::Ready(Some(data)) => Poll::Ready(data),
			Poll::Ready(None) => Poll::Ready(None),
		}
	}
}

mod tests {

	use super::*;
	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn recent_stream_test() {
		// ensure that we:
		// - only read latest value, and reads it only once
		// - upon stream ends, gets a None
		let (tx, rx) = tokio::sync::mpsc::channel(4);
		let receiver_stream = tokio_stream::wrappers::ReceiverStream::new(rx);
		let mut recent_stream = RecentStream::new(receiver_stream);

		tx.send("booba".to_string()).await;
		tx.send("hello".to_string()).await;
		tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
		assert_eq!(recent_stream.next().await, Some("hello".to_string()));
		tx.send("goodbye".to_string()).await;
		tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
		assert_eq!(recent_stream.next().await, Some("goodbye".to_string()));
		drop(tx);
		tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
		assert_eq!(recent_stream.next().await, None);
	}
}
