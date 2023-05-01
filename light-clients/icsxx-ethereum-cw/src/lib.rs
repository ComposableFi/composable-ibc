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

pub const STORAGE_PREFIX: &'static [u8] = b"";

pub type Bytes = Vec<u8>;
