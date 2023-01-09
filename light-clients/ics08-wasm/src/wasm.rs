// use crate::proto::{
// 	MsgPushNewWasmCode as RawMsgPushNewWasmCode,
// 	MsgPushNewWasmCodeResponse as RawMsgPushNewWasmCodeResponse,
// };
// use ibc::protobuf::Protobuf;
// 
// pub struct MsgPushNewWasmCode {
// 	pub signer: String,
// 	pub code: Vec<u8>,
// }
// 
// impl MsgPushNewWasmCode {
// 	pub fn new(signer: String, code: Vec<u8>) -> Self {
// 		Self { signer, code }
// 	}
// }
// 
// impl From<RawMsgPushNewWasmCode> for MsgPushNewWasmCode {
// 	fn from(raw: RawMsgPushNewWasmCode) -> Self {
// 		Self { signer: raw.signer, code: raw.code }
// 	}
// }
// 
// impl Into<RawMsgPushNewWasmCode> for MsgPushNewWasmCode {
// 	fn into(self) -> RawMsgPushNewWasmCode {
// 		RawMsgPushNewWasmCode { signer: self.signer, code: self.code }
// 	}
// }
// 
// impl Protobuf<RawMsgPushNewWasmCode> for MsgPushNewWasmCode {}
// 
// pub struct MsgPushNewWasmCodeResponse {
// 	pub code_id: Vec<u8>,
// }
// 
// impl MsgPushNewWasmCodeResponse {
// 	pub fn new(code_id: Vec<u8>) -> Self {
// 		Self { code_id }
// 	}
// }
// 
// impl From<RawMsgPushNewWasmCodeResponse> for MsgPushNewWasmCodeResponse {
// 	fn from(raw: RawMsgPushNewWasmCodeResponse) -> Self {
// 		Self { code_id: raw.code_id }
// 	}
// }
// 
// impl Into<RawMsgPushNewWasmCodeResponse> for MsgPushNewWasmCodeResponse {
// 	fn into(self) -> RawMsgPushNewWasmCodeResponse {
// 		RawMsgPushNewWasmCodeResponse { code_id: self.code_id }
// 	}
// }
// 
// impl Protobuf<RawMsgPushNewWasmCodeResponse> for MsgPushNewWasmCodeResponse {}
