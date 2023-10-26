use alloc::collections::BTreeMap;
use anchor_client::anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

pub type InnerHeight = (u64, u64);
pub type HostHeight = InnerHeight;
pub type SolanaTimestamp = u64;
pub type InnerClientId = String;
pub type InnerConnectionId = String;
pub type InnerPortId = String;
pub type InnerChannelId = String;
pub type InnerSequence = u64;
pub type InnerIbcEvent = Vec<u8>;
pub type InnerClient = String; // Serialized
pub type InnerConnectionEnd = String; // Serialized
pub type InnerChannelEnd = String; // Serialized
pub type InnerConsensusState = String; // Serialized

/// A triple of send, receive and acknowledge sequences.
#[derive(Clone, Debug, Default, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InnerSequenceTriple {
	sequences: [u64; 3],
	mask: u8,
}

#[derive(Clone, Copy)]
pub enum SequenceTripleIdx {
	Send = 0,
	Recv = 1,
	Ack = 2,
}

#[derive(Debug, AnchorSerialize, AnchorDeserialize)]
/// All the structs from IBC are stored as String since they dont implement AnchorSerialize and
/// AnchorDeserialize
pub struct PrivateStorage {
	pub height: InnerHeight,
	pub clients: BTreeMap<InnerClientId, InnerClient>,
	/// The client ids of the clients.
	pub client_id_set: Vec<InnerClientId>,
	pub client_counter: u64,
	pub client_processed_times: BTreeMap<InnerClientId, BTreeMap<InnerHeight, SolanaTimestamp>>,
	pub client_processed_heights: BTreeMap<InnerClientId, BTreeMap<InnerHeight, HostHeight>>,
	pub consensus_states: BTreeMap<(InnerClientId, InnerHeight), InnerConsensusState>,
	/// This collection contains the heights corresponding to all consensus states of
	/// all clients stored in the contract.
	pub client_consensus_state_height_sets: BTreeMap<InnerClientId, Vec<InnerHeight>>,
	/// The connection ids of the connections.
	pub connection_id_set: Vec<InnerConnectionId>,
	pub connection_counter: u64,
	pub connections: BTreeMap<InnerConnectionId, InnerConnectionEnd>,
	pub channel_ends: BTreeMap<(InnerPortId, InnerChannelId), InnerChannelEnd>,
	// Contains the client id corresponding to the connectionId
	pub connection_to_client: BTreeMap<InnerConnectionId, InnerClientId>,
	/// The port and channel id tuples of the channels.
	pub port_channel_id_set: Vec<(InnerPortId, InnerChannelId)>,
	pub channel_counter: u64,

	/// Next send, receive and ack sequence for given (port, channel).
	///
	/// We’re storing all three sequences in a single object to reduce amount of
	/// different maps we need to maintain.  This saves us on the amount of
	/// trie nodes we need to maintain.
	pub next_sequence: BTreeMap<(InnerPortId, InnerChannelId), InnerSequenceTriple>,

	/// The sequence numbers of the packet commitments.
	pub packet_commitment_sequence_sets:
		BTreeMap<(InnerPortId, InnerChannelId), Vec<InnerSequence>>,
	/// The sequence numbers of the packet receipts.
	pub packet_receipt_sequence_sets: BTreeMap<(InnerPortId, InnerChannelId), Vec<InnerSequence>>,
	/// The sequence numbers of the packet acknowledgements.
	pub packet_acknowledgement_sequence_sets:
		BTreeMap<(InnerPortId, InnerChannelId), Vec<InnerSequence>>,
	/// The history of IBC events.
	pub ibc_events_history: BTreeMap<InnerHeight, Vec<InnerIbcEvent>>,
}
