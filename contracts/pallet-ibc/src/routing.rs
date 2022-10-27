use super::*;
use core::borrow::Borrow;
use ibc::{
	applications::transfer::MODULE_ID_STR as IBC_TRANSFER_MODULE_ID,
	core::{
		ics24_host::identifier::PortId,
		ics26_routing::context::{Ics26Context, Module, ModuleId, ReaderContext, Router},
	},
};
use scale_info::prelude::string::ToString;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Context<T: Config> {
	pub _pd: PhantomData<T>,
	router: IbcRouter<T>,
}

impl<T: Config + Send + Sync> Default for Context<T> {
	fn default() -> Self {
		Self { _pd: PhantomData::default(), router: IbcRouter::default() }
	}
}

impl<T: Config + Send + Sync> Context<T> {
	pub fn new() -> Self {
		Self::default()
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IbcRouter<T: Config> {
	pallet_ibc_ping: pallet_ibc_ping::IbcModule<T>,
	ibc_transfer: ics20::IbcModule<T>,
}

impl<T: Config> Default for IbcRouter<T> {
	fn default() -> Self {
		Self {
			pallet_ibc_ping: pallet_ibc_ping::IbcModule::<T>::default(),
			ibc_transfer: ics20::IbcModule::<T>::default(),
		}
	}
}

/// Module routing abstraction for down substrate runtimes. Notice the lack of &self in the methods,
/// this means routes need to be statically defined
pub trait ModuleRouter {
	/// Returns a mutable reference to a `Module` registered against the specified `ModuleId`
	fn get_route_mut(module_id: &impl Borrow<ModuleId>) -> Option<&mut dyn Module>;
	/// Returns true if the `Router` has a `Module` registered against the specified `ModuleId`
	fn has_route(module_id: &impl Borrow<ModuleId>) -> bool;
	/// Should return the module_id associated with a given port_id
	fn lookup_module_by_port(port_id: &PortId) -> Option<ModuleId>;
}

impl<T: Config + Send + Sync> Router for IbcRouter<T>
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
	T::Balance: From<u128>,
{
	fn get_route_mut(&mut self, module_id: &impl Borrow<ModuleId>) -> Option<&mut dyn Module> {
		// check if the user has defined any custom routes
		if let Some(module) = T::Router::get_route_mut(module_id) {
			return Some(module)
		}

		match module_id.borrow().to_string().as_str() {
			pallet_ibc_ping::MODULE_ID => Some(&mut self.pallet_ibc_ping),
			IBC_TRANSFER_MODULE_ID => Some(&mut self.ibc_transfer),
			&_ => None,
		}
	}

	fn has_route(&self, module_id: &impl Borrow<ModuleId>) -> bool {
		// check if the user has defined any custom routes
		if T::Router::has_route(module_id) {
			return true
		}

		matches!(
			module_id.borrow().to_string().as_str(),
			pallet_ibc_ping::MODULE_ID | IBC_TRANSFER_MODULE_ID
		)
	}
}

impl<T: Config + Send + Sync> Ics26Context for Context<T>
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
	T::Balance: From<u128>,
{
	type Router = IbcRouter<T>;

	fn router(&self) -> &Self::Router {
		&self.router
	}

	fn router_mut(&mut self) -> &mut Self::Router {
		&mut self.router
	}
}

impl<T: Config + Send + Sync> ReaderContext for Context<T>
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
	T::Balance: From<u128>,
{
}
