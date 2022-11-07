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

use crate::routing::{Context, ModuleRouter};
use ibc::{
	applications::transfer::{
		MODULE_ID_STR as TRANSFER_MODULE_ID, PORT_ID_STR as TRANSFER_PORT_ID,
	},
	core::{
		ics05_port::{context::PortReader, error::Error as ICS05Error},
		ics24_host::identifier::PortId,
		ics26_routing::context::ModuleId,
	},
};

impl<T: Config + Sync + Send> PortReader for Context<T> {
	fn lookup_module_by_port(&self, port_id: &PortId) -> Result<ModuleId, ICS05Error> {
		// check if the user has defined any custom modules
		if let Some(module_id) = T::Router::lookup_module_by_port(port_id) {
			return Ok(module_id)
		}

		match port_id.as_str() {
			TRANSFER_PORT_ID => Ok(ModuleId::from_str(TRANSFER_MODULE_ID)
				.map_err(|_| ICS05Error::module_not_found(port_id.clone()))?),
			_ => Err(ICS05Error::module_not_found(port_id.clone())),
		}
	}
}
