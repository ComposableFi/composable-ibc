#![allow(warnings)]

pub mod config;

pub mod client;

pub mod contract {
	use std::sync::Arc;

	use ethers::{
		abi::{Abi, Address, Detokenize},
		prelude::Contract,
		providers::Middleware,
	};

	pub const ABI_JSON: &str = include_str!("./contract-abi.json");

	/// Create a new contract instance from the given address and ABI.
	#[track_caller]
	pub fn contract<M>(address: Address, client: Arc<M>) -> Contract<M>
	where
		M: Middleware,
	{
		let abi: Abi = serde_json::from_str(ABI_JSON).unwrap();
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
		fn from_tokens(
			tokens: Vec<ethers::abi::Token>,
		) -> Result<Self, ethers::abi::InvalidOutputType>
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
		fn from_tokens(
			tokens: Vec<ethers::abi::Token>,
		) -> Result<Self, ethers::abi::InvalidOutputType>
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

	pub const LIGHT_CLIENT_ABI_JSON: &str = include_str!("./light-client-abi.json");

	/// Create a new contract instance from the given address and ABI.
	#[track_caller]
	pub fn light_client_contract<M>(address: Address, client: Arc<M>) -> Contract<M>
	where
		M: Middleware,
	{
		let abi: Abi = serde_json::from_str(LIGHT_CLIENT_ABI_JSON).unwrap();
		Contract::new(address, abi, client)
	}
}

pub mod chain;
pub mod ibc_provider;
pub mod key_provider;
pub mod light_client_sync;
