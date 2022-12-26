use core::convert::TryFrom;
use core::fmt::{Display, Error as FmtError, Formatter};
use ibc_relayer_types::clients::ics07_tendermint::header::decode_header as tm_decode_header;
use ibc_relayer_types::events::{Error as IbcEventError, IbcEvent, IbcEventType};
use ibc_relayer_types::{
    core::ics02_client::{
        error::Error as ClientError,
        events::HEADER_ATTRIBUTE_KEY,
        events::{self as client_events, Attributes as ClientAttributes},
        header::Header,
        height::{Height, HeightErrorDetail},
    },
    core::ics03_connection::{
        error::Error as ConnectionError,
        events::{self as connection_events, Attributes as ConnectionAttributes},
    },
    core::ics04_channel::{
        error::Error as ChannelError,
        events::{self as channel_events, Attributes as ChannelAttributes},
        packet::Packet,
        timeout::TimeoutHeight,
    },
};

use serde::Serialize;
use tendermint::abci::Event as AbciEvent;

#[derive(Clone, Debug, Serialize)]
pub struct IbcEventWithHeight {
    pub event: IbcEvent,
    pub height: Height,
}

impl IbcEventWithHeight {
    pub fn new(event: IbcEvent, height: Height) -> Self {
        Self { event, height }
    }

    pub fn with_height(self, height: Height) -> Self {
        Self {
            event: self.event,
            height,
        }
    }
}

impl Display for IbcEventWithHeight {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, "{} at height {}", self.event, self.height)
    }
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

/// Note: This function, as well as other helpers, are needed as a workaround to
/// Rust's orphan rule. That is, we want the AbciEvent -> IbcEvent to be defined
/// in the relayer crate, but can't because neither AbciEvent nor IbcEvent are
/// defined in this crate. Hence, we are forced to make an ad-hoc function for
/// it.
pub fn ibc_event_try_from_abci_event(abci_event: &AbciEvent) -> Result<IbcEvent, IbcEventError> {
    match abci_event.kind.parse() {
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
            connection_open_init_try_from_abci_event(abci_event)
                .map_err(IbcEventError::connection)?,
        )),
        Ok(IbcEventType::OpenTryConnection) => Ok(IbcEvent::OpenTryConnection(
            connection_open_try_try_from_abci_event(abci_event)
                .map_err(IbcEventError::connection)?,
        )),
        Ok(IbcEventType::OpenAckConnection) => Ok(IbcEvent::OpenAckConnection(
            connection_open_ack_try_from_abci_event(abci_event)
                .map_err(IbcEventError::connection)?,
        )),
        Ok(IbcEventType::OpenConfirmConnection) => Ok(IbcEvent::OpenConfirmConnection(
            connection_open_confirm_try_from_abci_event(abci_event)
                .map_err(IbcEventError::connection)?,
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
        _ => Err(IbcEventError::unsupported_abci_event(
            abci_event.kind.to_owned(),
        )),
    }
}

pub fn create_client_try_from_abci_event(
    abci_event: &AbciEvent,
) -> Result<client_events::CreateClient, ClientError> {
    client_extract_attributes_from_tx(abci_event).map(client_events::CreateClient)
}

pub fn update_client_try_from_abci_event(
    abci_event: &AbciEvent,
) -> Result<client_events::UpdateClient, ClientError> {
    client_extract_attributes_from_tx(abci_event).map(|attributes| client_events::UpdateClient {
        common: attributes,
        header: extract_header_from_tx(abci_event).ok(),
    })
}

pub fn upgrade_client_try_from_abci_event(
    abci_event: &AbciEvent,
) -> Result<client_events::UpgradeClient, ClientError> {
    client_extract_attributes_from_tx(abci_event).map(client_events::UpgradeClient)
}

pub fn client_misbehaviour_try_from_abci_event(
    abci_event: &AbciEvent,
) -> Result<client_events::ClientMisbehaviour, ClientError> {
    client_extract_attributes_from_tx(abci_event).map(client_events::ClientMisbehaviour)
}

pub fn connection_open_init_try_from_abci_event(
    abci_event: &AbciEvent,
) -> Result<connection_events::OpenInit, ConnectionError> {
    connection_extract_attributes_from_tx(abci_event).map(connection_events::OpenInit)
}

pub fn connection_open_try_try_from_abci_event(
    abci_event: &AbciEvent,
) -> Result<connection_events::OpenTry, ConnectionError> {
    connection_extract_attributes_from_tx(abci_event).map(connection_events::OpenTry)
}

pub fn connection_open_ack_try_from_abci_event(
    abci_event: &AbciEvent,
) -> Result<connection_events::OpenAck, ConnectionError> {
    connection_extract_attributes_from_tx(abci_event).map(connection_events::OpenAck)
}

pub fn connection_open_confirm_try_from_abci_event(
    abci_event: &AbciEvent,
) -> Result<connection_events::OpenConfirm, ConnectionError> {
    connection_extract_attributes_from_tx(abci_event).map(connection_events::OpenConfirm)
}

pub fn channel_open_init_try_from_abci_event(
    abci_event: &AbciEvent,
) -> Result<channel_events::OpenInit, ChannelError> {
    match channel_extract_attributes_from_tx(abci_event) {
        Ok(attrs) => channel_events::OpenInit::try_from(attrs)
            .map_err(|_| ChannelError::implementation_specific()),
        Err(e) => Err(e),
    }
}

pub fn channel_open_try_try_from_abci_event(
    abci_event: &AbciEvent,
) -> Result<channel_events::OpenTry, ChannelError> {
    match channel_extract_attributes_from_tx(abci_event) {
        Ok(attrs) => channel_events::OpenTry::try_from(attrs)
            .map_err(|_| ChannelError::implementation_specific()),
        Err(e) => Err(e),
    }
}

pub fn channel_open_ack_try_from_abci_event(
    abci_event: &AbciEvent,
) -> Result<channel_events::OpenAck, ChannelError> {
    match channel_extract_attributes_from_tx(abci_event) {
        Ok(attrs) => channel_events::OpenAck::try_from(attrs)
            .map_err(|_| ChannelError::implementation_specific()),
        Err(e) => Err(e),
    }
}

pub fn channel_open_confirm_try_from_abci_event(
    abci_event: &AbciEvent,
) -> Result<channel_events::OpenConfirm, ChannelError> {
    match channel_extract_attributes_from_tx(abci_event) {
        Ok(attrs) => channel_events::OpenConfirm::try_from(attrs)
            .map_err(|_| ChannelError::implementation_specific()),
        Err(e) => Err(e),
    }
}

pub fn channel_close_init_try_from_abci_event(
    abci_event: &AbciEvent,
) -> Result<channel_events::CloseInit, ChannelError> {
    match channel_extract_attributes_from_tx(abci_event) {
        Ok(attrs) => channel_events::CloseInit::try_from(attrs)
            .map_err(|_| ChannelError::implementation_specific()),
        Err(e) => Err(e),
    }
}

pub fn channel_close_confirm_try_from_abci_event(
    abci_event: &AbciEvent,
) -> Result<channel_events::CloseConfirm, ChannelError> {
    match channel_extract_attributes_from_tx(abci_event) {
        Ok(attrs) => channel_events::CloseConfirm::try_from(attrs)
            .map_err(|_| ChannelError::implementation_specific()),
        Err(e) => Err(e),
    }
}

pub fn send_packet_try_from_abci_event(
    abci_event: &AbciEvent,
) -> Result<channel_events::SendPacket, ChannelError> {
    extract_packet_and_write_ack_from_tx(abci_event)
        .map(|(packet, write_ack)| {
            // This event should not have a write ack.
            debug_assert_eq!(write_ack.len(), 0);
            channel_events::SendPacket { packet }
        })
        .map_err(|_| ChannelError::abci_conversion_failed(abci_event.kind.to_owned()))
}

pub fn write_acknowledgement_try_from_abci_event(
    abci_event: &AbciEvent,
) -> Result<channel_events::WriteAcknowledgement, ChannelError> {
    extract_packet_and_write_ack_from_tx(abci_event)
        .map(|(packet, write_ack)| channel_events::WriteAcknowledgement {
            packet,
            ack: write_ack,
        })
        .map_err(|_| ChannelError::abci_conversion_failed(abci_event.kind.to_owned()))
}

pub fn acknowledge_packet_try_from_abci_event(
    abci_event: &AbciEvent,
) -> Result<channel_events::AcknowledgePacket, ChannelError> {
    extract_packet_and_write_ack_from_tx(abci_event)
        .map(|(packet, write_ack)| {
            // This event should not have a write ack.
            debug_assert_eq!(write_ack.len(), 0);
            channel_events::AcknowledgePacket { packet }
        })
        .map_err(|_| ChannelError::abci_conversion_failed(abci_event.kind.to_owned()))
}

pub fn timeout_packet_try_from_abci_event(
    abci_event: &AbciEvent,
) -> Result<channel_events::TimeoutPacket, ChannelError> {
    extract_packet_and_write_ack_from_tx(abci_event)
        .map(|(packet, write_ack)| {
            // This event should not have a write ack.
            debug_assert_eq!(write_ack.len(), 0);
            channel_events::TimeoutPacket { packet }
        })
        .map_err(|_| ChannelError::abci_conversion_failed(abci_event.kind.to_owned()))
}

fn client_extract_attributes_from_tx(event: &AbciEvent) -> Result<ClientAttributes, ClientError> {
    let mut attr = ClientAttributes::default();

    for tag in &event.attributes {
        let key = tag.key.as_str();
        let value = tag.value.as_str();
        match key {
            client_events::CLIENT_ID_ATTRIBUTE_KEY => {
                attr.client_id = value
                    .parse()
                    .map_err(ClientError::invalid_client_identifier)?
            }
            client_events::CLIENT_TYPE_ATTRIBUTE_KEY => {
                attr.client_type = value
                    .parse()
                    .map_err(|_| ClientError::unknown_client_type(value.to_string()))?
            }
            client_events::CONSENSUS_HEIGHT_ATTRIBUTE_KEY => {
                attr.consensus_height = value
                    .parse()
                    .map_err(|e| ClientError::invalid_string_as_height(value.to_string(), e))?
            }
            _ => {}
        }
    }

    Ok(attr)
}

pub fn extract_header_from_tx(event: &AbciEvent) -> Result<Box<dyn Header>, ClientError> {
    for tag in &event.attributes {
        let key = tag.key.as_str();
        let value = tag.value.as_str();
        if key == HEADER_ATTRIBUTE_KEY {
            let header_bytes = hex::decode(value).map_err(|_| ClientError::malformed_header())?;
            return decode_header(&header_bytes);
        }
    }
    Err(ClientError::missing_raw_header())
}

fn connection_extract_attributes_from_tx(
    event: &AbciEvent,
) -> Result<ConnectionAttributes, ConnectionError> {
    let mut attr = ConnectionAttributes::default();

    for tag in &event.attributes {
        let key = tag.key.as_str();
        let value = tag.value.as_str();
        match key {
            connection_events::CONN_ID_ATTRIBUTE_KEY => {
                attr.connection_id = value.parse().ok();
            }
            connection_events::CLIENT_ID_ATTRIBUTE_KEY => {
                attr.client_id = value.parse().map_err(ConnectionError::invalid_identifier)?;
            }
            connection_events::COUNTERPARTY_CONN_ID_ATTRIBUTE_KEY => {
                attr.counterparty_connection_id = value.parse().ok();
            }
            connection_events::COUNTERPARTY_CLIENT_ID_ATTRIBUTE_KEY => {
                attr.counterparty_client_id =
                    value.parse().map_err(ConnectionError::invalid_identifier)?;
            }
            _ => {}
        }
    }

    Ok(attr)
}

fn channel_extract_attributes_from_tx(
    event: &AbciEvent,
) -> Result<ChannelAttributes, ChannelError> {
    let mut attr = ChannelAttributes::default();

    for tag in &event.attributes {
        let key = tag.key.as_str();
        let value = tag.value.as_str();
        match key {
            channel_events::PORT_ID_ATTRIBUTE_KEY => {
                attr.port_id = value.parse().map_err(ChannelError::identifier)?
            }
            channel_events::CHANNEL_ID_ATTRIBUTE_KEY => {
                attr.channel_id = value.parse().ok();
            }
            channel_events::CONNECTION_ID_ATTRIBUTE_KEY => {
                attr.connection_id = value.parse().map_err(ChannelError::identifier)?;
            }
            channel_events::COUNTERPARTY_PORT_ID_ATTRIBUTE_KEY => {
                attr.counterparty_port_id = value.parse().map_err(ChannelError::identifier)?;
            }
            channel_events::COUNTERPARTY_CHANNEL_ID_ATTRIBUTE_KEY => {
                attr.counterparty_channel_id = value.parse().ok();
            }
            _ => {}
        }
    }

    Ok(attr)
}

fn extract_packet_and_write_ack_from_tx(
    event: &AbciEvent,
) -> Result<(Packet, Vec<u8>), ChannelError> {
    let mut packet = Packet::default();
    let mut write_ack: Vec<u8> = Vec::new();
    for tag in &event.attributes {
        let key = tag.key.as_str();
        let value = tag.value.as_str();
        match key {
            channel_events::PKT_SRC_PORT_ATTRIBUTE_KEY => {
                packet.source_port = value.parse().map_err(ChannelError::identifier)?;
            }
            channel_events::PKT_SRC_CHANNEL_ATTRIBUTE_KEY => {
                packet.source_channel = value.parse().map_err(ChannelError::identifier)?;
            }
            channel_events::PKT_DST_PORT_ATTRIBUTE_KEY => {
                packet.destination_port = value.parse().map_err(ChannelError::identifier)?;
            }
            channel_events::PKT_DST_CHANNEL_ATTRIBUTE_KEY => {
                packet.destination_channel = value.parse().map_err(ChannelError::identifier)?;
            }
            channel_events::PKT_SEQ_ATTRIBUTE_KEY => {
                packet.sequence = value
                    .parse::<u64>()
                    .map_err(|e| ChannelError::invalid_string_as_sequence(value.to_string(), e))?
                    .into()
            }
            channel_events::PKT_TIMEOUT_HEIGHT_ATTRIBUTE_KEY => {
                packet.timeout_height = parse_timeout_height(value)?;
            }
            channel_events::PKT_TIMEOUT_TIMESTAMP_ATTRIBUTE_KEY => {
                packet.timeout_timestamp = value.parse().unwrap();
            }
            channel_events::PKT_DATA_ATTRIBUTE_KEY => {
                packet.data = Vec::from(value.as_bytes());
            }
            channel_events::PKT_ACK_ATTRIBUTE_KEY => {
                write_ack = Vec::from(value.as_bytes());
            }
            _ => {}
        }
    }

    Ok((packet, write_ack))
}

/// Parse a string into a timeout height expected to be stored in
/// `Packet.timeout_height`. We need to parse the timeout height differently
/// because of a quirk introduced in ibc-go. See comment in
/// `TryFrom<RawPacket> for Packet`.
pub fn parse_timeout_height(s: &str) -> Result<TimeoutHeight, ChannelError> {
    match s.parse::<Height>() {
        Ok(height) => Ok(TimeoutHeight::from(height)),
        Err(e) => match e.into_detail() {
            HeightErrorDetail::ZeroHeight(_) => Ok(TimeoutHeight::no_timeout()),
            _ => Err(ChannelError::invalid_timeout_height()),
        },
    }
}

/// Decodes an encoded header into a known `Header` type,
pub fn decode_header(header_bytes: &[u8]) -> Result<Box<dyn Header>, ClientError> {
    let header = tm_decode_header(header_bytes)?;
    Ok(Box::new(header))
}

// fn extract_block_events(
// 	height: Height,
// 	block_events: &HashMap<String, Vec<String>>,
// ) -> Vec<IbcEvent> {
// 	#[inline]
// 	fn extract_events<'a, T: TryFrom<RawObject<'a>>>(
// 		height: Height,
// 		block_events: &'a HashMap<String, Vec<String>>,
// 		event_type: &str,
// 		event_field: &str,
// 	) -> Vec<T> {
// 		block_events
// 			.get(&format!("{}.{}", event_type, event_field))
// 			.unwrap_or(&vec![])
// 			.iter()
// 			.enumerate()
// 			.filter_map(|(i, _)| {
// 				let raw_obj = RawObject::new(height, event_type.to_owned(), i, block_events);
// 				T::try_from(raw_obj).ok()
// 			})
// 			.collect()
// 	}

// 	#[inline]
// 	fn append_events<T: Into<IbcEvent>>(
// 		events: &mut Vec<IbcEventWithHeight>,
// 		chan_events: Vec<T>,
// 		height: Height,
// 	) {
// 		events.append(
// 			&mut chan_events
// 				.into_iter()
// 				.map(|ev| IbcEventWithHeight::new(ev.into(), height))
// 				.collect(),
// 		);
// 	}

// 	let mut events: Vec<IbcEventWithHeight> = vec![];
// 	append_events::<ChannelEvents::OpenInit>(
// 		&mut events,
// 		extract_events(height, block_events, "channel_open_init", "channel_id"),
// 		height,
// 	);
// 	append_events::<ChannelEvents::OpenTry>(
// 		&mut events,
// 		extract_events(height, block_events, "channel_open_try", "channel_id"),
// 		height,
// 	);
// 	append_events::<ChannelEvents::OpenAck>(
// 		&mut events,
// 		extract_events(height, block_events, "channel_open_ack", "channel_id"),
// 		height,
// 	);
// 	append_events::<ChannelEvents::OpenConfirm>(
// 		&mut events,
// 		extract_events(height, block_events, "channel_open_confirm", "channel_id"),
// 		height,
// 	);
// 	append_events::<ChannelEvents::SendPacket>(
// 		&mut events,
// 		extract_events(height, block_events, "send_packet", "packet_data"),
// 		height,
// 	);
// 	append_events::<ChannelEvents::CloseInit>(
// 		&mut events,
// 		extract_events(height, block_events, "channel_close_init", "channel_id"),
// 		height,
// 	);
// 	append_events::<ChannelEvents::CloseConfirm>(
// 		&mut events,
// 		extract_events(height, block_events, "channel_close_confirm", "channel_id"),
// 		height,
// 	);
// 	events
// }
