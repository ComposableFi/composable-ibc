use crate::{client_def::WasmClient, Bytes};
use ibc::{
	core::{
		ics02_client::client_state::ClientState as IbcClientState, ics24_host::identifier::ChainId,
	},
	protobuf::Protobuf,
	Height,
};
use ibc_proto::ibc::lightclients::wasm::v1::ClientState as RawClientState;
use std::time::Duration;

pub const WASM_CLIENT_STATE_TYPE_URL: &str = "/ibc.lightclients.wasm.v1.ClientState";

#[derive(Clone, PartialEq, Debug, Eq)]
pub struct ClientState {
	// #[schemars(with = "String")]
	// #[serde(with = "Base64", default)]
	pub data: Bytes,
	// #[schemars(with = "String")]
	// #[serde(with = "Base64", default)]
	pub code_id: Bytes,
	// pub latest_height: HeightRaw,
	// pub proof_specs: Vec<ProofSpec>,
	// pub repository: String,
}

impl IbcClientState for ClientState {
	type UpgradeOptions = ();
	type ClientDef = WasmClient;

	fn chain_id(&self) -> ChainId {
		todo!()
	}

	fn client_def(&self) -> Self::ClientDef {
		todo!()
	}

	fn client_type(&self) -> String {
		"08-wasm".to_string()
	}

	fn latest_height(&self) -> Height {
		todo!()
	}

	fn frozen_height(&self) -> Option<Height> {
		todo!()
	}

	fn upgrade(
		self,
		_upgrade_height: Height,
		_upgrade_options: Self::UpgradeOptions,
		_chain_id: ChainId,
	) -> Self {
		todo!()
	}

	fn expired(&self, _elapsed: Duration) -> bool {
		todo!()
	}

	fn encode_to_vec(&self) -> Vec<u8> {
		todo!()
	}
}

impl From<RawClientState> for ClientState {
	fn from(raw: RawClientState) -> Self {
		Self { data: raw.data, code_id: raw.code_id }
	}
}

impl From<ClientState> for RawClientState {
	fn from(client_state: ClientState) -> Self {
		Self {
			data: client_state.data,
			code_id: client_state.code_id,
			latest_height: None,
			proof_specs: vec![],
			repository: "".to_string(),
		}
	}
}

impl Protobuf<RawClientState> for ClientState {}
