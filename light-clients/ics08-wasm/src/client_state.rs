use crate::{client_def::WasmClient, Bytes};
use alloc::{
	boxed::Box,
	string::{String, ToString},
	vec::Vec,
};
use core::{fmt::Debug, marker::PhantomData, time::Duration};
use ibc::{
	core::{
		ics02_client::{
			client_consensus::ConsensusState as IbcConsensusState,
			client_def::ClientDef,
			client_state::{ClientState as IbcClientState, ClientType},
		},
		ics24_host::identifier::ChainId,
	},
	protobuf::Protobuf,
	Height,
};
use ibc_proto::{
	google::protobuf::Any as RawAny, ibc::lightclients::wasm::v1::ClientState as RawClientState,
	ics23::ProofSpec,
};

pub const WASM_CLIENT_STATE_TYPE_URL: &str = "/ibc.lightclients.wasm.v1.ClientState";

#[derive(Clone, PartialEq, Debug, Eq)]
pub struct ClientState<AnyClient, AnyClientState, AnyConsensusState> {
	// #[schemars(with = "String")]
	// #[serde(with = "Base64", default)]
	pub data: Bytes,
	// #[schemars(with = "String")]
	// #[serde(with = "Base64", default)]
	pub code_id: Bytes,
	pub inner: Box<AnyClientState>,
	pub latest_height: Height,
	pub proof_specs: Vec<ProofSpec>,
	pub repository: String,
	pub _phantom: PhantomData<(AnyConsensusState, AnyClient)>,
}

impl<AnyClient, AnyClientState, AnyConsensusState> IbcClientState
	for ClientState<AnyClient, AnyClientState, AnyConsensusState>
where
	AnyClientState: for<'a> TryFrom<(ClientType, &'a Bytes)>,
	AnyConsensusState: IbcConsensusState + Eq,
	AnyConsensusState: for<'a> TryFrom<(ClientType, &'a Bytes)>,
	AnyClient: ClientDef<ClientState = AnyClientState, ConsensusState = AnyConsensusState>
		+ Debug
		+ Send
		+ Sync
		+ Eq,
	AnyClientState: IbcClientState<ClientDef = AnyClient> + Eq,
	AnyClient::ClientMessage: for<'a> TryFrom<(ClientType, &'a Bytes)>,
{
	type UpgradeOptions = Box<AnyClientState::UpgradeOptions>;
	type ClientDef = WasmClient<AnyClient, AnyClientState, AnyConsensusState>;

	fn chain_id(&self) -> ChainId {
		// self.inner.chain_id()
		self.inner.chain_id()
	}

	fn client_def(&self) -> Self::ClientDef {
		let inner = self.inner.client_def();
		WasmClient { inner: Box::new(inner), _phantom: Default::default() }
	}

	fn client_type(&self) -> String {
		"08-wasm".to_string()
	}

	fn latest_height(&self) -> Height {
		// TODO: use self.latest_height?
		// self.inner.latest_height()
		self.latest_height
	}

	fn frozen_height(&self) -> Option<Height> {
		self.inner.frozen_height()
	}

	fn upgrade(
		self,
		upgrade_height: Height,
		upgrade_options: Self::UpgradeOptions,
		chain_id: ChainId,
	) -> Self {
		let inner = self.inner.upgrade(upgrade_height, *upgrade_options, chain_id);
		Self { inner: Box::new(inner), ..self }
	}

	fn expired(&self, elapsed: Duration) -> bool {
		self.inner.expired(elapsed)
	}

	fn encode_to_vec(&self) -> Vec<u8> {
		self.encode_vec().unwrap()
	}
}

#[derive(Clone)]
pub struct Any {
	pub type_url: String,
	pub value: Bytes,
}

impl Protobuf<RawAny> for Any {}

impl From<RawAny> for Any {
	fn from(value: RawAny) -> Self {
		Self { type_url: value.type_url, value: value.value }
	}
}

impl From<Any> for RawAny {
	fn from(value: Any) -> Self {
		Self { type_url: value.type_url, value: value.value }
	}
}

impl<AnyClient, AnyClientState, AnyConsensusState> TryFrom<RawClientState>
	for ClientState<AnyClient, AnyClientState, AnyConsensusState>
where
	AnyClientState: for<'a> TryFrom<(ClientType, &'a Bytes)>,
	// Self::Error: for<'a> From<<AnyClientState as TryFrom<(ClientType, &'a Bytes)>>::Error>,
{
	type Error = String;

	fn try_from(raw: RawClientState) -> Result<Self, Self::Error> {
		let client_type = crate::get_wasm_client_type(&raw.code_id).expect(&format!(
			"WASM client type is not set for the code_id: {}",
			hex::encode(&raw.code_id)
		));
		// panic!("DATA  = {}", hex::encode(&raw.data));
		let any = Any::decode_vec(&raw.data).unwrap();
		let inner = AnyClientState::try_from((client_type, &any.value))
			.map_err(|_| ())
			// .map_err(|e| println!("Error: {:?}", e))?;
			.expect("Any* cannot be decoded");
		Ok(Self {
			data: raw.data,
			code_id: raw.code_id,
			inner: Box::new(inner),
			latest_height: raw
				.latest_height
				.map(|h| Height::new(h.revision_number, h.revision_height))
				.unwrap_or_default(),
			proof_specs: raw.proof_specs,
			repository: raw.repository,
			_phantom: Default::default(),
		})
	}
}

impl<AnyClient, AnyClientState, AnyConsensusState>
	From<ClientState<AnyClient, AnyClientState, AnyConsensusState>> for RawClientState
{
	fn from(client_state: ClientState<AnyClient, AnyClientState, AnyConsensusState>) -> Self {
		Self {
			data: client_state.data,
			code_id: client_state.code_id,
			latest_height: Some(client_state.latest_height.into()),
			proof_specs: client_state.proof_specs,
			repository: client_state.repository,
		}
	}
}

impl<AnyClient, AnyClientState, AnyConsensusState> Protobuf<RawClientState>
	for ClientState<AnyClient, AnyClientState, AnyConsensusState>
where
	AnyClientState: Clone,
	AnyClientState: for<'a> TryFrom<(ClientType, &'a Bytes)>,
	AnyConsensusState: Clone,
	AnyClient: Clone,
{
}

// impl<AnyConsensusState> From<ConsensusState<AnyConsensusState>> for RawClientState {
// 	fn from(value: ConsensusState<AnyConsensusState>) -> Self {
// 		Self {
// 			data: value.data,
// 			code_id: value.code_id,
// 			latest_height: None,
// 			proof_specs: value.proof_specs,
// 			repository: value.repository,
// 		}
// 	}
// }
