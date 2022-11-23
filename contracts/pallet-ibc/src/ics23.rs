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

//! Uses substrate's child trie api for ICS23 vector commitment.
//! This allows us to progressively mutate the trie and recalculate its root in O(log n).
//!
//! A much better approach than having to reconstruct the trie every time its changed
//! just to recalculate its root hash.

pub mod acknowledgements;
pub mod channels;
pub mod client_states;
pub mod clients;
pub mod connections;
pub mod consensus_states;
pub mod next_seq_ack;
pub mod next_seq_recv;
pub mod next_seq_send;
pub mod packet_commitments;
pub mod receipts;
