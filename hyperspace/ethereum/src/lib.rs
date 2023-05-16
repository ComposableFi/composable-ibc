#![allow(warnings)]

pub mod config;

pub mod client;

pub mod contract {
	use std::sync::Arc;

	use ethers::{
		abi::{Abi, Address},
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
}

pub mod chain;
pub mod ibc_provider;
pub mod key_provider;
pub mod light_client_sync;
