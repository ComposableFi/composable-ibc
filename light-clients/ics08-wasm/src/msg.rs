use crate::Bytes;
use alloc::string::ToString;
use core::str::FromStr;
use ibc::{
	protobuf::Protobuf,
	signer::{Signer, SignerError},
};
use ibc_proto::{
	google::protobuf::Any, ibc::lightclients::wasm::v1::MsgPushNewWasmCode as RawMsgPushNewWasmCode,
};

pub const WASM_PUSH_WASM_CODE_TYPE_URL: &str = "/ibc.lightclients.wasm.v1.MsgPushNewWasmCode";

#[derive(Clone, PartialEq, Debug, Eq)]
pub struct MsgPushNewWasmCode {
	pub code: Bytes,
	pub signer: Signer,
}

impl Protobuf<RawMsgPushNewWasmCode> for MsgPushNewWasmCode {}

impl From<MsgPushNewWasmCode> for RawMsgPushNewWasmCode {
	fn from(value: MsgPushNewWasmCode) -> Self {
		Self { code: value.code, signer: value.signer.to_string() }
	}
}

impl TryFrom<RawMsgPushNewWasmCode> for MsgPushNewWasmCode {
	type Error = SignerError;

	fn try_from(value: RawMsgPushNewWasmCode) -> Result<Self, Self::Error> {
		Ok(Self { code: value.code, signer: Signer::from_str(&value.signer)? })
	}
}

impl From<MsgPushNewWasmCode> for Any {
	fn from(value: MsgPushNewWasmCode) -> Self {
		Any {
			type_url: WASM_PUSH_WASM_CODE_TYPE_URL.to_string(),
			value: value.encode_vec().unwrap(),
		}
	}
}
