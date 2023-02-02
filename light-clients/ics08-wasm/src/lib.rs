#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

use alloc::{collections::BTreeMap, vec::Vec};
use ibc::core::ics02_client::{client_state::ClientType, events::CodeId};
use spin::RwLock;

pub mod client_def;
pub mod client_message;
pub mod client_state;
pub mod consensus_state;
pub mod msg;

pub type Bytes = Vec<u8>;
