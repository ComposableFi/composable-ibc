use crate::{
	impls::{OFFCHAIN_RECV_PACKET_SEQS, OFFCHAIN_SEND_PACKET_SEQS},
	light_clients::{AnyClientState, AnyConsensusState},
	mock::*,
	routing::Context,
	Any, Config, ConsensusHeights, DenomToAssetId, MultiAddress, Pallet, Timeout, TransferParams,
	MODULE_ID,
};
use core::time::Duration;
use frame_support::{
	assert_ok,
	traits::{
		fungibles::{Inspect, Mutate},
		Len,
	},
};
use ibc::{
	applications::transfer::{packet::PacketData, Coin, PrefixedDenom, VERSION},
	core::{
		ics02_client::{
			client_state::ClientState,
			context::{ClientKeeper, ClientReader},
			height::Height,
			msgs::create_client::{MsgCreateAnyClient, TYPE_URL},
		},
		ics03_connection::{
			connection::{ConnectionEnd, Counterparty, State as ConnState},
			context::{ConnectionKeeper, ConnectionReader},
			msgs::conn_open_init,
			version::Version as ConnVersion,
		},
		ics04_channel::{
			channel::{ChannelEnd, Counterparty as ChanCounterParty, Order, State},
			context::ChannelKeeper,
			msgs::recv_packet::MsgRecvPacket,
			packet::Packet,
			Version as ChanVersion,
		},
		ics23_commitment::commitment::CommitmentPrefix,
		ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
	},
	mock::{
		client_state::{MockClientState, MockConsensusState},
		header::{MockClientMessage, MockHeader},
	},
	proofs::Proofs,
	signer::Signer,
	tx_msg::Msg,
};
use ibc_primitives::{get_channel_escrow_address, HandlerMessage, IbcHandler};
use sp_core::Pair;
use sp_runtime::{
	offchain::storage::StorageValueRef,
	traits::{AccountIdConversion, IdentifyAccount},
	AccountId32,
};
use std::{
	collections::{BTreeMap, BTreeSet},
	str::FromStr,
};
use tendermint_proto::Protobuf;

fn setup_client_and_consensus_state(port_id: PortId) {
	// Set up client state and consensus states
	let mock_client_state =
		MockClientState::new(MockClientMessage::from(MockHeader::new(Height::new(0, 1))));
	let mock_cs_state = MockConsensusState::new(MockHeader::new(Height::new(0, 1)));
	let client_id = ClientId::new(&mock_client_state.client_type(), 0).unwrap();
	let counterparty_client_id = ClientId::new(&mock_client_state.client_type(), 1).unwrap();
	let msg = MsgCreateAnyClient::<Context<Test>>::new(
		AnyClientState::Mock(mock_client_state),
		AnyConsensusState::Mock(mock_cs_state),
		Signer::from_str(MODULE_ID).unwrap(),
	)
	.unwrap()
	.encode_vec()
	.unwrap();
	let mut ctx = Context::<Test>::default();

	let msg = Any { type_url: TYPE_URL.to_string(), value: msg };
	assert_ok!(Ibc::deliver(RuntimeOrigin::signed(AccountId32::new([0; 32])), vec![msg]));

	let connection_id = ConnectionId::new(0);
	let commitment_prefix: CommitmentPrefix =
		<Test as Config>::PalletPrefix::get().to_vec().try_into().unwrap();
	let delay_period = core::time::Duration::from_nanos(0);
	let connection_counterparty =
		Counterparty::new(counterparty_client_id, Some(ConnectionId::new(1)), commitment_prefix);
	let connection_end = ConnectionEnd::new(
		ConnState::Open,
		client_id,
		connection_counterparty,
		vec![ConnVersion::default()],
		delay_period,
	);

	let counterparty = ChanCounterParty::new(port_id.clone(), Some(ChannelId::new(1)));
	let channel_end = ChannelEnd::new(
		State::Open,
		Order::Unordered,
		counterparty,
		vec![connection_id.clone()],
		ChanVersion::new(VERSION.to_string()),
	);
	let channel_id = ChannelId::new(0);
	ctx.store_connection(connection_id, &connection_end).unwrap();
	ctx.store_channel((port_id.clone(), channel_id), &channel_end).unwrap();
	ctx.store_next_sequence_send((port_id.clone(), channel_id), 1.into()).unwrap();
	ctx.store_next_sequence_recv((port_id, channel_id), 1.into()).unwrap()
}

// Create a client and initialize a connection
#[test]
fn initialize_connection() {
	new_test_ext().execute_with(|| {
		let mock_client_state =
			MockClientState::new(MockClientMessage::from(MockHeader::default()));
		let mock_cs_state = MockConsensusState::new(MockHeader::default());
		let client_id = ClientId::new(&mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new(&mock_client_state.client_type(), 1).unwrap();
		let msg = MsgCreateAnyClient::<Context<Test>>::new(
			AnyClientState::Mock(mock_client_state),
			AnyConsensusState::Mock(mock_cs_state),
			Signer::from_str(MODULE_ID).unwrap(),
		)
		.unwrap()
		.encode_vec()
		.unwrap();

		let commitment_prefix: CommitmentPrefix =
			<Test as Config>::PalletPrefix::get().to_vec().try_into().unwrap();

		let msg = Any { type_url: TYPE_URL.to_string(), value: msg };

		assert_ok!(Ibc::deliver(RuntimeOrigin::signed(AccountId32::new([0; 32])), vec![msg]));

		let value = conn_open_init::MsgConnectionOpenInit {
			client_id,
			counterparty: Counterparty::new(
				counterparty_client_id,
				Some(ConnectionId::new(1)),
				commitment_prefix,
			),
			version: Some(ConnVersion::default()),
			delay_period: Duration::from_nanos(1000),
			signer: Signer::from_str(MODULE_ID).unwrap(),
		};

		let msg = Any {
			type_url: conn_open_init::TYPE_URL.to_string(),
			value: value.encode_vec().unwrap(),
		};

		assert_ok!(Ibc::deliver(RuntimeOrigin::signed(AccountId32::new([0; 32])), vec![msg]));
	})
}

// try to initialize a connection below the MinimumConnectionDelay
#[test]
fn initialize_connection_with_low_delay() {
	new_test_ext().execute_with(|| {
		let ctx = Context::<Test>::new();

		let mock_client_state =
			MockClientState::new(MockClientMessage::from(MockHeader::default()));
		let mock_cs_state = MockConsensusState::new(MockHeader::default());
		let client_id = ClientId::new(&mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new(&mock_client_state.client_type(), 1).unwrap();
		let msg = MsgCreateAnyClient::<Context<Test>>::new(
			AnyClientState::Mock(mock_client_state),
			AnyConsensusState::Mock(mock_cs_state),
			Signer::from_str(MODULE_ID).unwrap(),
		)
		.unwrap()
		.encode_vec()
		.unwrap();

		let commitment_prefix: CommitmentPrefix =
			<Test as Config>::PalletPrefix::get().to_vec().try_into().unwrap();

		let msg = Any { type_url: TYPE_URL.to_string(), value: msg };

		assert_ok!(Ibc::deliver(RuntimeOrigin::signed(AccountId32::new([0; 32])), vec![msg]));

		let value = conn_open_init::MsgConnectionOpenInit {
			client_id,
			counterparty: Counterparty::new(
				counterparty_client_id,
				Some(ConnectionId::new(1)),
				commitment_prefix,
			),
			version: Some(ConnVersion::default()),
			delay_period: Duration::from_nanos(900),
			signer: Signer::from_str(MODULE_ID).unwrap(),
		};

		let msg = Any {
			type_url: conn_open_init::TYPE_URL.to_string(),
			value: value.encode_vec().unwrap(),
		};

		Ibc::deliver(RuntimeOrigin::signed(AccountId32::new([0; 32])), vec![msg]).unwrap();

		let result = ConnectionReader::connection_end(&ctx, &ConnectionId::new(0));

		assert!(result.is_err())
	})
}

const MILLIS: u128 = 1000000;
#[test]
fn send_transfer() {
	let mut ext = new_test_ext();
	ext.execute_with(|| {
		let pair = sp_core::sr25519::Pair::from_seed(b"12345678901234567890123456789012");
		let raw_user = ibc_primitives::runtime_interface::account_id_to_ss58(pair.public().0, 49);
		let ss58_address = String::from_utf8(raw_user).unwrap();
		setup_client_and_consensus_state(PortId::transfer());
		let balance = 100000 * MILLIS;
		let asset_id =
			<<Test as Config>::IbcDenomToAssetIdConversion as DenomToAssetId<Test>>::from_denom_to_asset_id(
				&"PICA".to_string(),
			)
			.unwrap();
		<<Test as Config>::Fungibles as Mutate<
			<Test as frame_system::Config>::AccountId,
		>>::mint_into(asset_id, &AccountId32::new([0; 32]), balance).unwrap();

		let timeout = Timeout::Offset { timestamp: Some(1000), height: Some(5) };

		Ibc::transfer(
			RuntimeOrigin::signed(AccountId32::new([0; 32])),
			TransferParams {
				to: MultiAddress::Raw(ss58_address.as_bytes().to_vec()),
				source_channel: 0,
				timeout,
			},
			asset_id,
			balance,
			None,
		)
		.unwrap();
	});

	ext.persist_offchain_overlay();

	ext.execute_with(|| {
		let channel_id = ChannelId::new(0);
		let port_id = PortId::transfer();
		let packet_info = Pallet::<Test>::get_send_packet_info(
			channel_id.to_string().as_bytes().to_vec(),
			port_id.as_bytes().to_vec(),
			vec![1],
		)
		.unwrap()
		.get(0)
		.unwrap()
		.clone();
		assert!(!packet_info.data.is_empty())
	})
}

#[test]
fn on_deliver_ics20_recv_packet() {
	let mut ext = new_test_ext();
	ext.execute_with(|| {
		// Create  a new account
		let pair = sp_core::sr25519::Pair::from_seed(b"12345678901234567890123456789012");
		let ss58_address_bytes =
			ibc_primitives::runtime_interface::account_id_to_ss58(pair.public().0, 49);
		let ss58_address = String::from_utf8(ss58_address_bytes).unwrap();
		frame_system::Pallet::<Test>::set_block_number(1u32);
		let asset_id =
			<<Test as Config>::IbcDenomToAssetIdConversion as DenomToAssetId<Test>>::from_denom_to_asset_id(
				&"PICA".to_string(),
			)
			.unwrap();
		setup_client_and_consensus_state(PortId::transfer());

		let channel_id = ChannelId::new(0);
		let balance = 100000 * MILLIS;

		// We are simulating a transfer back to the source chain

		let denom = "transfer/channel-1/PICA";
		let channel_escrow_address =
			get_channel_escrow_address(&PortId::transfer(), channel_id).unwrap();
		let channel_escrow_address =
			<Test as Config>::AccountIdConversion::try_from(channel_escrow_address)
				.map_err(|_| ())
				.unwrap();
		let channel_escrow_address = channel_escrow_address.into_account();

		// Endow escrow address with tokens
		<<Test as Config>::Fungibles as Mutate<
			<Test as frame_system::Config>::AccountId,
		>>::mint_into(asset_id, &channel_escrow_address, balance)
		.unwrap();

		let prefixed_denom = PrefixedDenom::from_str(denom).unwrap();
		let amt = 1000 * MILLIS;
		println!("Transferred Amount {}", amt);
		let coin = Coin {
			denom: prefixed_denom,
			amount: ibc::applications::transfer::Amount::from_str(&format!("{:?}", amt)).unwrap(),
		};
		let packet_data = PacketData {
			token: coin,
			sender: Signer::from_str("alice").unwrap(),
			receiver: Signer::from_str(&ss58_address).unwrap(),
			memo: "".to_string(),
		};

		let data = serde_json::to_vec(&packet_data).unwrap();
		let packet = Packet {
			sequence: 1u64.into(),
			source_port: PortId::transfer(),
			source_channel: ChannelId::new(1),
			destination_port: PortId::transfer(),
			destination_channel: ChannelId::new(0),
			data,
			timeout_height: Height::new(2000, 5),
			timeout_timestamp: ibc::timestamp::Timestamp::from_nanoseconds(
				1690894363u64.saturating_mul(1000000000),
			)
			.unwrap(),
		};

		let msg = MsgRecvPacket {
			packet,
			proofs: Proofs::new(
				vec![0u8; 32].try_into().unwrap(),
				None,
				None,
				None,
				Height::new(0, 1),
			)
			.unwrap(),
			signer: Signer::from_str(MODULE_ID).unwrap(),
		};

		let msg = Any { type_url: msg.type_url(), value: msg.encode_vec().unwrap() };

		let account_data = Assets::balance(2u128, AccountId32::new(pair.public().0));
		// Assert account balance before transfer
		assert_eq!(account_data, 0);
		Ibc::deliver(RuntimeOrigin::signed(AccountId32::new([0; 32])), vec![msg]).unwrap();

		let balance =
			<Assets as Inspect<AccountId>>::balance(2, &AccountId32::new(pair.public().0));
		let pallet_balance = <Assets as Inspect<AccountId>>::balance(
			2,
			&<Test as crate::ics20_fee::Config>::PalletId::get().into_account_truncating(),
		);
		let fee = <Test as crate::ics20_fee::Config>::ServiceCharge::get() * amt;
		assert_eq!(balance, amt - fee);
		assert_eq!(pallet_balance, fee)
	})
}

#[test]
fn should_fetch_recv_packet_with_acknowledgement() {
	let mut ext = new_test_ext();
	ext.execute_with(|| {
		frame_system::Pallet::<Test>::set_block_number(1u32);
		let channel_id = ChannelId::new(0);
		let port_id = PortId::transfer();

		let mut ctx = Context::<Test>::default();

		let mut channel_end = ChannelEnd::default();
		channel_end.state = State::Open;
		ctx.store_channel((port_id.clone(), channel_id), &channel_end).unwrap();
		let packet = Packet {
			sequence: 1u64.into(),
			source_port: port_id.clone(),
			source_channel: channel_id,
			destination_port: port_id.clone(),
			destination_channel: channel_id,
			data: "hello".as_bytes().to_vec(),
			timeout_height: Default::default(),
			timeout_timestamp: Default::default(),
		};

		ctx.store_recv_packet((port_id.clone(), channel_id, packet.sequence), packet.clone())
			.unwrap();
		let ack = "success".as_bytes().to_vec();
		Pallet::<Test>::handle_message(HandlerMessage::WriteAck { packet, ack }).unwrap();
	});

	ext.persist_offchain_overlay();

	ext.execute_with(|| {
		let ack = "success".as_bytes().to_vec();
		let channel_id = ChannelId::new(0);
		let port_id = PortId::transfer();
		let packet_info = Pallet::<Test>::get_recv_packet_info(
			channel_id.to_string().as_bytes().to_vec(),
			port_id.as_bytes().to_vec(),
			vec![1],
		)
		.unwrap()
		.get(0)
		.unwrap()
		.clone();
		assert_eq!(packet_info.ack, Some(ack))
	})
}

#[test]
fn should_cleanup_offchain_packets_correctly() {
	let mut ext = new_test_ext();
	let channel_id = ChannelId::new(0);
	let port_id = PortId::transfer();

	ext.execute_with(|| {
		// Add some packets offchain
		let channel_end = ChannelEnd::default();
		let mut ctx = Context::<Test>::default();
		ctx.store_channel((port_id.clone(), channel_id.clone()), &channel_end).unwrap();
		ctx.store_next_sequence_send((port_id.clone(), channel_id.clone()), 11.into())
			.unwrap();
		ctx.store_next_sequence_recv((port_id.clone(), channel_id.clone()), 11.into())
			.unwrap();
		// Store packets and commitments
		for i in 1..=10u64 {
			let packet = Packet {
				sequence: i.into(),
				source_port: port_id.clone(),
				source_channel: channel_id,
				destination_port: port_id.clone(),
				destination_channel: channel_id,
				data: "hello".as_bytes().to_vec(),
				timeout_height: Default::default(),
				timeout_timestamp: Default::default(),
			};
			ctx.store_send_packet((port_id.clone(), channel_id.clone(), i.into()), packet)
				.unwrap();

			// Store commitment for even numbers
			if i % 2 == 0 {
				ctx.store_packet_commitment(
					(port_id.clone(), channel_id.clone(), i.into()),
					"commitment".as_bytes().to_vec().into(),
				)
				.unwrap();
			}
		}

		// Store packet acknowledgements

		for i in 1..=10u64 {
			let packet = Packet {
				sequence: i.into(),
				source_port: port_id.clone(),
				source_channel: channel_id,
				destination_port: port_id.clone(),
				destination_channel: channel_id,
				data: "hello".as_bytes().to_vec(),
				timeout_height: Default::default(),
				timeout_timestamp: Default::default(),
			};
			ctx.store_recv_packet((port_id.clone(), channel_id.clone(), i.into()), packet)
				.unwrap();
			// Store ack for odd numbers
			if i % 2 != 0 {
				ctx.store_packet_acknowledgement(
					(port_id.clone(), channel_id.clone(), i.into()),
					"commitment".as_bytes().to_vec().into(),
				)
				.unwrap();
				Pallet::<Test>::store_raw_acknowledgement(
					(port_id.clone(), channel_id.clone(), i.into()),
					"acknowledgement".as_bytes().to_vec(),
				)
				.unwrap();
			}
		}
	});

	ext.persist_offchain_overlay();

	ext.execute_with(|| {
		Pallet::<Test>::packet_cleanup().unwrap();
	});

	ext.persist_offchain_overlay();

	ext.execute_with(|| {
		let pending_send_packet_seqs = StorageValueRef::persistent(OFFCHAIN_SEND_PACKET_SEQS);
		let pending_recv_packet_seqs = StorageValueRef::persistent(OFFCHAIN_RECV_PACKET_SEQS);
		let pending_send_sequences: BTreeMap<(Vec<u8>, Vec<u8>), (BTreeSet<u64>, u64)> =
			pending_send_packet_seqs.get::<_>().ok().flatten().unwrap();
		let pending_recv_sequences: BTreeMap<(Vec<u8>, Vec<u8>), (BTreeSet<u64>, u64)> =
			pending_recv_packet_seqs.get::<_>().ok().flatten().unwrap();

		let channel_id_bytes = channel_id.to_string().as_bytes().to_vec();
		let port_id_bytes = port_id.as_bytes().to_vec();

		let (send_seq_set, last_removed_send) = pending_send_sequences
			.get(&(port_id_bytes.clone(), channel_id_bytes.clone()))
			.map(|set| set.clone())
			.unwrap();

		let (recv_seq_set, last_removed_ack) = pending_recv_sequences
			.get(&(port_id_bytes, channel_id_bytes))
			.map(|set| set.clone())
			.unwrap();

		assert_eq!(send_seq_set, vec![2, 4, 6, 8, 10].into_iter().collect());

		assert_eq!(recv_seq_set, vec![1, 3, 5, 7, 9].into_iter().collect());

		assert_eq!(last_removed_send, 9);
		assert_eq!(last_removed_ack, 10);
		// Now let's prepare to remove pending sequences
		let mut ctx = Context::<Test>::default();
		for i in send_seq_set {
			ctx.delete_packet_commitment((port_id.clone(), channel_id.clone(), i.into()))
				.unwrap();
		}

		for i in recv_seq_set {
			ctx.delete_packet_acknowledgement((port_id.clone(), channel_id.clone(), i.into()))
				.unwrap();
		}
	});

	ext.persist_offchain_overlay();

	ext.execute_with(|| {
		Pallet::<Test>::packet_cleanup().unwrap();
	});

	ext.persist_offchain_overlay();

	ext.execute_with(|| {
		let pending_send_packet_seqs = StorageValueRef::persistent(OFFCHAIN_SEND_PACKET_SEQS);
		let pending_recv_packet_seqs = StorageValueRef::persistent(OFFCHAIN_RECV_PACKET_SEQS);
		let pending_send_sequences: BTreeMap<(Vec<u8>, Vec<u8>), (BTreeSet<u64>, u64)> =
			pending_send_packet_seqs.get::<_>().ok().flatten().unwrap();
		let pending_recv_sequences: BTreeMap<(Vec<u8>, Vec<u8>), (BTreeSet<u64>, u64)> =
			pending_recv_packet_seqs.get::<_>().ok().flatten().unwrap();

		let channel_id_bytes = channel_id.to_string().as_bytes().to_vec();
		let port_id_bytes = port_id.as_bytes().to_vec();

		let (send_seq_set, last_removed_send) = pending_send_sequences
			.get(&(port_id_bytes.clone(), channel_id_bytes.clone()))
			.map(|set| set.clone())
			.unwrap();

		let (recv_seq_set, last_removed_ack) = pending_recv_sequences
			.get(&(port_id_bytes, channel_id_bytes))
			.map(|set| set.clone())
			.unwrap();

		println!("{send_seq_set:?}");

		assert!(send_seq_set.is_empty());

		assert!(recv_seq_set.is_empty());

		assert_eq!(last_removed_send, 10);
		assert_eq!(last_removed_ack, 10);
	});
}

#[test]
fn test_next_and_previous_consensus_state_for_beefy_and_grandpa_clients() {
	new_test_ext().execute_with(|| {
		let client_id_beefy = ClientId::from_str("11-beefy-0").unwrap();
		let mut ctx = Context::<Test>::default();
		let mock_cs_state = MockConsensusState::new(MockHeader::default());
		ctx.store_consensus_state(
			client_id_beefy.clone(),
			Height::new(0, 10),
			AnyConsensusState::Mock(mock_cs_state.clone()),
		)
		.unwrap();
		// Should return None for Beefy and grandpa clients since we do not cache recent consensus
		// heights for beefy and grandpa
		assert!(ctx
			.prev_consensus_state(&client_id_beefy, Height::new(0, 15))
			.unwrap()
			.is_none());
		assert!(ctx.next_consensus_state(&client_id_beefy, Height::new(0, 5)).unwrap().is_none());

		let client_id_grandpa = ClientId::from_str("10-grandpa-0").unwrap();
		ctx.store_consensus_state(
			client_id_grandpa.clone(),
			Height::new(0, 10),
			AnyConsensusState::Mock(mock_cs_state.clone()),
		)
		.unwrap();
		assert!(ctx
			.prev_consensus_state(&client_id_grandpa, Height::new(0, 15))
			.unwrap()
			.is_none());
		assert!(ctx
			.next_consensus_state(&client_id_grandpa, Height::new(0, 5))
			.unwrap()
			.is_none());
	})
}

#[test]
fn test_next_and_previous_consensus_state_for_other_client_types() {
	new_test_ext().execute_with(|| {
		let client_id = ClientId::from_str("07-tendermint-0").unwrap();
		let mut ctx = Context::<Test>::default();
		let mock_cs_state = MockConsensusState::new(MockHeader::default());
		// lets store 512 consensus states, only the 256 most recent should remain after the loop
		for i in 1..=512u64 {
			ctx.store_consensus_state(
				client_id.clone(),
				Height::new(0, i),
				AnyConsensusState::Mock(mock_cs_state.clone()),
			)
			.unwrap();
		}

		let stored_heights = ConsensusHeights::<Test>::get(client_id.as_bytes().to_vec());

		assert_eq!(stored_heights.len(), 256);
		assert_eq!(stored_heights.iter().rev().next(), Some(&Height::new(0, 512)));

		// Should return None for Beefy and grandpa clients since we do not cache recent consensus
		// heights for beefy and grandpa
		assert!(ctx.prev_consensus_state(&client_id, Height::new(0, 300)).unwrap().is_some());
		assert!(ctx.next_consensus_state(&client_id, Height::new(0, 400)).unwrap().is_some());
	})
}
