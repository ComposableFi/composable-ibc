use crate::context::Context;
use grandpa_light_client_primitives::HostFunctions;
use ibc::{
	core::{
		ics04_channel::{
			channel::ChannelEnd,
			commitment::{AcknowledgementCommitment, PacketCommitment},
			context::{ChannelKeeper, ChannelReader},
			error::Error,
			packet::{Packet, Receipt, Sequence},
		},
		ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
	},
	timestamp::Timestamp,
	Height,
};
use ics10_grandpa::client_message::RelayChainHeader;
use std::time::Duration;

impl<'a, H: HostFunctions<Header = RelayChainHeader>> ChannelReader for Context<'a, H> {
	fn channel_end(&self, _port_channel_id: &(PortId, ChannelId)) -> Result<ChannelEnd, Error> {
		todo!()
	}

	fn connection_channels(&self, _cid: &ConnectionId) -> Result<Vec<(PortId, ChannelId)>, Error> {
		todo!()
	}

	fn get_next_sequence_send(
		&self,
		_port_channel_id: &(PortId, ChannelId),
	) -> Result<Sequence, Error> {
		todo!()
	}

	fn get_next_sequence_recv(
		&self,
		_port_channel_id: &(PortId, ChannelId),
	) -> Result<Sequence, Error> {
		todo!()
	}

	fn get_next_sequence_ack(
		&self,
		_port_channel_id: &(PortId, ChannelId),
	) -> Result<Sequence, Error> {
		todo!()
	}

	fn get_packet_commitment(
		&self,
		_key: &(PortId, ChannelId, Sequence),
	) -> Result<PacketCommitment, Error> {
		todo!()
	}

	fn get_packet_receipt(&self, _key: &(PortId, ChannelId, Sequence)) -> Result<Receipt, Error> {
		todo!()
	}

	fn get_packet_acknowledgement(
		&self,
		_key: &(PortId, ChannelId, Sequence),
	) -> Result<AcknowledgementCommitment, Error> {
		todo!()
	}

	fn hash(&self, _value: Vec<u8>) -> Vec<u8> {
		todo!()
	}

	fn client_update_time(&self, _client_id: &ClientId, _height: Height) -> Result<Timestamp, Error> {
		todo!()
	}

	fn client_update_height(&self, _client_id: &ClientId, _height: Height) -> Result<Height, Error> {
		todo!()
	}

	fn channel_counter(&self) -> Result<u64, Error> {
		todo!()
	}

	fn max_expected_time_per_block(&self) -> Duration {
		todo!()
	}
}

impl<'a, H: HostFunctions<Header = RelayChainHeader>> ChannelKeeper for Context<'a, H> {
	fn store_packet_commitment(
		&mut self,
		_key: (PortId, ChannelId, Sequence),
		_commitment: PacketCommitment,
	) -> Result<(), Error> {
		todo!()
	}

	fn store_send_packet(
		&mut self,
		_key: (PortId, ChannelId, Sequence),
		_packet: Packet,
	) -> Result<(), Error> {
		todo!()
	}

	fn store_recv_packet(
		&mut self,
		_key: (PortId, ChannelId, Sequence),
		_packet: Packet,
	) -> Result<(), Error> {
		todo!()
	}

	fn delete_packet_commitment(
		&mut self,
		_key: (PortId, ChannelId, Sequence),
	) -> Result<(), Error> {
		todo!()
	}

	fn store_packet_receipt(
		&mut self,
		_key: (PortId, ChannelId, Sequence),
		_receipt: Receipt,
	) -> Result<(), Error> {
		todo!()
	}

	fn store_packet_acknowledgement(
		&mut self,
		_key: (PortId, ChannelId, Sequence),
		_ack_commitment: AcknowledgementCommitment,
	) -> Result<(), Error> {
		todo!()
	}

	fn delete_packet_acknowledgement(
		&mut self,
		_key: (PortId, ChannelId, Sequence),
	) -> Result<(), Error> {
		todo!()
	}

	fn store_connection_channels(
		&mut self,
		_conn_id: ConnectionId,
		_port_channel_id: &(PortId, ChannelId),
	) -> Result<(), Error> {
		todo!()
	}

	fn store_channel(
		&mut self,
		_port_channel_id: (PortId, ChannelId),
		_channel_end: &ChannelEnd,
	) -> Result<(), Error> {
		todo!()
	}

	fn store_next_sequence_send(
		&mut self,
		_port_channel_id: (PortId, ChannelId),
		_seq: Sequence,
	) -> Result<(), Error> {
		todo!()
	}

	fn store_next_sequence_recv(
		&mut self,
		_port_channel_id: (PortId, ChannelId),
		_seq: Sequence,
	) -> Result<(), Error> {
		todo!()
	}

	fn store_next_sequence_ack(
		&mut self,
		_port_channel_id: (PortId, ChannelId),
		_seq: Sequence,
	) -> Result<(), Error> {
		todo!()
	}

	fn increase_channel_counter(&mut self) {
		todo!()
	}
}
