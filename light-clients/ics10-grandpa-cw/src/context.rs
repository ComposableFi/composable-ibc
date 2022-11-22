use std::{cell::RefCell, fmt, fmt::Debug, marker::PhantomData, rc::Rc};

use cosmwasm_std::DepsMut;

use grandpa_light_client_primitives::HostFunctions;
use ibc::core::ics26_routing::context::ReaderContext;

pub struct Context<'a, H> {
	pub deps: Rc<RefCell<DepsMut<'a>>>,
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
	pub fn new(deps: Rc<RefCell<DepsMut<'a>>>) -> Self {
		Self { deps, _phantom: Default::default() }
	}
}

impl<'a, H: HostFunctions> ReaderContext for Context<'a, H> {}
