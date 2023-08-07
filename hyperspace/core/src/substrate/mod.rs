pub mod macros;

pub mod composable;
// pub mod dali;
pub mod default;
pub mod picasso_kusama;
pub mod picasso_rococo;

pub use composable::ComposableConfig;
// pub use dali::DaliConfig;
pub use default::DefaultConfig;
pub use picasso_kusama::PicassoKusamaConfig;
pub use picasso_rococo::PicassoRococoConfig;

use codec::{Decode, Encode};
use light_client_common::config::{AsInner, BeefyAuthoritySetT};
use sp_core::H256;

#[derive(Encode, Decode)]
pub struct DummyBeefyAuthoritySet;

impl BeefyAuthoritySetT for DummyBeefyAuthoritySet {
	fn root(&self) -> H256 {
		unimplemented!("DummyBeefyAuthoritySet root")
	}

	fn len(&self) -> u32 {
		unimplemented!("DummyBeefyAuthoritySet len")
	}
}

impl AsInner for DummyBeefyAuthoritySet {
	type Inner = ();

	fn from_inner(_inner: Self::Inner) -> Self {
		Self
	}
}

pub fn unimplemented<T>(s: &'static str) -> T {
	unimplemented!("{s}")
}
