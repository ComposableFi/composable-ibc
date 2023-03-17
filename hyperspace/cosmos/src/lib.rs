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

use ibc::core::ics02_client::height::Height;

pub mod chain;
pub mod client;
pub mod encode;
pub mod error;
pub mod events;
pub mod key_provider;
pub mod light_client;
pub mod provider;
#[cfg(any(test, feature = "testing"))]
pub mod test_provider;
pub mod tx;

pub type TimeoutHeight = Option<Height>;
