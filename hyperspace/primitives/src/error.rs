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

use thiserror::Error;

#[derive(Error, Debug)]
/// Error definition for the relayer
pub enum Error {
	/// subxt error
	#[error("Subxt basic error")]
	Subxt(#[from] subxt::Error),
	/// subxt rpc error
	#[error("Subxt rpc error")]
	SubxtRRpc(#[from] subxt::error::RpcError),
	/// Custom error
	#[error("{0}")]
	Custom(String),
	/// Scale codec error
	#[error("Scale decoding error")]
	Codec(#[from] codec::Error),
	/// Ibc client error
	#[error("Ibc client error")]
	IbcClientError(#[from] ibc::core::ics02_client::error::Error),
	#[error("Ibc channel error")]
	IbcChannelError(#[from] ibc::core::ics04_channel::error::Error),
	#[error("Ibc connection error")]
	IbcConnectionError(#[from] ibc::core::ics03_connection::error::Error),
	#[error("Ibc proof error")]
	IbcProofError(#[from] ibc::proofs::ProofError),
	#[error("Hex decode error")]
	HexDecode(#[from] hex::FromHexError),
}

impl From<String> for Error {
	fn from(error: String) -> Self {
		Self::Custom(error)
	}
}
