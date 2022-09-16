use crate::{
	client_def::BeefyClient,
	client_state::{ClientState as BeefyClientState, UpgradeOptions as BeefyUpgradeOptions},
	consensus_state::ConsensusState as BeefyConsensusState,
	header::BeefyHeader,
};

use crate::any::mock::context::Crypto;
use core::{convert::Infallible, time::Duration};
use ibc::{
	core::{
		ics02_client::{
			client_consensus::ConsensusState,
			client_def::{ClientDef, ConsensusUpdateResult},
			client_state::{ClientState, ClientType},
			error::Error,
			header::Header,
			height::Height,
			misbehaviour::Misbehaviour,
		},
		ics03_connection::connection::ConnectionEnd,
		ics04_channel::{
			channel::ChannelEnd,
			commitment::{AcknowledgementCommitment, PacketCommitment},
			packet::Sequence,
		},
		ics23_commitment::commitment::{CommitmentPrefix, CommitmentProofBytes, CommitmentRoot},
		ics24_host::identifier::{ChainId, ChannelId, ClientId, ConnectionId, PortId},
		ics26_routing::context::ReaderContext,
	},
	downcast,
	mock::{
		client_def::MockClient,
		client_state::{MockClientState, MockConsensusState},
		header::MockHeader,
		misbehaviour::MockMisbehaviour,
	},
	prelude::*,
	timestamp::Timestamp,
};
use ibc_proto::google::protobuf::Any;
use tendermint_proto::Protobuf;

pub const MOCK_CLIENT_STATE_TYPE_URL: &str = "/ibc.mock.ClientState";
pub const MOCK_HEADER_TYPE_URL: &str = "/ibc.mock.Header";
pub const MOCK_MISBEHAVIOUR_TYPE_URL: &str = "/ibc.mock.Misbehavior";
pub const MOCK_CONSENSUS_STATE_TYPE_URL: &str = "/ibc.mock.ConsensusState";

pub const BEEFY_CLIENT_STATE_TYPE_URL: &str = "/ibc.lightclients.beefy.v1.ClientState";
pub const BEEFY_HEADER_TYPE_URL: &str = "/ibc.lightclients.beefy.v1.Header";
pub const BEEFY_CONSENSUS_STATE_TYPE_URL: &str = "/ibc.lightclients.beefy.v1.ConsensusState";

#[derive(Clone, Debug, PartialEq, Eq, ClientDef)]
pub enum AnyClient {
	Mock(MockClient),
	Beefy(BeefyClient<Crypto>),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AnyUpgradeOptions {
	Mock(()),
	Beefy(BeefyUpgradeOptions),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, ClientState, Protobuf)]
#[serde(tag = "type")]
pub enum AnyClientState {
	#[ibc(proto_url = "MOCK_CLIENT_STATE_TYPE_URL")]
	Mock(MockClientState),
	#[serde(skip)]
	#[ibc(proto_url = "BEEFY_CLIENT_STATE_TYPE_URL")]
	Beefy(BeefyClientState<Crypto>),
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Header, Protobuf)]
#[allow(clippy::large_enum_variant)]
pub enum AnyHeader {
	#[ibc(proto_url = "MOCK_HEADER_TYPE_URL")]
	Mock(MockHeader),
	#[serde(skip)]
	#[ibc(proto_url = "BEEFY_HEADER_TYPE_URL")]
	Beefy(BeefyHeader),
}

#[derive(Clone, Debug, PartialEq, Misbehaviour, Protobuf)]
#[allow(clippy::large_enum_variant)]
pub enum AnyMisbehaviour {
	#[ibc(proto_url = "MOCK_MISBEHAVIOUR_TYPE_URL")]
	Mock(MockMisbehaviour),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, ConsensusState, Protobuf)]
#[serde(tag = "type")]
pub enum AnyConsensusState {
	#[ibc(proto_url = "BEEFY_CONSENSUS_STATE_TYPE_URL")]
	Beefy(BeefyConsensusState),
	#[ibc(proto_url = "MOCK_CONSENSUS_STATE_TYPE_URL")]
	Mock(MockConsensusState),
}
