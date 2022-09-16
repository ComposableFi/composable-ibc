//! Implementations of client verification algorithms for specific types of chains.
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
#[macro_use]
extern crate derive;
#[allow(unused_imports)]
#[macro_use]
extern crate serde;

pub mod any;
pub mod ics07_tendermint;
#[cfg(any(test, feature = "mocks", feature = "ics11_beefy"))]
pub mod ics11_beefy;
#[cfg(any(test, feature = "mocks", feature = "ics11_beefy"))]
pub mod ics13_near;

#[cfg(not(any(test, feature = "mocks", feature = "ics11_beefy")))]
use core::fmt::Debug;

#[cfg(any(test, feature = "mocks", feature = "ics11_beefy"))]
use ics11_beefy::client_def::HostFunctions as BeefyHostFunctions;

#[cfg(not(any(test, feature = "mocks", feature = "ics11_beefy")))]
pub trait BeefyHostFunctions {}

#[cfg(any(test, feature = "mocks", feature = "ics11_beefy"))]
use ics13_near::client_def::HostFunctionsTrait as NearHostFunctionsTrait;

#[cfg(not(any(test, feature = "mocks", feature = "ics11_beefy")))]
pub trait NearHostFunctionsTrait: Clone + Debug + PartialEq + Eq + Default + Send + Sync {}

pub trait AnyHostFunctionsTrait:
    beefy_client_primitives::HostFunctions
    + BeefyHostFunctions
    + NearHostFunctionsTrait
    + tendermint_light_client_verifier::host_functions::HostFunctionsProvider
    + ics23::HostFunctionsProvider
    + 'static
{
}
