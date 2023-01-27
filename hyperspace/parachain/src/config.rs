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

use async_trait::async_trait;
use subxt::{config::ExtrinsicParams, OnlineClient};

pub type CustomExtrinsicParams<T> = <<T as subxt::Config>::ExtrinsicParams as ExtrinsicParams<
	<T as subxt::Config>::Index,
	<T as subxt::Config>::Hash,
>>::OtherParams;

/// This allows end users of this crate return the correct extrinsic metadata required by their
/// runtimes into the transactions signed by this crate.
#[async_trait]
pub trait Config: subxt::Config + Sized {
	/// Asset Id type used by the parachain runtime
	type AssetId: codec::Codec + serde::Serialize + Send + Sync + 'static;
	/// the signature type of the runtime
	type Signature: sp_runtime::traits::Verify + From<<Self as subxt::Config>::Signature>;
	/// Address type used by the runtime;
	type Address: codec::Codec + From<<Self as subxt::Config>::Address>;
	/// use the subxt client to fetch any neccessary data needed for the extrinsic metadata.
	async fn custom_extrinsic_params(
		client: &OnlineClient<Self>,
	) -> Result<CustomExtrinsicParams<Self>, subxt::Error>;
}
