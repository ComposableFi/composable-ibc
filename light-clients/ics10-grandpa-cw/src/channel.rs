use crate::{
	context::Context,
	contract::{
		CHANNELS_CONNECTION, CHANNEL_COUNTER, CLIENT_UPDATE_HEIGHT, CLIENT_UPDATE_TIME,
		EXPECTED_BLOCK_TIME,
	},
	ics23::{
		ReadonlyAcknowledgements, ReadonlyChannels, ReadonlyNextSequenceAck,
		ReadonlyNextSequenceRecv, ReadonlyNextSequenceSend, ReadonlyPacketCommitments,
		ReadonlyPacketReceipts,
	},
	log,
};
use grandpa_light_client_primitives::HostFunctions;
use ibc::{
	core::{
		ics04_channel::{
			channel::ChannelEnd,
			commitment::{AcknowledgementCommitment, PacketCommitment as PacketCommitmentType},
			context::ChannelReader,
			error::Error,
			packet::{Receipt, Sequence},
		},
		ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
	},
	protobuf::Protobuf,
	timestamp::Timestamp,
	Height,
};
use sha2::{Digest, Sha256};
use std::{str::FromStr, time::Duration};

impl<'a, H: HostFunctions> ChannelReader for Context<'a, H> {
	fn channel_end(&self, port_channel_id: &(PortId, ChannelId)) -> Result<ChannelEnd, Error> {
		log!(
			self,
			"in channel : [channel_end] >> port_id = {:?}, channel_id = {:?}",
			port_channel_id.0,
			port_channel_id.1
		);
		let data = ReadonlyChannels::new(self.deps.storage)
			.get(port_channel_id.clone())
			.ok_or_else(Error::missing_channel)?;
		let channel_end = ChannelEnd::decode_vec(&*data).map_err(|_| {
			Error::channel_not_found(port_channel_id.clone().0, port_channel_id.clone().1)
		})?;
		log!(self, "in channel : [channel_end] >> channel_end = {:?}", channel_end);
		Ok(channel_end)
	}

	fn connection_channels(
		&self,
		conn_id: &ConnectionId,
	) -> Result<Vec<(PortId, ChannelId)>, Error> {
		let conn_id_bytes = conn_id.as_bytes().to_vec();
		let port_and_channel_id =
			CHANNELS_CONNECTION.load(self.storage(), conn_id_bytes).unwrap_or_default();
		if port_and_channel_id.is_empty() {
			return Err(Error::connection_not_open(conn_id.clone()))
		}

		let mut result = vec![];
		for item in port_and_channel_id {
			let port_id = String::from_utf8(item.0).map_err(|e| {
				Error::implementation_specific(format!(
					"[connection_channels]: error decoding port_id: {}",
					e
				))
			})?;
			let port_id = PortId::from_str(port_id.as_str()).map_err(|e| {
				Error::implementation_specific(format!(
					"[connection_channels]: invalid port id string: {}",
					e
				))
			})?;

			let channel_id = String::from_utf8(item.1).map_err(|e| {
				Error::implementation_specific(format!(
					"[connection_channels]: error decoding channel_id: {}",
					e
				))
			})?;
			let channel_id = ChannelId::from_str(channel_id.as_str()).map_err(|e| {
				Error::implementation_specific(format!(
					"[connection_channels]: error decoding channel_id: {}",
					e
				))
			})?;

			result.push((port_id, channel_id));
		}

		log!(
			self,
			"in channel : [connection_channels] >> Vector<(PortId, ChannelId)> =  {:?}",
			result
		);
		Ok(result)
	}

	fn get_next_sequence_send(
		&self,
		port_channel_id: &(PortId, ChannelId),
	) -> Result<Sequence, Error> {
		let seq = ReadonlyNextSequenceSend::new(self.storage())
			.get(port_channel_id.clone())
			.ok_or_else(|| Error::missing_next_send_seq(port_channel_id.clone()))?;
		log!(self, "in channel : [get_next_sequence] >> sequence  = {:?}", seq);
		Ok(Sequence::from(seq))
	}

	fn get_next_sequence_recv(
		&self,
		port_channel_id: &(PortId, ChannelId),
	) -> Result<Sequence, Error> {
		let seq = ReadonlyNextSequenceRecv::new(self.storage())
			.get(port_channel_id.clone())
			.ok_or_else(|| Error::missing_next_recv_seq(port_channel_id.clone()))?;

		log!(self, "in channel : [get_next_sequence_recv] >> sequence = {:?}", seq);
		Ok(Sequence::from(seq))
	}

	fn get_next_sequence_ack(
		&self,
		port_channel_id: &(PortId, ChannelId),
	) -> Result<Sequence, Error> {
		let seq = ReadonlyNextSequenceAck::new(self.storage())
			.get(port_channel_id.clone())
			.ok_or_else(|| Error::missing_next_ack_seq(port_channel_id.clone()))?;
		log!(self, "in channel : [get_next_sequence_ack] >> sequence = {:?}", seq);
		Ok(Sequence::from(seq))
	}

	fn get_packet_commitment(
		&self,
		key: &(PortId, ChannelId, Sequence),
	) -> Result<PacketCommitmentType, Error> {
		let packet_commitments = ReadonlyPacketCommitments::new(self.storage());
		if !packet_commitments.contains_key(key.clone()) {
			log!(
				self,
				"in channel : [get_packet_commitment] >> read get packet commitment return None"
			);
			return Err(Error::packet_commitment_not_found(key.2))
		}

		let data = packet_commitments.get(key.clone()).ok_or_else(Error::missing_packet)?;
		log!(self, "in channel : [get_packet_commitment] >> packet_commitment = {:?}", data);
		Ok(data.into())
	}

	fn get_packet_receipt(&self, key: &(PortId, ChannelId, Sequence)) -> Result<Receipt, Error> {
		let seq = u64::from(key.2);

		let packet_receipts = ReadonlyPacketReceipts::new(self.storage());
		if !packet_receipts.contains_key((key.0.clone(), key.1, key.2)) {
			log!(self, "in channel : [get_packet_receipt] >> read get packet receipt not found");
			return Err(Error::packet_receipt_not_found(key.2))
		}

		let data = packet_receipts
			.get((key.0.clone(), key.1, key.2))
			.ok_or_else(|| Error::packet_receipt_not_found(key.2))?;
		let data = String::from_utf8(data).map_err(|e| {
			Error::implementation_specific(format!(
				"[get_packet_receipt]: error decoding packet receipt: {}",
				e
			))
		})?;
		let data = match data.as_ref() {
			"Ok" => Receipt::Ok,
			_ => return Err(Error::packet_receipt_not_found(seq.into())),
		};
		log!(self, "in channel : [get_packet_receipt] >> packet_receipt = {:?}", data);
		Ok(data)
	}

	fn get_packet_acknowledgement(
		&self,
		key: &(PortId, ChannelId, Sequence),
	) -> Result<AcknowledgementCommitment, Error> {
		let acknowledgements = ReadonlyAcknowledgements::new(self.storage());
		if !acknowledgements.contains_key((key.0.clone(), key.1, key.2)) {
			log!(
				self,
				"in channel : [get_packet_acknowledgement] >> get acknowledgement not found"
			);
			return Err(Error::packet_acknowledgement_not_found(key.2))
		}

		let ack = acknowledgements
			.get(key.clone())
			.ok_or_else(|| Error::packet_acknowledgement_not_found(key.2))?;
		log!(
			self,
			"in channel : [get_packet_acknowledgement] >> packet_acknowledgement = {:?}",
			ack
		);
		Ok(ack.into())
	}

	/// A hashing function for packet commitments
	fn hash(&self, value: Vec<u8>) -> Vec<u8> {
		let mut hasher = Sha256::default();
		hasher.update(value);
		hasher.finalize().to_vec()
	}

	fn client_update_time(&self, client_id: &ClientId, height: Height) -> Result<Timestamp, Error> {
		let height_data = height.encode_vec();
		let client_id_data = client_id.as_bytes().to_vec();
		let timestamp = CLIENT_UPDATE_TIME
			.load(self.storage(), (client_id_data, height_data))
			.map_err(|_| Error::processed_time_not_found(client_id.clone(), height))?;

		Timestamp::from_nanoseconds(timestamp).map_err(|e| {
			Error::implementation_specific(format!(
				"[client_update_time]:  error decoding timestamp from nano seconds: {}",
				e
			))
		})
	}

	fn client_update_height(&self, client_id: &ClientId, height: Height) -> Result<Height, Error> {
		let height_data = height.encode_vec();
		let client_id_data = client_id.as_bytes().to_vec();
		let host_height = CLIENT_UPDATE_HEIGHT
			.load(self.storage(), (client_id_data, height_data))
			.map_err(|_| Error::processed_height_not_found(client_id.clone(), height))?;

		Height::decode_vec(&host_height).map_err(|e| {
			Error::implementation_specific(format!(
				"[client_update_height]: error decoding height: {}",
				e
			))
		})
	}

	/// Returns a counter on the number of channel ids have been created thus far.
	/// The value of this counter should increase only via method
	/// `ChannelKeeper::increase_channel_counter`.
	fn channel_counter(&self) -> Result<u64, Error> {
		Ok(CHANNEL_COUNTER.load(self.storage()).unwrap_or(0).into())
	}

	fn max_expected_time_per_block(&self) -> Duration {
		Duration::from_nanos(EXPECTED_BLOCK_TIME.load(self.storage()).unwrap_or(0))
	}
}
