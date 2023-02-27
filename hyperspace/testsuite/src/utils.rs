// Copyright 2022 ComposableFi
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::StreamExt;
use futures::future;
use hyperspace_primitives::{utils::timeout_after, TestProvider};
use ibc::events::IbcEvent;

pub async fn assert_timeout_packet<A>(chain: &A, blocks: u64)
where
	A: TestProvider,
	A::FinalityEvent: Send + Sync,
{
	// wait for the timeout packet
	let future = chain
		.ibc_events()
		.await
		.skip_while(|ev| {
			future::ready(!matches!(
				ev,
				IbcEvent::TimeoutPacket(_) | IbcEvent::TimeoutOnClosePacket(_)
			))
		})
		.take(1)
		.collect::<Vec<_>>();
	timeout_after(chain, future, blocks, format!("Didn't see Timeout packet on {}", chain.name()))
		.await;
}
