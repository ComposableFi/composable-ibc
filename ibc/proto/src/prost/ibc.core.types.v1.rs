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

/// GenesisState defines the ibc module's genesis state.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// ICS002 - Clients genesis state
    #[prost(message, optional, tag="1")]
    pub client_genesis: ::core::option::Option<super::super::client::v1::GenesisState>,
    /// ICS003 - Connections genesis state
    #[prost(message, optional, tag="2")]
    pub connection_genesis: ::core::option::Option<super::super::connection::v1::GenesisState>,
    /// ICS004 - Channel genesis state
    #[prost(message, optional, tag="3")]
    pub channel_genesis: ::core::option::Option<super::super::channel::v1::GenesisState>,
}
