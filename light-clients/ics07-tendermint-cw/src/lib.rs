extern crate alloc;
extern crate core;

mod channel;
mod client;
mod connection;
mod context;
pub mod contract;
mod error;
pub mod helpers;
pub mod ics23;
mod macros;
pub mod msg;
pub mod state;
mod types;

pub use crate::error::ContractError;

pub const CLIENT_STATE: &'static [u8] = b"client_state";
pub const STORAGE_PREFIX: &'static [u8] = b"";

pub type Bytes = Vec<u8>;
