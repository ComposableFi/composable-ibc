use alloc::collections::BTreeMap;
use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};
use ibc_new::{
	core::{
		ics02_client::error::ClientError,
		ics04_channel::{msgs::PacketMsg, packet::Sequence, channel::ChannelEnd},
		ics24_host::identifier::ClientId, ics03_connection::connection::ConnectionEnd,
	},
	Height,
};
use ibc_new::clients::ics07_tendermint::consensus_state::ConsensusState as TmConsensusState;
use ibc_new::clients::ics07_tendermint::client_state::ClientState as TmClientState;
use borsh::maybestd::io;
use lib::hash::CryptoHash;
use crate::ids;

type Result<T, E = anchor_lang::error::Error> = core::result::Result<T, E>;

pub type SolanaTimestamp = u64;
pub type InnerPortId = String;
pub type InnerChannelId = String;

#[derive(Clone, Debug, PartialEq, derive_more::From, derive_more::TryInto)]
pub enum AnyClientState {
    Tendermint(TmClientState),
}

#[derive(Clone, Debug, PartialEq, derive_more::From, derive_more::TryInto)]
pub enum AnyConsensusState {
    Tendermint(TmConsensusState),
}

/// A triple of send, receive and acknowledge sequences.
#[derive(Clone, Debug, Default, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SequenceTriple {
	sequences: [u64; 3],
	mask: u8,
}

#[derive(Clone, Copy)]
pub enum SequenceTripleIdx {
	Send = 0,
	Recv = 1,
	Ack = 2,
}

impl SequenceTriple {
	/// Returns sequence at given index or `None` if it wasn’t set yet.
	pub fn get(&self, idx: SequenceTripleIdx) -> Option<Sequence> {
		if self.mask & (1 << (idx as u32)) == 1 {
			Some(Sequence::from(self.sequences[idx as usize]))
		} else {
			None
		}
	}

	/// Sets sequence at given index.
	pub fn set(&mut self, idx: SequenceTripleIdx, seq: Sequence) {
		self.sequences[idx as usize] = u64::from(seq);
		self.mask |= 1 << (idx as u32)
	}

	/// Encodes the object as a `CryptoHash` so it can be stored in the trie
	/// directly.
	pub fn to_hash(&self) -> lib::hash::CryptoHash {
		let mut hash = lib::hash::CryptoHash::default();
		let (first, tail) = stdx::split_array_mut::<8, 24, 32>(&mut hash.0);
		let (second, tail) = stdx::split_array_mut::<8, 16, 24>(tail);
		let (third, tail) = stdx::split_array_mut::<8, 8, 16>(tail);
		*first = self.sequences[0].to_be_bytes();
		*second = self.sequences[1].to_be_bytes();
		*third = self.sequences[2].to_be_bytes();
		tail[0] = self.mask;
		hash
	}
}

#[derive(Clone, Debug, borsh::BorshSerialize, borsh::BorshDeserialize)]
pub struct ClientStore {
    pub client_id: ClientId,
    pub connection_id: Option<ids::ConnectionIdx>,

    pub client_state: Serialised<AnyClientState>,
    pub consensus_states: BTreeMap<Height, Serialised<AnyConsensusState>>,
    pub processed_times: BTreeMap<Height, SolanaTimestamp>,
    pub processed_heights: BTreeMap<Height, Height>,
}

#[account]
#[derive(Debug)]
pub struct IbcPackets(pub Vec<ibc_new::core::ics04_channel::msgs::PacketMsg>);

#[account]
#[derive(Debug)]
/// The private IBC storage, i.e. data which doesn’t require proofs.
pub struct PrivateStorage {
    /// Per-client information.
    ///
    /// Entry at index `N` corresponds to the client with IBC identifier
    /// `client-<N>`.
    pub clients: Vec<ClientStore>,

    /// Information about the counterparty on given connection.
    ///
    /// Entry at index `N` corresponds to the connection with IBC identifier
    /// `connection-<N>`.
    pub connections: Vec<Serialised<ConnectionEnd>>,

    pub channel_ends:
        BTreeMap<(InnerPortId, InnerChannelId), Serialised<ChannelEnd>>,
    pub channel_counter: u64,

    /// The sequence numbers of the packet commitments.
    pub packet_commitment_sequence_sets:
        BTreeMap<(InnerPortId, InnerChannelId), Vec<Sequence>>,
    /// The sequence numbers of the packet acknowledgements.
    pub packet_acknowledgement_sequence_sets:
        BTreeMap<(InnerPortId, InnerChannelId), Vec<Sequence>>,

    /// Next send, receive and ack sequence for given (port, channel).
    ///
    /// We’re storing all three sequences in a single object to reduce amount of
    /// different maps we need to maintain.  This saves us on the amount of
    /// trie nodes we need to maintain.
    pub next_sequence: BTreeMap<(InnerPortId, InnerChannelId), SequenceTriple>,
}

#[derive(Clone, Default, Debug)]
pub struct Serialised<T>(Vec<u8>, core::marker::PhantomData<T>);

impl<T> Serialised<T> {
    pub fn empty() -> Self { Self(Vec::new(), core::marker::PhantomData) }

    pub fn as_bytes(&self) -> &[u8] { self.0.as_slice() }

    pub fn digest(&self) -> CryptoHash { CryptoHash::digest(self.0.as_slice()) }

    fn make_err(err: io::Error) -> ClientError {
        ClientError::ClientSpecific { description: err.to_string() }
    }
}

impl<T: borsh::BorshSerialize> Serialised<T> {
    pub fn new(value: &T) -> Result<Self, ClientError> {
        borsh::to_vec(value)
            .map(|data| Self(data, core::marker::PhantomData))
            .map_err(Self::make_err)
    }

    pub fn set(&mut self, value: &T) -> Result<&mut Self, ClientError> {
        *self = Self::new(value)?;
        Ok(self)
    }
}

impl<T: borsh::BorshDeserialize> Serialised<T> {
    pub fn get(&self) -> Result<T, ClientError> {
        T::try_from_slice(self.0.as_slice()).map_err(Self::make_err)
    }
}

impl<T> borsh::BorshSerialize for Serialised<T> {
    fn serialize<W: io::Write>(&self, wr: &mut W) -> io::Result<()> {
        u16::try_from(self.0.len())
            .map_err(|_| io::ErrorKind::InvalidData.into())
            .and_then(|len| len.serialize(wr))?;
        wr.write_all(self.0.as_slice())
    }
}

impl<T> borsh::BorshDeserialize for Serialised<T> {
    fn deserialize_reader<R: io::Read>(rd: &mut R) -> io::Result<Self> {
        let len = u16::deserialize_reader(rd)?.into();
        let mut data = vec![0; len];
        rd.read_exact(data.as_mut_slice())?;
        Ok(Self(data, core::marker::PhantomData))
    }
}
