extern crate alloc;

mod channel;
mod client;
mod connection;
mod context;
pub mod contract;
mod error;
pub mod ics23;
mod macros;
pub mod msg;
pub mod state;

pub use crate::error::ContractError;

pub const CLIENT_STATE: &[u8] = b"client_state";
pub const STORAGE_PREFIX: &[u8] = b"";

pub type Bytes = Vec<u8>;
