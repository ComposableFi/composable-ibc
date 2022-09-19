#![cfg_attr(not(feature = "std"), no_std)]

//! ICS-11: Grandpa IBC light client protocol implementation

extern crate alloc;

mod client_def;
mod client_state;
mod error;
mod proto;
