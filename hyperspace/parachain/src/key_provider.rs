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

use super::ParachainClient;
use primitives::KeyProvider;
use sp_core::crypto::Ss58Codec;
use sp_runtime::traits::IdentifyAccount;
use std::str::FromStr;

impl<T: light_client_common::config::Config> KeyProvider for ParachainClient<T> {
	fn account_id(&self) -> ibc::signer::Signer {
		let hex_string = self
			.public_key
			.clone()
			.into_account()
			.to_ss58check_with_version(self.ss58_version);

		ibc::signer::Signer::from_str(&hex_string).expect("Account Id should be valid")
	}
}
