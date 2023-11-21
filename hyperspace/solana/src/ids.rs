use anchor_lang::prelude::borsh;
use ibc::core::{
	ics02_client::error::Error as ClientError, ics03_connection::error::Error as ConnectionError,
	ics24_host::identifier::ConnectionId,
};

type Result<T, E = ClientError> = core::result::Result<T, E>;

/// Prefix of IBC channel ids.
///
/// Note: We’re not using ChannelId::prefix() because it returns the prefix
/// without trailing `-` which we want included to simplify stripping of the
/// prefix.
pub(super) const CHANNEL_ID_PREFIX: &str = "channel-";

/// An index used as unique identifier for a client.
///
/// IBC client id uses `<client-type>-<counter>` format.  This index is
/// constructed from a client id by stripping the client type.  Since counter is
/// unique within an IBC module, the index is enough to identify a known client.
///
/// To avoid confusing identifiers with the same counter but different client
/// type (which may be crafted by an attacker), we always check that client type
/// matches one we know.  Because of this check, to get `ClientIdx`
/// [`PrivateStorage::client`] needs to be used.
///
/// The index is guaranteed to fit `u32` and `usize`.
#[derive(Clone, Copy, PartialEq, Eq, derive_more::From, derive_more::Into)]
pub struct ClientIdx(u32);

impl From<ClientIdx> for usize {
	#[inline]
	fn from(index: ClientIdx) -> usize {
		index.0 as usize
	}
}

impl core::str::FromStr for ClientIdx {
	type Err = core::num::ParseIntError;

	#[inline]
	fn from_str(value: &str) -> Result<Self, Self::Err> {
		if core::mem::size_of::<usize>() < 4 {
			usize::from_str(value).map(|index| Self(index as u32))
		} else {
			u32::from_str(value).map(Self)
		}
	}
}

impl PartialEq<usize> for ClientIdx {
	#[inline]
	fn eq(&self, rhs: &usize) -> bool {
		u32::try_from(*rhs).ok().filter(|rhs| self.0 == *rhs).is_some()
	}
}

/// An internal connection identifier.
///
/// The identifier is build from IBC identifiers which are of the form
/// `connection-<number>`.  Rather than treating the identifier as a string,
/// we’re parsing the number out and keep only that.
#[derive(
	Clone,
	Copy,
	Debug,
	PartialEq,
	Eq,
	borsh::BorshSerialize,
	borsh::BorshDeserialize,
	derive_more::From,
	derive_more::Into,
)]
pub struct ConnectionIdx(u32);

impl ConnectionIdx {
	/// Prefix of IBC connection ids.
	///
	/// Note: We’re not using ConnectionId::prefix() because it returns the
	/// prefix without trailing `-` which we want included to simplify stripping
	/// of the prefix.
	const IBC_PREFIX: &'static str = "connection-";
}

impl From<ConnectionIdx> for usize {
	#[inline]
	fn from(index: ConnectionIdx) -> usize {
		index.0 as usize
	}
}

impl TryFrom<ConnectionId> for ConnectionIdx {
	type Error = ConnectionError;

	fn try_from(id: ConnectionId) -> Result<Self, Self::Error> {
		match parse_sans_prefix(Self::IBC_PREFIX, id.as_str()) {
			Some(num) => Ok(Self(num)),
			None => Err(Self::Error::connection_not_found(id)),
		}
	}
}

impl TryFrom<&ConnectionId> for ConnectionIdx {
    type Error = ConnectionError;

    fn try_from(id: &ConnectionId) -> Result<Self, Self::Error> {
        match parse_sans_prefix(Self::IBC_PREFIX, id.as_str()) {
            Some(num) => Ok(Self(num)),
            None => Err(Self::Error::connection_not_found(id.clone())),
        }
    }
}

/// Strips `prefix` from `data` and parses it to get `u32`.  Panics if data
/// doesn’t start with the prefix or parsing fails.
fn parse_sans_prefix(prefix: &'static str, data: &str) -> Option<u32> {
	data.strip_prefix(prefix)
		.and_then(|index| index.parse().ok())
		.filter(|index| usize::try_from(*index).is_ok())
}
