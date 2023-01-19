use crate::Bytes;
use alloc::{
	boxed::Box,
	string::{String, ToString},
	vec::Vec,
};
use core::{convert::Infallible, fmt::Debug};
use ibc::{
	core::{
		ics02_client::{
			client_consensus::ConsensusState as IbcConsensusState, client_state::ClientType,
		},
		ics23_commitment::commitment::CommitmentRoot,
	},
	protobuf::Protobuf,
	timestamp::Timestamp,
};
use ibc_proto::{
	google::protobuf::Any,
	ibc::{
		core::{client::v1::Height, commitment::v1::MerkleRoot},
		lightclients::wasm::v1::ConsensusState as RawConsensusState,
	},
	ics23::ProofSpec,
};
use prost::Message;

pub const WASM_CONSENSUS_STATE_TYPE_URL: &str = "/ibc.lightclients.wasm.v1.ConsensusState";

// #[cw_serde]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConsensusState<AnyConsensusState> {
	// #[schemars(with = "String")]
	// #[serde(with = "Base64", default)]
	pub data: Bytes,
	// #[schemars(with = "String")]
	// #[serde(with = "Base64", default)]
	pub code_id: Bytes,
	pub timestamp: u64,
	pub root: CommitmentRoot,
	pub inner: Box<AnyConsensusState>,
}

impl<AnyConsensusState: IbcConsensusState> IbcConsensusState for ConsensusState<AnyConsensusState>
where
	AnyConsensusState: Clone + Debug + Send + Sync,
	AnyConsensusState: TryFrom<Any>,
{
	type Error = Infallible;

	fn root(&self) -> &CommitmentRoot {
		// TODO: use self.root?
		// self.inner.root()
		&self.root
	}

	fn timestamp(&self) -> Timestamp {
		// TODO: use self.timestamp?
		// self.inner.timestamp()
		Timestamp::from_nanoseconds(self.timestamp.saturating_mul(1000000000)).unwrap()
	}

	fn encode_to_vec(&self) -> Vec<u8> {
		self.encode_vec().unwrap()
	}
}

impl<AnyConsensusState> ConsensusState<AnyConsensusState>
where
	AnyConsensusState: Clone + Debug + Send + Sync,
	AnyConsensusState: TryFrom<Any> + IbcConsensusState,
{
	pub fn to_any(&self) -> Any {
		Any { type_url: WASM_CONSENSUS_STATE_TYPE_URL.to_string(), value: self.encode_to_vec() }
	}
}

impl<AnyConsensusState> TryFrom<RawConsensusState> for ConsensusState<AnyConsensusState>
where
	AnyConsensusState: TryFrom<Any>,
{
	type Error = String;

	fn try_from(raw: RawConsensusState) -> Result<Self, Self::Error> {
		#[cfg(feature = "std")]
		println!("RawConsensusState.code_id = {}", hex::encode(&raw.code_id));
		#[cfg(feature = "std")]
		println!("RawConsensusState.data = {}", hex::encode(&raw.data));
		let any = Any::decode(&mut &raw.data[..]).unwrap();
		let inner = AnyConsensusState::try_from(any)
			.map_err(|_| ())
			.expect("Any* cannot be decoded");
		Ok(Self {
			data: raw.data,
			code_id: raw.code_id,
			timestamp: raw.timestamp,
			root: CommitmentRoot::from_bytes(&raw.root.unwrap().hash),
			inner: Box::new(inner),
		})
	}
}

impl<AnyConsensusState> From<ConsensusState<AnyConsensusState>> for RawConsensusState {
	fn from(value: ConsensusState<AnyConsensusState>) -> Self {
		Self {
			data: value.data,
			code_id: value.code_id,
			timestamp: value.timestamp,
			root: Some(value.root.into()),
		}
	}
}

impl<AnyConsensusState> Protobuf<RawConsensusState> for ConsensusState<AnyConsensusState>
where
	AnyConsensusState: Clone,
	AnyConsensusState: TryFrom<Any>,
{
}
