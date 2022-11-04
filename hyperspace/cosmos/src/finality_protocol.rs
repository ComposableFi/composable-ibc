//! Light client protocols for cosmos chains.

use crate::CosmosClient;
use codec::{Decode, Encode};
use ibc::events::IbcEvent;
use ibc_proto::google::protobuf::Any;
use ics07_tendermint::client_message::Header;

use primitives::{Chain, KeyProvider, UpdateType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FinalityProtocol {
	Tendermint,
}

/// Finality event for cosmos chains.
pub enum FinalityEvent {
	Tendermint(Header),
}

impl FinalityProtocol {
	pub async fn query_latest_ibc_events<C, H>(
		&self,
		source: &mut CosmosClient<H>,
		finality_event: FinalityEvent,
		counterparty: &C,
	) -> Result<(Any, Vec<IbcEvent>, UpdateType), anyhow::Error>
	where
		H: Clone + Send + Sync + 'static,
		C: Chain,
		CosmosClient<H>: Chain + KeyProvider,
	{
		match self {
			FinalityProtocol::Tendermint =>
				query_latest_ibc_events_with_tendermint(source, finality_event, counterparty).await,
		}
	}
}

/// Query the latest events that have been finalized by the Tendermint finality protocol.
pub async fn query_latest_ibc_events_with_tendermint<C, H>(
	source: &mut CosmosClient<H>,
	finality_event: FinalityEvent,
	counterparty: &C,
) -> Result<(Any, Vec<IbcEvent>, UpdateType), anyhow::Error>
where
	C: Chain,
	H: Clone + Send + Sync + 'static,
	CosmosClient<H>: Chain + KeyProvider,
{
	todo!()
}
