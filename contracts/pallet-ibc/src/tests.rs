// Copyright 2022 ComposableFi
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{
	light_clients::{AnyClientState, AnyConsensusState},
	mock::*,
	routing::Context,
	Any, Config, DenomToAssetId, MultiAddress, Pallet, PalletParams, Timeout, TransferParams,
	MODULE_ID,
};
use core::time::Duration;
use frame_support::{
	assert_ok,
	traits::fungibles::{Inspect, Mutate},
};
use ibc::{
	applications::transfer::{packet::PacketData, Coin, PrefixedDenom, VERSION},
	core::{
		ics02_client::{
			client_state::ClientState,
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
use ibc_primitives::{get_channel_escrow_address, IbcHandler};
use sp_core::Pair;
use sp_runtime::{traits::IdentifyAccount, AccountId32};
use std::str::FromStr;
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
	.encode_vec();
	let mut ctx = Context::<Test>::default();

	let msg = Any { type_url: TYPE_URL.to_string().as_bytes().to_vec(), value: msg };
	assert_ok!(Ibc::deliver(Origin::signed(AccountId32::new([0; 32])), vec![msg]));

	let connection_id = ConnectionId::new(0);
	let commitment_prefix: CommitmentPrefix =
		<Test as Config>::PALLET_PREFIX.to_vec().try_into().unwrap();
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
		.encode_vec();

		let commitment_prefix: CommitmentPrefix =
			<Test as Config>::PALLET_PREFIX.to_vec().try_into().unwrap();

		let msg = Any { type_url: TYPE_URL.to_string().as_bytes().to_vec(), value: msg };

		assert_ok!(Ibc::deliver(Origin::signed(AccountId32::new([0; 32])), vec![msg]));

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
			type_url: conn_open_init::TYPE_URL.as_bytes().to_vec(),
			value: value.encode_vec(),
		};

		assert_ok!(Ibc::deliver(Origin::signed(AccountId32::new([0; 32])), vec![msg]));
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
		.encode_vec();

		let commitment_prefix: CommitmentPrefix =
			<Test as Config>::PALLET_PREFIX.to_vec().try_into().unwrap();

		let msg = Any { type_url: TYPE_URL.to_string().as_bytes().to_vec(), value: msg };

		assert_ok!(Ibc::deliver(Origin::signed(AccountId32::new([0; 32])), vec![msg]));

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
			type_url: conn_open_init::TYPE_URL.as_bytes().to_vec(),
			value: value.encode_vec(),
		};

		Ibc::deliver(Origin::signed(AccountId32::new([0; 32])), vec![msg]).unwrap();

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
		let raw_user =
			ibc_primitives::runtime_interface::account_id_to_ss58(pair.public().0).unwrap();
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

		Ibc::set_params(Origin::root(), PalletParams { send_enabled: true, receive_enabled: true })
			.unwrap();

		let timeout = Timeout::Offset { timestamp: Some(1000), height: Some(5) };

		Ibc::transfer(
			Origin::signed(AccountId32::new([0; 32])),
			TransferParams {
				to: MultiAddress::Raw(ss58_address.as_bytes().to_vec()),
				source_channel: 0,
				timeout,
			},
			asset_id,
			balance,
		)
		.unwrap();
	})
}

#[test]
fn on_deliver_ics20_recv_packet() {
	let mut ext = new_test_ext();
	ext.execute_with(|| {
		// Create  a new account
		let pair = sp_core::sr25519::Pair::from_seed(b"12345678901234567890123456789012");
		let ss58_address_bytes =
			ibc_primitives::runtime_interface::account_id_to_ss58(pair.public().0).unwrap();
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

		Ibc::set_params(Origin::root(), PalletParams { send_enabled: true, receive_enabled: true })
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

		let msg = Any { type_url: msg.type_url().as_bytes().to_vec(), value: msg.encode_vec() };

		let account_data = Assets::balance(2u128, AccountId32::new(pair.public().0));
		// Assert account balance before transfer
		assert_eq!(account_data, 0);
		Ibc::deliver(Origin::signed(AccountId32::new([0; 32])), vec![msg]).unwrap();

		let balance =
			<Assets as Inspect<AccountId>>::balance(2, &AccountId32::new(pair.public().0));
		assert_eq!(balance, amt.into())
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

		let channel_end = ChannelEnd::default();
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
		Pallet::<Test>::write_acknowledgement(&packet, ack.clone()).unwrap();

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
	});
}
