use crate::error::Error;
use core::convert::TryFrom;
use ibc::events::{Error as IbcEventError, IbcEvent, IbcEventType};
use std::collections::BTreeMap as HashMap;
use tendermint_rpc::abci::Event as AbciEvent;
use tendermint_rpc::{event::Event as RpcEvent, event::EventData as RpcEventData};

use ibc::core::ics02_client::{
	events::{self as ClientEvents, Attributes as ClientAttributes},
	header::Header,
};
use ibc::core::ics03_connection::events::{
	self as ConnectionEvents, Attributes as ConnectionAttributes,
};
use ibc::core::ics04_channel::{
	events::{self as ChannelEvents, Attributes as ChannelAttributes},
	packet::Packet,
};

fn extract_block_events(block_events: &HashMap<String, Vec<String>>) -> Vec<IbcEvent> {
	#[inline]
	fn extract_events<'a, T: TryFrom<RawObject<'a>>>(
		block_events: &'a HashMap<String, Vec<String>>,
		event_type: &str,
		event_field: &str,
	) -> Vec<T> {
		block_events
			.get(&format!("{}.{}", event_type, event_field))
			.unwrap_or(&vec![])
			.iter()
			.filter_map(|_| block_events)
			.collect()
	}

	#[inline]
	fn append_events<T: Into<IbcEvent>>(events: &mut Vec<IbcEvent>, chan_events: Vec<T>) {
		events.append(&mut chan_events.into_iter().map(|ev| ev.into()).collect());
	}

	let mut events: Vec<IbcEvent> = vec![];
	append_events::<ChannelEvents::OpenInit>(
		&mut events,
		extract_events(block_events, "channel_open_init", "channel_id"),
	);
	append_events::<ChannelEvents::OpenTry>(
		&mut events,
		extract_events(block_events, "channel_open_try", "channel_id"),
	);
	append_events::<ChannelEvents::OpenAck>(
		&mut events,
		extract_events(block_events, "channel_open_ack", "channel_id"),
	);
	append_events::<ChannelEvents::OpenConfirm>(
		&mut events,
		extract_events(block_events, "channel_open_confirm", "channel_id"),
	);
	append_events::<ChannelEvents::SendPacket>(
		&mut events,
		extract_events(block_events, "send_packet", "packet_data"),
	);
	append_events::<ChannelEvents::CloseInit>(
		&mut events,
		extract_events(block_events, "channel_close_init", "channel_id"),
	);
	append_events::<ChannelEvents::CloseConfirm>(
		&mut events,
		extract_events(block_events, "channel_close_confirm", "channel_id"),
	);
	events
}

pub fn ibc_event_try_from_abci_event(abci_event: &AbciEvent) -> Result<IbcEvent, Error> {
	match abci_event.type_str.parse() {
		Ok(IbcEventType::CreateClient) => Ok(IbcEvent::CreateClient(
			create_client_try_from_abci_event(abci_event).map_err(IbcEventError::client)?,
		)),
		Ok(IbcEventType::UpdateClient) => Ok(IbcEvent::UpdateClient(
			update_client_try_from_abci_event(abci_event).map_err(IbcEventError::client)?,
		)),
		Ok(IbcEventType::UpgradeClient) => Ok(IbcEvent::UpgradeClient(
			upgrade_client_try_from_abci_event(abci_event).map_err(IbcEventError::client)?,
		)),
		Ok(IbcEventType::ClientMisbehaviour) => Ok(IbcEvent::ClientMisbehaviour(
			client_misbehaviour_try_from_abci_event(abci_event).map_err(IbcEventError::client)?,
		)),
		Ok(IbcEventType::OpenInitConnection) => Ok(IbcEvent::OpenInitConnection(
			connection_open_init_try_from_abci_event(abci_event).map_err(|e| {
				Error::from(format!("Failed to parse connection open init event {:?}", e))
			})?,
		)),
		Ok(IbcEventType::OpenTryConnection) => Ok(IbcEvent::OpenTryConnection(
			connection_open_try_try_from_abci_event(abci_event).map_err(|e| {
				Error::from(format!("Failed to parse connection open try event {:?}", e))
			})?,
		)),
		Ok(IbcEventType::OpenAckConnection) => Ok(IbcEvent::OpenAckConnection(
			connection_open_ack_try_from_abci_event(abci_event).map_err(|e| {
				Error::from(format!("Failed to parse connection open ack event {:?}", e))
			})?,
		)),
		Ok(IbcEventType::OpenConfirmConnection) => Ok(IbcEvent::OpenConfirmConnection(
			connection_open_confirm_try_from_abci_event(abci_event).map_err(|e| {
				Error::from(format!("Failed to parse connection open confirm event {:?}", e))
			})?,
		)),
		Ok(IbcEventType::OpenInitChannel) => Ok(IbcEvent::OpenInitChannel(
			channel_open_init_try_from_abci_event(abci_event).map_err(IbcEventError::channel)?,
		)),
		Ok(IbcEventType::OpenTryChannel) => Ok(IbcEvent::OpenTryChannel(
			channel_open_try_try_from_abci_event(abci_event).map_err(IbcEventError::channel)?,
		)),
		Ok(IbcEventType::OpenAckChannel) => Ok(IbcEvent::OpenAckChannel(
			channel_open_ack_try_from_abci_event(abci_event).map_err(IbcEventError::channel)?,
		)),
		Ok(IbcEventType::OpenConfirmChannel) => Ok(IbcEvent::OpenConfirmChannel(
			channel_open_confirm_try_from_abci_event(abci_event).map_err(IbcEventError::channel)?,
		)),
		Ok(IbcEventType::CloseInitChannel) => Ok(IbcEvent::CloseInitChannel(
			channel_close_init_try_from_abci_event(abci_event).map_err(IbcEventError::channel)?,
		)),
		Ok(IbcEventType::CloseConfirmChannel) => Ok(IbcEvent::CloseConfirmChannel(
			channel_close_confirm_try_from_abci_event(abci_event)
				.map_err(IbcEventError::channel)?,
		)),
		Ok(IbcEventType::SendPacket) => Ok(IbcEvent::SendPacket(
			send_packet_try_from_abci_event(abci_event).map_err(IbcEventError::channel)?,
		)),
		Ok(IbcEventType::WriteAck) => Ok(IbcEvent::WriteAcknowledgement(
			write_acknowledgement_try_from_abci_event(abci_event)
				.map_err(IbcEventError::channel)?,
		)),
		Ok(IbcEventType::AckPacket) => Ok(IbcEvent::AcknowledgePacket(
			acknowledge_packet_try_from_abci_event(abci_event).map_err(IbcEventError::channel)?,
		)),
		Ok(IbcEventType::Timeout) => Ok(IbcEvent::TimeoutPacket(
			timeout_packet_try_from_abci_event(abci_event).map_err(IbcEventError::channel)?,
		)),
		_ => Err(Error::from(format!("Failed to parse event type {:?}", abci_event.type_str))),
	}
}

pub fn create_client_try_from_abci_event(
	abci_event: &AbciEvent,
) -> Result<ClientEvents::CreateClient, Error> {
	client_extract_attributes_from_tx(abci_event).map(ClientEvents::CreateClient)
}

pub fn update_client_try_from_abci_event(
	abci_event: &AbciEvent,
) -> Result<ClientEvents::UpdateClient, Error> {
	client_extract_attributes_from_tx(abci_event).map(|attributes| ClientEvents::UpdateClient {
		common: attributes,
		header: extract_header_from_tx(abci_event).ok(),
	})
}

pub fn upgrade_client_try_from_abci_event(
	abci_event: &AbciEvent,
) -> Result<ClientEvents::UpgradeClient, Error> {
	client_extract_attributes_from_tx(abci_event).map(ClientEvents::UpgradeClient)
}

pub fn client_misbehaviour_try_from_abci_event(
	abci_event: &AbciEvent,
) -> Result<ClientEvents::ClientMisbehaviour, Error> {
	client_extract_attributes_from_tx(abci_event).map(ClientEvents::ClientMisbehaviour)
}

pub fn connection_open_init_try_from_abci_event(
	abci_event: &AbciEvent,
) -> Result<ConnectionEvents::OpenInit, Error> {
	connection_extract_attributes_from_tx(abci_event).map(ConnectionEvents::OpenInit)
}

pub fn connection_open_try_try_from_abci_event(
	abci_event: &AbciEvent,
) -> Result<ConnectionEvents::OpenTry, Error> {
	connection_extract_attributes_from_tx(abci_event).map(ConnectionEvents::OpenTry)
}

pub fn connection_open_ack_try_from_abci_event(
	abci_event: &AbciEvent,
) -> Result<ConnectionEvents::OpenAck, Error> {
	connection_extract_attributes_from_tx(abci_event).map(ConnectionEvents::OpenAck)
}

pub fn connection_open_confirm_try_from_abci_event(
	abci_event: &AbciEvent,
) -> Result<ConnectionEvents::OpenConfirm, Error> {
	connection_extract_attributes_from_tx(abci_event).map(ConnectionEvents::OpenConfirm)
}

pub fn channel_open_init_try_from_abci_event(
	abci_event: &AbciEvent,
) -> Result<ConnectionEvents::OpenInit, Error> {
	match channel_extract_attributes_from_tx(abci_event) {
		Ok(attrs) => ChannelEvents::OpenInit::try_from(attrs)
			.map_err(|e| Error::from(format!("Failed to parse channel open init event {:?}", e))),
		Err(e) => Err(e),
	}
}

pub fn channel_open_try_try_from_abci_event(
	abci_event: &AbciEvent,
) -> Result<ChannelEvents::OpenTry, Error> {
	match channel_extract_attributes_from_tx(abci_event) {
		Ok(attrs) => ChannelEvents::OpenTry::try_from(attrs)
			.map_err(|e| Error::from(format!("Failed to parse channel open try event {:?}", e))),
		Err(e) => Err(e),
	}
}

pub fn channel_open_ack_try_from_abci_event(
	abci_event: &AbciEvent,
) -> Result<ChannelEvents::OpenAck, Error> {
	match channel_extract_attributes_from_tx(abci_event) {
		Ok(attrs) => ChannelEvents::OpenAck::try_from(attrs)
			.map_err(|e| Error::from(format!("Failed to parse channel open ack event {:?}", e))),
		Err(e) => Err(e),
	}
}

pub fn channel_open_confirm_try_from_abci_event(
	abci_event: &AbciEvent,
) -> Result<ChannelEvents::OpenConfirm, Error> {
	match channel_extract_attributes_from_tx(abci_event) {
		Ok(attrs) => ChannelEvents::OpenConfirm::try_from(attrs).map_err(|e| {
			Error::from(format!("Failed to parse channel open confirm event {:?}", e))
		}),
		Err(e) => Err(e),
	}
}

pub fn channel_close_init_try_from_abci_event(
	abci_event: &AbciEvent,
) -> Result<ChannelEvents::CloseInit, Error> {
	match channel_extract_attributes_from_tx(abci_event) {
		Ok(attrs) => ChannelEvents::CloseInit::try_from(attrs)
			.map_err(|e| Error::from(format!("Failed to parse channel close init event {:?}", e))),
		Err(e) => Err(e),
	}
}

pub fn channel_close_confirm_try_from_abci_event(
	abci_event: &AbciEvent,
) -> Result<ChannelEvents::CloseConfirm, Error> {
	match channel_extract_attributes_from_tx(abci_event) {
		Ok(attrs) => ChannelEvents::CloseConfirm::try_from(attrs).map_err(|e| {
			Error::from(format!("Failed to parse channel close confirm event {:?}", e))
		}),
		Err(e) => Err(e),
	}
}

pub fn send_packet_try_from_abci_event(
	abci_event: &AbciEvent,
) -> Result<ChannelEvents::SendPacket, Error> {
	extract_packet_and_write_ack_from_tx(abci_event)
		.map(|(packet, write_ack)| {
			// This event should not have a write ack.
			debug_assert_eq!(write_ack.len(), 0);
			ChannelEvents::SendPacket { height: _, packet }
		})
		.map_err(|e| Error::from(format!("Failed to parse send packet event {:?}", e)))
}

pub fn write_acknowledgement_try_from_abci_event(
	abci_event: &AbciEvent,
) -> Result<ChannelEvents::WriteAcknowledgement, Error> {
	extract_packet_and_write_ack_from_tx(abci_event)
		.map(|(packet, write_ack)| ChannelEvents::WriteAcknowledgement {
			height: _,
			packet,
			ack: write_ack,
		})
		.map_err(|e| Error::from(format!("Failed to parse write acknowledgement event {:?}", e)))
}

pub fn acknowledge_packet_try_from_abci_event(
	abci_event: &AbciEvent,
) -> Result<ChannelEvents::AcknowledgePacket, Error> {
	extract_packet_and_write_ack_from_tx(abci_event)
		.map(|(packet, write_ack)| {
			// This event should not have a write ack.
			debug_assert_eq!(write_ack.len(), 0);
			ChannelEvents::AcknowledgePacket { height: _, packet }
		})
		.map_err(|e| Error::from(format!("Failed to parse acknowledge packet event {:?}", e)))
}

pub fn timeout_packet_try_from_abci_event(
	abci_event: &AbciEvent,
) -> Result<ChannelEvents::TimeoutPacket, Error> {
	extract_packet_and_write_ack_from_tx(abci_event)
		.map(|(packet, write_ack)| {
			// This event should not have a write ack.
			debug_assert_eq!(write_ack.len(), 0);
			ChannelEvents::TimeoutPacket { height: _, packet }
		})
		.map_err(|e| Error::from(format!("Failed to parse timeout packet event {:?}", e)))
}

pub fn event_is_type_client(ev: &IbcEvent) -> bool {
	matches!(
		ev,
		IbcEvent::CreateClient(_)
			| IbcEvent::UpdateClient(_)
			| IbcEvent::UpgradeClient(_)
			| IbcEvent::ClientMisbehaviour(_)
	)
}

pub fn event_is_type_connection(ev: &IbcEvent) -> bool {
	matches!(
		ev,
		IbcEvent::OpenInitConnection(_)
			| IbcEvent::OpenTryConnection(_)
			| IbcEvent::OpenAckConnection(_)
			| IbcEvent::OpenConfirmConnection(_)
	)
}

pub fn event_is_type_channel(ev: &IbcEvent) -> bool {
	matches!(
		ev,
		IbcEvent::OpenInitChannel(_)
			| IbcEvent::OpenTryChannel(_)
			| IbcEvent::OpenAckChannel(_)
			| IbcEvent::OpenConfirmChannel(_)
			| IbcEvent::CloseInitChannel(_)
			| IbcEvent::CloseConfirmChannel(_)
			| IbcEvent::SendPacket(_)
			| IbcEvent::ReceivePacket(_)
			| IbcEvent::WriteAcknowledgement(_)
			| IbcEvent::AcknowledgePacket(_)
			| IbcEvent::TimeoutPacket(_)
			| IbcEvent::TimeoutOnClosePacket(_)
	)
}

pub fn client_extract_attributes_from_tx(event: &AbciEvent) -> Result<ClientAttributes, Error> {
	let mut attr = ClientAttributes::default();

	for tag in &event.attributes {
		let key = tag.key.as_ref();
		let value = tag.value.as_ref();
		match key {
			ClientEvents::CLIENT_ID_ATTRIBUTE_KEY => {
				attr.client_id = value.parse().map_err(|e| {
					Error::from(format!("Failed to parse client id from event attribute: {:?}", e))
				})?
			},
			ClientEvents::CLIENT_TYPE_ATTRIBUTE_KEY => {
				attr.client_type = value.parse().map_err(|e| {
					Error::from(format!(
						"Failed to parse client type from event attribute: {:?}",
						e
					))
				})?
			},
			ClientEvents::CONSENSUS_HEIGHT_ATTRIBUTE_KEY => {
				attr.consensus_height = value.parse().map_err(|e| {
					Error::from(format!(
						"Failed to parse consensus height from event attribute: {:?}",
						e
					))
				})?
			},
			_ => {},
		}
	}

	Ok(attr)
}

pub fn extract_header_from_tx(event: &AbciEvent) -> Result<Box<dyn Header>, Error> {
	for tag in &event.attributes {
		let key = tag.key.as_ref();
		let value = tag.value.as_ref();
		if key == "header" {
			let header_bytes = hex::decode(value).map_err(|e| Error::from(e.to_string()))?;
			return decode_header(&header_bytes);
		}
	}
	Err(Error::from("Failed to extract header from event"))
}

fn connection_extract_attributes_from_tx(event: &AbciEvent) -> Result<ConnectionAttributes, Error> {
	let mut attr = ConnectionAttributes::default();

	for tag in &event.attributes {
		let key = tag.key.as_ref();
		let value = tag.value.as_ref();
		match key {
			ConnectionEvents::CONN_ID_ATTRIBUTE_KEY => {
				attr.connection_id = value.parse().ok();
			},
			ConnectionEvents::CLIENT_ID_ATTRIBUTE_KEY => {
				attr.client_id = value.parse().map_err(|e| Error::from(e.to_string()))?
			},
			ConnectionEvents::COUNTERPARTY_CONN_ID_ATTRIBUTE_KEY => {
				attr.counterparty_connection_id = value.parse().ok();
			},
			ConnectionEvents::COUNTERPARTY_CLIENT_ID_ATTRIBUTE_KEY => {
				attr.counterparty_client_id =
					value.parse().map_err(|e| Error::from(e.to_string()))?
			},
			_ => {},
		}
	}

	Ok(attr)
}

fn channel_extract_attributes_from_tx(event: &AbciEvent) -> Result<ChannelAttributes, Error> {
	let mut attr = ChannelAttributes::default();

	for tag in &event.attributes {
		let key = tag.key.as_ref();
		let value = tag.value.as_ref();
		match key {
			ChannelEvents::PORT_ID_ATTRIBUTE_KEY => {
				attr.port_id = value.parse().map_err(|e| Error::from(e.to_string()))?
			},
			ChannelEvents::CHANNEL_ID_ATTRIBUTE_KEY => {
				attr.channel_id = value.parse().ok();
			},
			ChannelEvents::CONNECTION_ID_ATTRIBUTE_KEY => {
				attr.connection_id = value.parse().map_err(|e| Error::from(e.to_string()))?
			},
			ChannelEvents::COUNTERPARTY_PORT_ID_ATTRIBUTE_KEY => {
				attr.counterparty_port_id = value.parse().map_err(|e| Error::from(e.to_string()))?
			},
			ChannelEvents::COUNTERPARTY_CHANNEL_ID_ATTRIBUTE_KEY => {
				attr.counterparty_channel_id = value.parse().ok();
			},
			_ => {},
		}
	}

	Ok(attr)
}

fn extract_packet_and_write_ack_from_tx(event: &AbciEvent) -> Result<(Packet, Vec<u8>), Error> {
	let mut packet = Packet::default();
	let mut write_ack: Vec<u8> = Vec::new();
	for tag in &event.attributes {
		let key = tag.key.as_ref();
		let value = tag.value.as_ref();
		match key {
			ChannelEvents::PKT_SRC_PORT_ATTRIBUTE_KEY => {
				packet.source_port = value.parse().map_err(|e| Error::from(e.to_string()))?
			},
			ChannelEvents::PKT_SRC_CHANNEL_ATTRIBUTE_KEY => {
				packet.source_channel = value.parse().map_err(|e| Error::from(e.to_string()))?
			},
			ChannelEvents::PKT_DST_PORT_ATTRIBUTE_KEY => {
				packet.destination_port = value.parse().map_err(|e| Error::from(e.to_string()))?
			},
			ChannelEvents::PKT_DST_CHANNEL_ATTRIBUTE_KEY => {
				packet.destination_channel =
					value.parse().map_err(|e| Error::from(e.to_string()))?
			},
			ChannelEvents::PKT_SEQ_ATTRIBUTE_KEY => {
				packet.sequence =
					value.parse::<u64>().map_err(|e| Error::from(e.to_string()))?.into()
			},
			ChannelEvents::PKT_TIMEOUT_HEIGHT_ATTRIBUTE_KEY => {
				packet.timeout_height = value.parse().map_err(|e| Error::from(e.to_string()))?
			},
			ChannelEvents::PKT_TIMEOUT_TIMESTAMP_ATTRIBUTE_KEY => {
				packet.timeout_timestamp = value.parse().unwrap();
			},
			ChannelEvents::PKT_DATA_ATTRIBUTE_KEY => {
				packet.data = Vec::from(value.as_bytes());
			},
			ChannelEvents::PKT_ACK_ATTRIBUTE_KEY => {
				write_ack = Vec::from(value.as_bytes());
			},
			_ => {},
		}
	}

	Ok((packet, write_ack))
}
