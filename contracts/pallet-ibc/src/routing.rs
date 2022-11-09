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

use super::*;
use core::{borrow::Borrow, fmt::Debug};
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
	ibc_transfer: ics20::IbcModule<T>,
	sub_router: T::Router,
}

impl<T: Config> Default for IbcRouter<T> {
	fn default() -> Self {
		Self { ibc_transfer: ics20::IbcModule::<T>::default(), sub_router: Default::default() }
	}
}

/// Module routing abstraction for downstream substrate runtimes.
pub trait ModuleRouter: Default + Clone + Eq + PartialEq + Debug {
	/// Returns a mutable reference to a `Module` registered against the specified `ModuleId`
	fn get_route_mut(&mut self, module_id: &impl Borrow<ModuleId>) -> Option<&mut dyn Module>;
	/// Returns true if the `Router` has a `Module` registered against the specified `ModuleId`
	fn has_route(module_id: &impl Borrow<ModuleId>) -> bool;
	/// Should return the module_id associated with a given port_id
	fn lookup_module_by_port(port_id: &PortId) -> Option<ModuleId>;
}

impl<T: Config + Send + Sync> Router for IbcRouter<T>
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
{
	fn get_route_mut(&mut self, module_id: &impl Borrow<ModuleId>) -> Option<&mut dyn Module> {
		// check if the user has defined any custom routes
		if let Some(module) = self.sub_router.get_route_mut(module_id) {
			return Some(module)
		}

		match module_id.borrow().to_string().as_str() {
			IBC_TRANSFER_MODULE_ID => Some(&mut self.ibc_transfer),
			&_ => None,
		}
	}

	fn has_route(&self, module_id: &impl Borrow<ModuleId>) -> bool {
		// check if the user has defined any custom routes
		if T::Router::has_route(module_id) {
			return true
		}

		matches!(module_id.borrow().to_string().as_str(), IBC_TRANSFER_MODULE_ID,)
	}
}

impl<T: Config + Send + Sync> Ics26Context for Context<T>
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
{
	type Router = IbcRouter<T>;

	fn router(&self) -> &Self::Router {
		&self.router
	}

	fn router_mut(&mut self) -> &mut Self::Router {
		&mut self.router
	}
}

impl<T: Config + Send + Sync> ReaderContext for Context<T> where
	u32: From<<T as frame_system::Config>::BlockNumber>
{
}
