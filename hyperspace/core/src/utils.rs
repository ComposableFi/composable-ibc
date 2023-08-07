use futures::{Stream, StreamExt};
use std::{
	pin::Pin,
	sync::{Arc, Mutex},
	task::Poll,
};

/// Keeps the most recent value of a stream and acts as stream itself.
pub struct RecentStream<T: Send + 'static> {
	value: Arc<Mutex<Option<Option<T>>>>,
}

impl<T: Send + 'static> RecentStream<T> {
	pub fn new(mut stream: impl Stream<Item = T> + Send + Unpin + 'static) -> Self {
		let value = Arc::new(Mutex::new(Some(None)));
		let value_cloned = value.clone();
		tokio::spawn(async move {
			while let Some(v) = stream.next().await {
				*value_cloned.lock().unwrap() = Some(Some(v));
			}
			*value_cloned.lock().unwrap() = None;
		});
		Self { value }
	}
}

impl<T: Send> Stream for RecentStream<T> {
	type Item = T;

	fn poll_next(
		self: Pin<&mut Self>,
		cx: &mut std::task::Context<'_>,
	) -> Poll<Option<Self::Item>> {
		let this = self.get_mut();
		let mut value = this.value.lock().unwrap();
		match value.as_mut() {
			Some(v) => match v.take() {
				Some(v) => Poll::Ready(Some(v)),
				None => {
					cx.waker().wake_by_ref();
					Poll::Pending
				},
			},
			None => Poll::Ready(None),
		}
	}
}
