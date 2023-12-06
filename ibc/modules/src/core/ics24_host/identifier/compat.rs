//! Compatibility conversion function between our IBC fork and upstream IBC.
//!
//! Upcoming Solana bridging uses newest ibc-rs crate.  This module provides
//! conversion between identifiers of our fork and the upstream crate used by
//! Solana.

use alloc::string::String;
use core::str::FromStr;

use super::{ChainId, ChannelId, ClientId, ConnectionId, PortId, ValidationError};

mod new {
	pub use ibc_core_host_types_new::{
		error::IdentifierError,
		identifiers::{ChainId, ChannelId, ClientId, ConnectionId, PortId},
	};
}

// Helper macros

macro_rules! impl_eq {
	($ty:ident) => {
		impl PartialEq<$ty> for new::$ty {
			fn eq(&self, rhs: &$ty) -> bool {
				self.as_str() == rhs.as_str()
			}
		}

		impl PartialEq<new::$ty> for $ty {
			fn eq(&self, rhs: &new::$ty) -> bool {
				self.as_str() == rhs.as_str()
			}
		}
	};
}

macro_rules! impl_test {
	($test:ident $ty:ident, $id:literal) => {
		#[test]
		fn $test() {
			let old = $ty::from_str($id).unwrap();
			let new = new::$ty::from_str($id).unwrap();
			assert_eq!(old, new);
			assert_eq!(old, $ty::try_from(&new).unwrap());
			assert_eq!(new, new::$ty::try_from(&old).unwrap());
		}
	};
}

// ChainId

impl From<new::ChainId> for ChainId {
	fn from(id: new::ChainId) -> Self {
		let version = id.revision_number();
		let id = String::from(id);
		Self { id, version }
	}
}

impl From<&new::ChainId> for ChainId {
	fn from(id: &new::ChainId) -> Self {
		let version = id.revision_number();
		let id = String::from(id.as_str());
		Self { id, version }
	}
}

impl TryFrom<ChainId> for new::ChainId {
	type Error = ValidationError;

	fn try_from(id: ChainId) -> Result<Self, Self::Error> {
		Self::try_from(&id)
	}
}

impl TryFrom<&ChainId> for new::ChainId {
	type Error = ValidationError;

	fn try_from(id: &ChainId) -> Result<Self, Self::Error> {
		let id = id.as_str();
		new::ChainId::new(id).map_err(|_| ValidationError::chain_id_invalid_format(id.into()))
	}
}

impl_eq!(ChainId);
impl_test!(test_chain_id ChainId, "foo-bar-42");

// ClientId

impl From<new::ClientId> for ClientId {
	fn from(id: new::ClientId) -> Self {
		Self(String::from(id))
	}
}

impl From<&new::ClientId> for ClientId {
	fn from(id: &new::ClientId) -> Self {
		Self(String::from(id.as_str()))
	}
}

impl TryFrom<ClientId> for new::ClientId {
	type Error = new::IdentifierError;

	fn try_from(id: ClientId) -> Result<Self, Self::Error> {
		Self::try_from(&id)
	}
}

impl TryFrom<&ClientId> for new::ClientId {
	type Error = new::IdentifierError;

	fn try_from(id: &ClientId) -> Result<Self, Self::Error> {
		new::ClientId::from_str(id.as_str())
	}
}

impl_eq!(ClientId);
impl_test!(test_client_id ClientId, "foo-bar-42");

// ConnectionId

impl From<new::ConnectionId> for ConnectionId {
	fn from(id: new::ConnectionId) -> Self {
		Self(String::from(id))
	}
}

impl From<&new::ConnectionId> for ConnectionId {
	fn from(id: &new::ConnectionId) -> Self {
		Self(String::from(id.as_str()))
	}
}

impl TryFrom<ConnectionId> for new::ConnectionId {
	type Error = new::IdentifierError;

	fn try_from(id: ConnectionId) -> Result<Self, Self::Error> {
		Self::try_from(&id)
	}
}

impl TryFrom<&ConnectionId> for new::ConnectionId {
	type Error = new::IdentifierError;

	fn try_from(id: &ConnectionId) -> Result<Self, Self::Error> {
		new::ConnectionId::from_str(id.as_str())
	}
}

impl_eq!(ConnectionId);
impl_test!(test_connection_id ConnectionId, "connection-42");

// PortId

impl From<new::PortId> for PortId {
	fn from(id: new::PortId) -> Self {
		Self(String::from(id))
	}
}

impl From<&new::PortId> for PortId {
	fn from(id: &new::PortId) -> Self {
		Self(String::from(id.as_str()))
	}
}

impl TryFrom<PortId> for new::PortId {
	type Error = new::IdentifierError;

	fn try_from(id: PortId) -> Result<Self, Self::Error> {
		Self::try_from(&id)
	}
}

impl TryFrom<&PortId> for new::PortId {
	type Error = new::IdentifierError;

	fn try_from(id: &PortId) -> Result<Self, Self::Error> {
		new::PortId::from_str(id.as_str())
	}
}

impl_eq!(PortId);
impl_test!(test_port_id PortId, "transfer");

// ChannelId

impl TryFrom<new::ChannelId> for ChannelId {
	type Error = ValidationError;

	fn try_from(id: new::ChannelId) -> Result<Self, Self::Error> {
		Self::from_str(id.as_str())
	}
}

impl TryFrom<&new::ChannelId> for ChannelId {
	type Error = ValidationError;

	fn try_from(id: &new::ChannelId) -> Result<Self, Self::Error> {
		Self::from_str(id.as_str())
	}
}

impl From<ChannelId> for new::ChannelId {
	fn from(id: ChannelId) -> Self {
		Self::new(id.sequence())
	}
}

impl From<&ChannelId> for new::ChannelId {
	fn from(id: &ChannelId) -> Self {
		Self::new(id.sequence())
	}
}

impl PartialEq<new::ChannelId> for ChannelId {
	fn eq(&self, rhs: &new::ChannelId) -> bool {
		Self::from_id(rhs.as_str()).filter(|rhs| self == rhs).is_some()
	}
}

impl PartialEq<ChannelId> for new::ChannelId {
	fn eq(&self, rhs: &ChannelId) -> bool {
		rhs.eq(self)
	}
}

impl_test!(test_channel_id ChannelId, "channel-42");
