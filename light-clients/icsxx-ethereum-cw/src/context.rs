use cosmwasm_std::{DepsMut, Env, Storage};
use ibc::core::ics26_routing::context::ReaderContext;
use std::{fmt, fmt::Debug, marker::PhantomData};
use sync_committee_verifier::BlsVerify;

pub struct Context<'a, H> {
	pub deps: DepsMut<'a>,
	pub env: Env,
	_phantom: PhantomData<H>,
}

impl<'a, H> PartialEq for Context<'a, H> {
	fn eq(&self, _other: &Self) -> bool {
		true
	}
}

impl<'a, H> Eq for Context<'a, H> {}

impl<'a, H> Debug for Context<'a, H> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Context {{ deps: DepsMut }}")
	}
}

impl<'a, H> Clone for Context<'a, H> {
	fn clone(&self) -> Self {
		panic!("Context is not cloneable")
	}
}

impl<'a, H> Context<'a, H> {
	pub fn new(deps: DepsMut<'a>, env: Env) -> Self {
		Self { deps, _phantom: Default::default(), env }
	}

	pub fn log(&self, msg: &str) {
		self.deps.api.debug(msg)
	}

	pub fn storage(&self) -> &dyn Storage {
		self.deps.storage
	}

	pub fn storage_mut(&mut self) -> &mut dyn Storage {
		self.deps.storage
	}
}

impl<'a, H> ReaderContext for Context<'a, H> where
	H: Clone + Eq + Send + Sync + Debug + Default + BlsVerify + 'static
{
}
