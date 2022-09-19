#![cfg_attr(not(feature = "std"), no_std)]

//! ICS-11: Beefy IBC light client protocol implementation

extern crate alloc;

pub mod client_def;
pub mod client_state;
pub mod consensus_state;
pub mod error;
pub mod header;
pub mod misbehaviour;
mod proto;

#[cfg(any(test, feature = "mocks"))]
mod mock;
#[cfg(any(test, feature = "mocks"))]
mod tests;
