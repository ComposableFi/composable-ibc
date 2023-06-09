use std::sync::Arc;

use ethers::{
	abi::{Abi, Address, Detokenize, Token},
	prelude::Contract,
	providers::Middleware,
};

/// Unwraps a contract error, decoding the revert reason if possible
pub trait UnwrapContractError<T> {
	fn unwrap_contract_error(self) -> T;
}

impl<T, M> UnwrapContractError<T> for Result<T, ethers::prelude::ContractError<M>>
where
	M: Middleware,
{
	/// Unwraps a contract error, decoding the revert reason if possible
	#[track_caller]
	fn unwrap_contract_error(self) -> T {
		match self {
			Ok(t) => t,
			Err(ethers::prelude::ContractError::Revert(bytes)) => {
				// abi decode the bytes after the first 4 bytes (the error selector)
				if bytes.len() < 4 {
					panic!("contract-error: {:?}", bytes);
				}
				let bytes = &bytes[4..];
				let tokens = ethers::abi::decode(&[ethers::abi::ParamType::String], bytes).unwrap();
				panic!("contract-error: {tokens:#?}")
			},
			Err(e) => panic!("contract-error: {:?}", e),
		}
	}
}


pub const IBC_HANDLER_ABI: &str = include_str!("./abi/ibc-handler-abi.json");

/// Create a new contract instance from the given address and ABI.
#[track_caller]
pub fn ibc_handler<M>(address: Address, client: Arc<M>) -> Contract<M>
where
	M: Middleware,
{
	let abi: Abi = serde_json::from_str(IBC_HANDLER_ABI).unwrap();
	Contract::new(address, abi, client)
}

pub(crate) struct Counterparty {
	pub(crate) client_id: String,
	pub(crate) connection_id: String,
	pub(crate) prefix: Vec<u8>,
}

pub(crate) struct Version {
	pub(crate) identifier: String,
	pub(crate) features: Vec<String>,
}

pub struct ChannelEnd {
	pub(crate) state: u32,
	pub(crate) ordering: u32,
	pub(crate) counterparty: Counterparty,
	pub(crate) connection_hops: Vec<String>,
	pub(crate) version: String,
}

impl Detokenize for ChannelEnd {
	fn from_tokens(tokens: Vec<ethers::abi::Token>) -> Result<Self, ethers::abi::InvalidOutputType>
	where
		Self: Sized,
	{
		let vec = tokens[3].clone().into_array().unwrap();

		let connection_hops = {
			let mut it = vec.into_iter();
			let mut v = vec![];

			while let Some(connection_id) = it.next() {
				v.push(connection_id.into_string().unwrap())
			}

			v
		};

		Ok(ChannelEnd {
			state: tokens[0].clone().into_uint().unwrap().as_u32(),
			ordering: tokens[1].clone().into_uint().unwrap().as_u32(),
			counterparty: Counterparty {
				client_id: tokens[2].clone().into_string().unwrap(),
				connection_id: tokens[3].clone().into_string().unwrap(),
				prefix: tokens[4].clone().into_bytes().unwrap(),
			},
			connection_hops,
			version: tokens[5].clone().into_string().unwrap(),
		})
	}
}

pub struct ConnectionEnd {
	pub(crate) client_id: String,
	pub(crate) versions: Vec<Version>,
	pub(crate) state: u32,
	pub(crate) counterparty: Counterparty,
	pub(crate) delay_period: u64,
}

impl Detokenize for ConnectionEnd {
	fn from_tokens(tokens: Vec<ethers::abi::Token>) -> Result<Self, ethers::abi::InvalidOutputType>
	where
		Self: Sized,
	{
		let vec = tokens[1].clone().into_array().unwrap();

		let versions = {
			let mut it = vec.into_iter();
			let mut v = vec![];

			while let (Some(identifier), Some(features)) = (it.next(), it.next()) {
				v.push(Version {
					identifier: identifier.into_string().unwrap(),
					features: features
						.into_array()
						.unwrap()
						.into_iter()
						.map(|t| t.into_string().unwrap())
						.collect(),
				})
			}

			v
		};

		Ok(ConnectionEnd {
			client_id: tokens[0].clone().into_string().unwrap(),
			versions,
			state: tokens[2].clone().into_uint().unwrap().as_u32(),
			counterparty: Counterparty {
				client_id: tokens[3].clone().into_string().unwrap(),
				connection_id: tokens[4].clone().into_string().unwrap(),
				prefix: tokens[5].clone().into_bytes().unwrap(),
			},
			delay_period: tokens[4].clone().into_uint().unwrap().as_u64(),
		})
	}
}

pub const LIGHT_CLIENT_ABI_JSON: &str = include_str!("./abi/light-client-abi.json");

/// Create a new contract instance from the given address and ABI.
#[track_caller]
pub fn light_client_contract<M>(address: Address, client: Arc<M>) -> Contract<M>
where
	M: Middleware,
{
	let abi: Abi = serde_json::from_str(LIGHT_CLIENT_ABI_JSON).unwrap();
	Contract::new(address, abi, client)
}
