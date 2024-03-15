use anchor_client::{
	solana_client::{
		nonblocking::rpc_client::RpcClient, rpc_client::GetConfirmedSignaturesForAddress2Config,
	},
	solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey},
};
use guestchain::{BlockHeader, Signature as SignatureTrait};
use lib::hash::CryptoHash;
use serde::{Deserialize, Serialize};
use solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta;
use std::{str::FromStr, thread::sleep, time::Duration};

use base64::Engine;
use ibc::{
	core::{
		ics02_client::{
			client_state::ClientType,
			events::{
				Attributes as ClientAttributes, ClientMisbehaviour, CreateClient, UpdateClient,
				UpgradeClient,
			},
		},
		ics03_connection::events::{
			Attributes as ConnAttributes, OpenAck as ConnOpenAck, OpenConfirm as ConnOpenConfirm,
			OpenInit as ConnOpenInit, OpenTry as ConnOpenTry,
		},
		ics04_channel::{
			events::{
				AcknowledgePacket, CloseConfirm as ChanCloseConfirm, CloseInit as ChanCloseInit,
				OpenAck as ChanOpenAck, OpenConfirm as ChanOpenConfirm, OpenInit as ChanOpenInit,
				OpenTry as ChanOpenTry, ReceivePacket, SendPacket, TimeoutPacket,
				WriteAcknowledgement,
			},
			packet::{Packet, Sequence},
		},
		ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
		ics26_routing::context::ModuleId,
	},
	events::{IbcEvent, ModuleEvent, ModuleEventAttribute},
	timestamp::Timestamp,
	Height,
};
use pallet_ibc::light_clients::{PubKey, Signature};

pub fn convert_new_event_to_old(
	event: ibc_core_handler_types::events::IbcEvent,
	height: Height,
) -> Option<IbcEvent> {
	match event {
		ibc_core_handler_types::events::IbcEvent::CreateClient(e) => {
			let eve = CreateClient(ClientAttributes {
				height: Height {
					revision_number: e.consensus_height().revision_number(),
					revision_height: e.consensus_height().revision_height(),
				},
				client_id: ClientId::from_str(e.client_id().as_str()).unwrap(),
				client_type: ClientType::from_str(e.client_type().as_str()).unwrap(),
				consensus_height: Height {
					revision_number: e.consensus_height().revision_number(),
					revision_height: e.consensus_height().revision_height(),
				},
			});
			Some(IbcEvent::CreateClient(eve))
		},
		ibc_core_handler_types::events::IbcEvent::UpdateClient(e) => {
			let eve = UpdateClient {
				common: ClientAttributes {
					height: Height {
						revision_number: e.consensus_height().revision_number(),
						revision_height: e.consensus_height().revision_height(),
					},
					client_id: ClientId::from_str(e.client_id().as_str()).unwrap(),
					client_type: ClientType::from_str(e.client_type().as_str()).unwrap(),
					consensus_height: Height {
						revision_number: e.consensus_height().revision_number(),
						revision_height: e.consensus_height().revision_height(),
					},
				},
				header: Some(e.header().clone()),
			};
			Some(IbcEvent::UpdateClient(eve))
		},
		ibc_core_handler_types::events::IbcEvent::UpgradeClient(e) => {
			let eve = UpgradeClient(ClientAttributes {
				height: Height {
					revision_number: e.consensus_height().revision_number(),
					revision_height: e.consensus_height().revision_height(),
				},
				client_id: ClientId::from_str(e.client_id().as_str()).unwrap(),
				client_type: ClientType::from_str(e.client_type().as_str()).unwrap(),
				consensus_height: Height {
					revision_number: e.consensus_height().revision_number(),
					revision_height: e.consensus_height().revision_height(),
				},
			});
			Some(IbcEvent::UpgradeClient(eve))
		},
		ibc_core_handler_types::events::IbcEvent::ClientMisbehaviour(e) => {
			let eve = ClientMisbehaviour(ClientAttributes {
				height,
				client_id: ClientId::from_str(e.client_id().as_str()).unwrap(),
				client_type: ClientType::from_str(e.client_type().as_str()).unwrap(),
				consensus_height: height,
			});
			Some(IbcEvent::ClientMisbehaviour(eve))
		},
		ibc_core_handler_types::events::IbcEvent::OpenInitConnection(e) => {
			let eve = ConnOpenInit(ConnAttributes {
				height,
				client_id: ClientId::from_str(e.client_id_on_a().as_str()).unwrap(),
				counterparty_client_id: ClientId::from_str(e.client_id_on_b().as_str()).unwrap(),
				counterparty_connection_id: e
					.conn_id_on_b()
					.and_then(|conn| Some(ConnectionId::from_str(conn.as_str()).unwrap())),
				connection_id: Some(ConnectionId::from_str(e.conn_id_on_a().as_str()).unwrap()),
			});
			Some(IbcEvent::OpenInitConnection(eve))
		},
		ibc_core_handler_types::events::IbcEvent::OpenTryConnection(e) => {
			let eve = ConnOpenTry(ConnAttributes {
				height,
				client_id: ClientId::from_str(e.client_id_on_b().as_str()).unwrap(),
				counterparty_client_id: ClientId::from_str(e.client_id_on_b().as_str()).unwrap(),
				counterparty_connection_id: e.conn_id_on_a().and_then(|conn_id| {
					Some(ConnectionId::from_str(conn_id.clone().as_str()).unwrap())
				}),
				connection_id: Some(
					ConnectionId::from_str(e.conn_id_on_b().clone().as_str()).unwrap(),
				),
			});
			Some(IbcEvent::OpenTryConnection(eve))
		},
		ibc_core_handler_types::events::IbcEvent::OpenAckConnection(e) => {
			let eve = ConnOpenAck(ConnAttributes {
				height,
				client_id: ClientId::from_str(e.client_id_on_a().as_str()).unwrap(),
				counterparty_client_id: ClientId::from_str(e.client_id_on_b().as_str()).unwrap(),
				counterparty_connection_id: e
					.conn_id_on_b()
					.and_then(|conn| Some(ConnectionId::from_str(conn.as_str()).unwrap())),
				connection_id: Some(ConnectionId::from_str(e.conn_id_on_a().as_str()).unwrap()),
			});
			Some(IbcEvent::OpenAckConnection(eve))
		},
		ibc_core_handler_types::events::IbcEvent::OpenConfirmConnection(e) => {
			let eve = ConnOpenConfirm(ConnAttributes {
				height,
				client_id: ClientId::from_str(e.client_id_on_a().as_str()).unwrap(),
				counterparty_client_id: ClientId::from_str(e.client_id_on_b().as_str()).unwrap(),
				counterparty_connection_id: Some(
					ConnectionId::from_str(e.conn_id_on_b().as_str()).unwrap(),
				),
				connection_id: e
					.conn_id_on_a()
					.and_then(|conn| Some(ConnectionId::from_str(conn.as_str()).unwrap())),
			});
			Some(IbcEvent::OpenConfirmConnection(eve))
		},
		ibc_core_handler_types::events::IbcEvent::OpenInitChannel(e) => {
			let eve = ChanOpenInit {
				height,
				port_id: PortId::from_str(e.port_id_on_a().as_str()).unwrap(),
				channel_id: Some(ChannelId::from_str(e.chan_id_on_a().as_str()).unwrap()),
				connection_id: ConnectionId::from_str(e.conn_id_on_a().as_str()).unwrap(),
				counterparty_port_id: PortId::from_str(e.port_id_on_b().as_str()).unwrap(),
				counterparty_channel_id: None,
			};
			Some(IbcEvent::OpenInitChannel(eve))
		},
		ibc_core_handler_types::events::IbcEvent::OpenTryChannel(e) => {
			let eve = ChanOpenTry {
				height,
				port_id: PortId::from_str(e.port_id_on_a().as_str()).unwrap(),
				channel_id: Some(ChannelId::from_str(e.chan_id_on_a().as_str()).unwrap()),
				connection_id: ConnectionId::from_str(e.conn_id_on_b().as_str()).unwrap(),
				counterparty_port_id: PortId::from_str(e.port_id_on_b().as_str()).unwrap(),
				counterparty_channel_id: None,
			};
			Some(IbcEvent::OpenTryChannel(eve))
		},
		ibc_core_handler_types::events::IbcEvent::OpenAckChannel(e) => {
			let eve = ChanOpenAck {
				height,
				port_id: PortId::from_str(e.port_id_on_a().as_str()).unwrap(),
				channel_id: Some(ChannelId::from_str(e.chan_id_on_a().as_str()).unwrap()),
				connection_id: ConnectionId::from_str(e.conn_id_on_a().as_str()).unwrap(),
				counterparty_port_id: PortId::from_str(e.port_id_on_b().as_str()).unwrap(),
				counterparty_channel_id: None,
			};
			Some(IbcEvent::OpenAckChannel(eve))
		},
		ibc_core_handler_types::events::IbcEvent::OpenConfirmChannel(e) => {
			let eve = ChanOpenConfirm {
				height,
				port_id: PortId::from_str(e.port_id_on_a().as_str()).unwrap(),
				channel_id: Some(ChannelId::from_str(e.chan_id_on_a().as_str()).unwrap()),
				connection_id: ConnectionId::from_str(e.conn_id_on_b().as_str()).unwrap(),
				counterparty_port_id: PortId::from_str(e.port_id_on_b().as_str()).unwrap(),
				counterparty_channel_id: None,
			};
			Some(IbcEvent::OpenConfirmChannel(eve))
		},
		ibc_core_handler_types::events::IbcEvent::CloseInitChannel(e) => {
			let eve = ChanCloseInit {
				height,
				port_id: PortId::from_str(e.port_id_on_a().as_str()).unwrap(),
				channel_id: ChannelId::from_str(e.chan_id_on_a().as_str()).unwrap(),
				connection_id: ConnectionId::from_str(e.conn_id_on_a().as_str()).unwrap(),
				counterparty_port_id: PortId::from_str(e.port_id_on_b().as_str()).unwrap(),
				counterparty_channel_id: None,
			};
			Some(IbcEvent::CloseInitChannel(eve))
		},
		ibc_core_handler_types::events::IbcEvent::CloseConfirmChannel(e) => {
			let eve = ChanCloseConfirm {
				height,
				port_id: PortId::from_str(e.port_id_on_a().as_str()).unwrap(),
				channel_id: Some(ChannelId::from_str(e.chan_id_on_a().as_str()).unwrap()),
				connection_id: ConnectionId::from_str(e.conn_id_on_b().as_str()).unwrap(),
				counterparty_port_id: PortId::from_str(e.port_id_on_b().as_str()).unwrap(),
				counterparty_channel_id: None,
			};
			Some(IbcEvent::CloseConfirmChannel(eve))
		},
		ibc_core_handler_types::events::IbcEvent::SendPacket(e) => {
			let eve = SendPacket {
				height,
				packet: Packet {
					sequence: Sequence(e.seq_on_a().value()),
					source_port: PortId::from_str(e.port_id_on_a().as_str()).unwrap(),
					source_channel: ChannelId::from_str(e.chan_id_on_a().as_str()).unwrap(),
					destination_port: PortId::from_str(e.port_id_on_b().as_str()).unwrap(),
					destination_channel: ChannelId::from_str(e.chan_id_on_b().as_str()).unwrap(),
					data: e.packet_data().to_vec(),
					timeout_height: match e.timeout_height_on_b() {
						ibc_core_channel_types::timeout::TimeoutHeight::Never =>
							Height { revision_height: 0, revision_number: 0 },
						ibc_core_channel_types::timeout::TimeoutHeight::At(h) => Height {
							revision_height: h.revision_height(),
							revision_number: h.revision_number(),
						},
					},
					timeout_timestamp: Timestamp::from_nanoseconds(
						e.timeout_timestamp_on_b().nanoseconds(),
					)
					.unwrap(),
				},
			};
			Some(IbcEvent::SendPacket(eve))
		},
		ibc_core_handler_types::events::IbcEvent::ReceivePacket(e) => {
			let eve = ReceivePacket {
				height,
				packet: Packet {
					sequence: Sequence(e.seq_on_b().value()),
					source_port: PortId::from_str(e.port_id_on_a().as_str()).unwrap(),
					source_channel: ChannelId::from_str(e.chan_id_on_a().as_str()).unwrap(),
					destination_port: PortId::from_str(e.port_id_on_b().as_str()).unwrap(),
					destination_channel: ChannelId::from_str(e.chan_id_on_b().as_str()).unwrap(),
					data: e.packet_data().to_vec(),
					timeout_height: match e.timeout_height_on_b() {
						ibc_core_channel_types::timeout::TimeoutHeight::Never =>
							Height { revision_height: 0, revision_number: 0 },
						ibc_core_channel_types::timeout::TimeoutHeight::At(h) => Height {
							revision_height: h.revision_height(),
							revision_number: h.revision_number(),
						},
					},
					timeout_timestamp: Timestamp::from_nanoseconds(
						e.timeout_timestamp_on_b().nanoseconds(),
					)
					.unwrap(),
				},
			};
			Some(IbcEvent::ReceivePacket(eve))
		},
		ibc_core_handler_types::events::IbcEvent::WriteAcknowledgement(e) => {
			let eve = WriteAcknowledgement {
				height,
				packet: Packet {
					sequence: Sequence(e.seq_on_a().value()),
					source_port: PortId::from_str(e.port_id_on_a().as_str()).unwrap(),
					source_channel: ChannelId::from_str(e.chan_id_on_a().as_str()).unwrap(),
					destination_port: PortId::from_str(e.port_id_on_b().as_str()).unwrap(),
					destination_channel: ChannelId::from_str(e.chan_id_on_b().as_str()).unwrap(),
					data: e.packet_data().to_vec(),
					timeout_height: match e.timeout_height_on_b() {
						ibc_core_channel_types::timeout::TimeoutHeight::Never =>
							Height { revision_height: 0, revision_number: 0 },
						ibc_core_channel_types::timeout::TimeoutHeight::At(h) => Height {
							revision_height: h.revision_height(),
							revision_number: h.revision_number(),
						},
					},
					timeout_timestamp: Timestamp::from_nanoseconds(
						e.timeout_timestamp_on_b().nanoseconds(),
					)
					.unwrap(),
				},
				ack: e.acknowledgement().as_bytes().to_vec(),
			};
			Some(IbcEvent::WriteAcknowledgement(eve))
		},
		ibc_core_handler_types::events::IbcEvent::AcknowledgePacket(e) => {
			let eve = AcknowledgePacket {
				height,
				packet: Packet {
					sequence: Sequence(e.seq_on_a().value()),
					source_port: PortId::from_str(e.port_id_on_a().as_str()).unwrap(),
					source_channel: ChannelId::from_str(e.chan_id_on_a().as_str()).unwrap(),
					destination_port: PortId::from_str(e.port_id_on_b().as_str()).unwrap(),
					destination_channel: ChannelId::from_str(e.chan_id_on_b().as_str()).unwrap(),
					data: Vec::new(),
					timeout_height: match e.timeout_height_on_b() {
						ibc_core_channel_types::timeout::TimeoutHeight::Never =>
							Height { revision_height: 0, revision_number: 0 },
						ibc_core_channel_types::timeout::TimeoutHeight::At(h) => Height {
							revision_height: h.revision_height(),
							revision_number: h.revision_number(),
						},
					},
					timeout_timestamp: Timestamp::from_nanoseconds(
						e.timeout_timestamp_on_b().nanoseconds(),
					)
					.unwrap(),
				},
			};
			Some(IbcEvent::AcknowledgePacket(eve))
		},
		ibc_core_handler_types::events::IbcEvent::TimeoutPacket(e) => {
			let eve = TimeoutPacket {
				height,
				packet: Packet {
					sequence: Sequence(e.seq_on_a().value()),
					source_port: PortId::from_str(e.port_id_on_a().as_str()).unwrap(),
					source_channel: ChannelId::from_str(e.chan_id_on_a().as_str()).unwrap(),
					destination_port: PortId::from_str(e.port_id_on_b().as_str()).unwrap(),
					destination_channel: ChannelId::from_str(e.chan_id_on_b().as_str()).unwrap(),
					data: Vec::new(), // Not sure about this
					timeout_height: match e.timeout_height_on_b() {
						ibc_core_channel_types::timeout::TimeoutHeight::Never =>
							Height { revision_height: 0, revision_number: 0 },
						ibc_core_channel_types::timeout::TimeoutHeight::At(h) => Height {
							revision_height: h.revision_height(),
							revision_number: h.revision_number(),
						},
					},
					timeout_timestamp: Timestamp::from_nanoseconds(
						e.timeout_timestamp_on_b().nanoseconds(),
					)
					.unwrap(),
				},
			};
			Some(IbcEvent::TimeoutPacket(eve))
		},
		ibc_core_handler_types::events::IbcEvent::ChannelClosed(_) => None,
		ibc_core_handler_types::events::IbcEvent::Module(e) => {
			let attributes: Vec<ModuleEventAttribute> = e
				.attributes
				.iter()
				.map(|attr| ModuleEventAttribute {
					key: attr.clone().key,
					value: attr.clone().value,
				})
				.collect();
			let eve = ModuleEvent {
				kind: e.kind,
				module_name: ModuleId::from_str("transfer").unwrap(),
				attributes,
			};
			Some(IbcEvent::AppModule(eve))
		},
		ibc_core_handler_types::events::IbcEvent::Message(_) => None,
	}
}

pub fn get_ibc_events_from_logs(
	logs: Vec<String>,
) -> Vec<ibc_core_handler_types::events::IbcEvent> {
	let events = get_events_from_logs(logs);
	let events: Vec<ibc_core_handler_types::events::IbcEvent> = events
		.iter()
		.filter_map(|event| match event {
			solana_ibc::events::Event::IbcEvent(e) => Some(e.clone()),
			_ => None,
		})
		.collect();
	events
}

pub fn get_events_from_logs(logs: Vec<String>) -> Vec<solana_ibc::events::Event<'static>> {
	let serialized_events: Vec<&str> = logs
		.iter()
		.filter_map(|log| {
			if log.starts_with("Program data: ") {
				Some(log.strip_prefix("Program data: ").unwrap())
			} else {
				None
			}
		})
		.collect();
	let events: Vec<solana_ibc::events::Event> = serialized_events
		.iter()
		.map(|event| {
			let decoded_event = base64::prelude::BASE64_STANDARD.decode(event).unwrap();
			let decoded_event: solana_ibc::events::Event =
				borsh::BorshDeserialize::try_from_slice(&decoded_event).unwrap();
			decoded_event
		})
		.collect();
	events
}

pub async fn get_signatures_for_blockhash(
	rpc: RpcClient,
	program_id: Pubkey,
	blockhash: CryptoHash,
) -> Result<(Vec<(u16, Signature)>, BlockHeader), String> {
	sleep(Duration::from_secs(15));
	let transaction_signatures = rpc
		.get_signatures_for_address_with_config(
			&program_id,
			GetConfirmedSignaturesForAddress2Config { limit: Some(50), ..Default::default() },
		)
		.await
		.unwrap();
	let mut body = vec![];
	for sig in transaction_signatures {
		let signature = sig.signature.clone();
		let payload = Payload {
			jsonrpc: "2.0".to_string(),
			id: 1,
			method: "getTransaction".to_string(),
			params: vec![signature, "json".to_string()],
		};
		body.push(payload);
	}
	let transactions = tokio::task::spawn_blocking(move || {
		let transactions: std::result::Result<Vec<Response>, reqwest::Error> =
			reqwest::blocking::Client::new()
				.post(rpc.url())
				.json(&body)
				.send()
				.unwrap()
				.json();
		transactions
	})
	.await
	.unwrap();

	let mut signatures = Vec::new();
	let mut index = 0;
	for tx in transactions.unwrap() {
		let logs = match tx.result.transaction.meta.clone().unwrap().log_messages {
			solana_transaction_status::option_serializer::OptionSerializer::Some(e) => e,
			_ => Vec::new(),
		};
		let events = get_events_from_logs(logs);
		// Find block signed events with blockhash
		let block_header: Vec<Option<BlockHeader>> = events.iter().map(|event| match event {
			solana_ibc::events::Event::NewBlock(e) => {
				println!("This is new block event {:?}", e.block_header.0.block_height);
				let new_blockhash = e.block_header.0.calc_hash();
				if blockhash == new_blockhash {
					println!("New block event where it is true");
					return Some(e.block_header.0.clone());
				}
				None
			},
			solana_ibc::events::Event::BlockSigned(e) => {
				println!("This is block signed event {:?}", e.block_height);
				if e.block_hash == blockhash {
					println!("This is block signed in side blockhash");
					signatures.push((0_u16, Signature::from_bytes(&e.signature.to_vec()).unwrap()))
				};
				None
			},
			_ => None,
		}).collect();
		if let Some(header) = block_header.iter().find(|b| b.is_some()) {
			return Ok((signatures, header.clone().unwrap()))
		}	
	}
	Err("Couldnt find blocks".to_string())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
	jsonrpc: String,
	id: u64,
	method: String,
	params: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
	jsonrpc: String,
	id: u64,
	result: EncodedConfirmedTransactionWithStatusMeta,
}
