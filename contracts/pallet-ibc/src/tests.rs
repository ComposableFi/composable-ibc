use crate::{
	ics20_fee::FlatFeeConverter,
	ics23::client_states::ClientStates,
	light_clients::{AnyClientState, AnyClientStateV1, AnyConsensusState},
	mock::*,
	routing::Context,
	Any, Config, ConsensusHeights, DenomToAssetId, MultiAddress, Pallet, PendingRecvPacketSeqs,
	PendingSendPacketSeqs, Timeout, TransferParams, MODULE_ID,
};
use core::time::Duration;
use frame_support::{
	assert_ok,
	traits::{
		fungibles::{Inspect, Mutate},
		Currency, Hooks, Len,
	},
	weights::Weight,
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
			context::{ChannelKeeper, ChannelReader},
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
	timestamp::Timestamp,
	tx_msg::Msg,
};
use ibc_primitives::{get_channel_escrow_address, HandlerMessage, IbcHandler};
use ics08_wasm::client_state::ClientState as WasmClientState;
use sp_core::Pair;
use sp_runtime::{
	traits::{Bounded, IdentifyAccount},
	AccountId32,
};
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
	let balance = 100000 * MILLIS;
	ext.execute_with(|| {
		let pair = sp_core::sr25519::Pair::from_seed(b"12345678901234567890123456789012");
		let ss58_address =
			ibc_primitives::runtime_interface::account_id_to_ss58(pair.public().0, 49);
		setup_client_and_consensus_state(PortId::transfer());
		let asset_id =
			<<Test as Config>::IbcDenomToAssetIdConversion as DenomToAssetId<Test>>::from_denom_to_asset_id(
				&"PICA".to_string(),
			)
			.unwrap();
		let _ = <<Test as Config>::NativeCurrency as Currency<
			<Test as frame_system::Config>::AccountId,
		>>::deposit_creating(&AccountId32::new([0; 32]), balance.into());

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
		assert!(!packet_info.data.is_empty());

		let packet_data: PacketData = serde_json::from_slice(packet_info.data.as_slice()).unwrap();
		let send_amount = packet_data.token.amount.as_u256().as_u128();
		let fee = <Test as crate::ics20_fee::Config>::ServiceChargeIn::get() * balance;
		assert_eq!(send_amount, balance - fee);
	})
}

#[test]
fn send_transfer_no_fee_feeless_channels() {
	let mut ext = new_test_ext();
	let balance = 100000 * MILLIS;
	ext.execute_with(|| {
		let pair = sp_core::sr25519::Pair::from_seed(b"12345678901234567890123456789012");
		let ss58_address =
			ibc_primitives::runtime_interface::account_id_to_ss58(pair.public().0, 49);
		setup_client_and_consensus_state(PortId::transfer());
		let asset_id =
			<<Test as Config>::IbcDenomToAssetIdConversion as DenomToAssetId<Test>>::from_denom_to_asset_id(
				&"PICA".to_string(),
			)
			.unwrap();
		let _ = <<Test as Config>::NativeCurrency as Currency<
			<Test as frame_system::Config>::AccountId,
		>>::deposit_creating(&AccountId32::new([0; 32]), balance.into());

		let timeout = Timeout::Offset { timestamp: Some(1000), height: Some(5) };

		let ctx = Context::<Test>::default();
		let channel_end = ctx
			.channel_end(&(PortId::transfer(), ChannelId::new(0)))
			.expect("expect source_channel unwrap");
		let destination_channel = channel_end.counterparty().channel_id.unwrap();
		//feeless channels
		let _ = Ibc::add_channels_to_feeless_channel_list(
			RuntimeOrigin::root(),
			0,
			destination_channel.sequence(),
		)
		.expect("expect add channels to feeless list");

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
		assert!(!packet_info.data.is_empty());

		let packet_data: PacketData = serde_json::from_slice(packet_info.data.as_slice()).unwrap();
		let send_amount = packet_data.token.amount.as_u256().as_u128();
		assert_eq!(send_amount, balance);
	})
}

#[test]
fn on_deliver_ics20_recv_packet() {
	let mut ext = new_test_ext();
	ext.execute_with(|| {
		// Create  a new account
		let pair = sp_core::sr25519::Pair::from_seed(b"12345678901234567890123456789012");
		let ss58_address =
			ibc_primitives::runtime_interface::account_id_to_ss58(pair.public().0, 49);
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
		let _ = <<Test as Config>::NativeCurrency as Currency<
			<Test as frame_system::Config>::AccountId,
		>>::deposit_creating(&channel_escrow_address, balance);

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

		let account_data = Assets::balance(asset_id, AccountId32::new(pair.public().0));
		// Assert account balance before transfer
		assert_eq!(account_data, 0);
		Ibc::deliver(RuntimeOrigin::signed(AccountId32::new([0; 32])), vec![msg]).unwrap();

		let balance = <<Test as Config>::NativeCurrency as Currency<
			<Test as frame_system::Config>::AccountId,
		>>::free_balance(&AccountId32::new(pair.public().0));

		let pallet_balance =
			<<Test as Config>::NativeCurrency as Currency<
				<Test as frame_system::Config>::AccountId,
			>>::free_balance(&<Test as crate::Config>::FeeAccount::get().into_account());
		let fee = <Test as crate::ics20_fee::Config>::ServiceChargeIn::get() * amt;
		assert_eq!(balance, amt - fee);
		assert_eq!(pallet_balance, fee)
	})
}

#[test]
fn on_deliver_ics20_recv_packet_with_flat_fee() {
	let mut ext = new_test_ext();
	ext.execute_with(|| {
		// Create  a new account
		let pair = sp_core::sr25519::Pair::from_seed(b"12345678901234567890123456789012");
		let ss58_address =
			ibc_primitives::runtime_interface::account_id_to_ss58(pair.public().0, 49);
		frame_system::Pallet::<Test>::set_block_number(1u32);
		let asset_id =
			<<Test as Config>::IbcDenomToAssetIdConversion as DenomToAssetId<Test>>::from_denom_to_asset_id(
				&"PICAFLATFEE".to_string(),
			)
			.unwrap();
		setup_client_and_consensus_state(PortId::transfer());

		let channel_id = ChannelId::new(0);
		let balance = 100000 * MILLIS;

		// We are simulating a transfer back to the source chain

		let denom = "transfer/channel-1/PICAFLATFEE";
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

		let account_data = Assets::balance(asset_id, AccountId32::new(pair.public().0));
		// Assert account balance before transfer
		assert_eq!(account_data, 0);
		Ibc::deliver(RuntimeOrigin::signed(AccountId32::new([0; 32])), vec![msg]).unwrap();

		let balance =
			<Assets as Inspect<AccountId>>::balance(asset_id, &AccountId32::new(pair.public().0));
		let pallet_balance = <Assets as Inspect<AccountId>>::balance(
			asset_id,
			&<Test as crate::Config>::FeeAccount::get().into_account(),
		);
		let fee_asset_id = <Test as crate::Config>::FlatFeeAssetId::get();
		let fee =
			<Test as crate::Config>::FlatFeeConverter::get_flat_fee(asset_id, fee_asset_id, amt)
				.unwrap_or_default();
		assert_eq!(balance, amt - fee);
		assert_eq!(pallet_balance, fee)
	})
}

#[test]
fn on_deliver_ics20_recv_packet_transfered_amount_less_then_flat_fee() {
	let mut ext = new_test_ext();
	ext.execute_with(|| {
		// Create  a new account
		let pair = sp_core::sr25519::Pair::from_seed(b"12345678901234567890123456789012");
		let ss58_address =
			ibc_primitives::runtime_interface::account_id_to_ss58(pair.public().0, 49);
		frame_system::Pallet::<Test>::set_block_number(1u32);
		let asset_id =
			<<Test as Config>::IbcDenomToAssetIdConversion as DenomToAssetId<Test>>::from_denom_to_asset_id(
				&"PICAFLATFEE".to_string(),
			)
			.unwrap();
		setup_client_and_consensus_state(PortId::transfer());

		let channel_id = ChannelId::new(0);
		let balance = 100000 * MILLIS;

		// We are simulating a transfer back to the source chain

		let denom = "transfer/channel-1/PICAFLATFEE";
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
		let usdt_fee_amount = 1000 * MILLIS;
		let fee_asset_id = <Test as crate::Config>::FlatFeeAssetId::get();
		let amt = <Test as crate::Config>::FlatFeeConverter::get_flat_fee(
			asset_id,
			fee_asset_id,
			usdt_fee_amount,
		)
		.unwrap_or_default();
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

		let account_data = Assets::balance(asset_id, AccountId32::new(pair.public().0));
		// Assert account balance before transfer
		assert_eq!(account_data, 0);
		Ibc::deliver(RuntimeOrigin::signed(AccountId32::new([0; 32])), vec![msg]).unwrap();

		let balance =
			<Assets as Inspect<AccountId>>::balance(asset_id, &AccountId32::new(pair.public().0));
		let pallet_balance = <Assets as Inspect<AccountId>>::balance(
			asset_id,
			&<Test as crate::Config>::FeeAccount::get().into_account(),
		);
		let fee = <Test as crate::Config>::FlatFeeConverter::get_flat_fee(
			asset_id,
			fee_asset_id,
			usdt_fee_amount,
		)
		.unwrap_or_default();
		let actual_fee = fee.min(amt);
		assert_eq!(balance, 0);
		assert_eq!(balance, amt - actual_fee);
		assert_eq!(pallet_balance, actual_fee);
	})
}

#[test]
fn on_deliver_ics20_recv_packet_should_not_double_spend() {
	let mut ext = new_test_ext();
	ext.execute_with(|| {
		// Create  a new account
		let pair = sp_core::sr25519::Pair::from_seed(b"12345678901234567890123456789012");
		let ss58_address =
			ibc_primitives::runtime_interface::account_id_to_ss58(pair.public().0, 49);
		frame_system::Pallet::<Test>::set_block_number(1u32);
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
		let _ = <<Test as Config>::NativeCurrency as Currency<
			<Test as frame_system::Config>::AccountId,
		>>::deposit_creating(&channel_escrow_address, balance);

		let prefixed_denom = PrefixedDenom::from_str(denom).unwrap();
		let amt = MILLIS / 100;
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
		let fee_amt = <Test as crate::ics20_fee::Config>::ServiceChargeIn::get() * amt;

		let account_balance = <<Test as Config>::NativeCurrency as Currency<
			<Test as frame_system::Config>::AccountId,
		>>::free_balance(&AccountId32::new(pair.public().0));
		// Assert account balance before transfer
		assert_eq!(account_balance, 0);
		Ibc::deliver(RuntimeOrigin::signed(AccountId32::new([0; 32])), vec![msg.clone()]).unwrap();

		let account_balance = <<Test as Config>::NativeCurrency as Currency<
			<Test as frame_system::Config>::AccountId,
		>>::free_balance(&AccountId32::new(pair.public().0));
		// Assert account balance after transfer
		assert_eq!(account_balance, amt - fee_amt);

		let balance = <<Test as Config>::NativeCurrency as Currency<
			<Test as frame_system::Config>::AccountId,
		>>::free_balance(&AccountId32::new(pair.public().0));
		let pallet_balance =
			<<Test as Config>::NativeCurrency as Currency<
				<Test as frame_system::Config>::AccountId,
			>>::free_balance(&<Test as crate::Config>::FeeAccount::get().into_account());
		// fee is less than ExistentialDeposit, so it is not deducted
		assert_eq!(balance, amt - fee_amt);
		assert_eq!(pallet_balance, fee_amt);

		// try sending the same packet again
		Ibc::deliver(RuntimeOrigin::signed(AccountId32::new([0; 32])), vec![msg]).unwrap();

		let account_data = <<Test as Config>::NativeCurrency as Currency<
			<Test as frame_system::Config>::AccountId,
		>>::free_balance(&AccountId32::new(pair.public().0));
		// Assert account balance after transfer
		assert_eq!(account_data, amt - fee_amt);

		let balance = <<Test as Config>::NativeCurrency as Currency<
			<Test as frame_system::Config>::AccountId,
		>>::free_balance(&AccountId32::new(pair.public().0));
		let pallet_balance =
			<<Test as Config>::NativeCurrency as Currency<
				<Test as frame_system::Config>::AccountId,
			>>::free_balance(&<Test as crate::Config>::FeeAccount::get().into_account());
		// fee is less than ExistentialDeposit, so it is not deducted
		assert_eq!(balance, amt - fee_amt);
		assert_eq!(pallet_balance, fee_amt);
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
	let cleanup_period = CleanUpPacketsPeriod::get();

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

	ext.execute_with(|| {
		// should not cleanup withing the period
		for i in 1..cleanup_period {
			Pallet::<Test>::on_idle(i, Weight::max_value());
		}
	});

	ext.execute_with(|| {
		let channel_id_bytes = channel_id.to_string().as_bytes().to_vec();
		let port_id_bytes = port_id.as_bytes().to_vec();

		let (send_seq_set, _) =
			PendingSendPacketSeqs::<Test>::get(&(port_id_bytes.clone(), channel_id_bytes.clone()));

		let (recv_seq_set, _) =
			PendingRecvPacketSeqs::<Test>::get(&(port_id_bytes, channel_id_bytes));

		assert_eq!(send_seq_set, Default::default());

		assert_eq!(recv_seq_set, Default::default());
	});

	ext.execute_with(|| {
		Pallet::<Test>::on_idle(cleanup_period, Weight::max_value());
	});

	ext.execute_with(|| {
		let channel_id_bytes = channel_id.to_string().as_bytes().to_vec();
		let port_id_bytes = port_id.as_bytes().to_vec();

		let (send_seq_set, last_removed_send) =
			PendingSendPacketSeqs::<Test>::get(&(port_id_bytes.clone(), channel_id_bytes.clone()));

		let (recv_seq_set, last_removed_ack) =
			PendingRecvPacketSeqs::<Test>::get(&(port_id_bytes, channel_id_bytes));

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

	ext.execute_with(|| {
		Pallet::<Test>::on_idle(cleanup_period, Weight::max_value());
	});

	ext.execute_with(|| {
		let channel_id_bytes = channel_id.to_string().as_bytes().to_vec();
		let port_id_bytes = port_id.as_bytes().to_vec();

		let (send_seq_set, last_removed_send) =
			PendingSendPacketSeqs::<Test>::get(&(port_id_bytes.clone(), channel_id_bytes.clone()));

		let (recv_seq_set, last_removed_ack) =
			PendingRecvPacketSeqs::<Test>::get(&(port_id_bytes, channel_id_bytes));

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

#[test]
fn test_migration_to_grandpa_v2() {
	new_test_ext().execute_with(|| {
		let client_id = ClientId::from_str("10-grandpa-0").unwrap();
		let wasm_client_id = ClientId::from_str("08-wasm-0").unwrap();
		let ctx = Context::<Test>::default();

		// this is a real state queried from the node
		let data = hex::decode("0a282f6962632e6c69676874636c69656e74732e6772616e6470612e76312e436c69656e74537461746512c9580a206fa8cbe41be17654e5c5a88098ba35f04798f3f671bddb67d4917b7b378a4d1110d0b9980718f10a280230e30f38c9ce6d42240a20db30c718a48a844d7bffad2dce4859812e999c7d65d496ceecfe803fb556c46d100142240a20fff437ff18629bf1490e5c9b3ec6f1515d46bb9b2aeaa6e39e36611f2479b50d100142240a2047f940bc985355be7187c709ded2b689b66d69e61e293a507f1d4f90bdbeceee100142240a20525c0460e98f5cda7d20e67ea1e27b3ca471760559baa73dcf77b2e163b95718100142240a204fc65112cfbcd612a79a752ecce93465965f0415428f049f0430360426becf2f100142240a20f98b21ab797ac19b101f8e446865564901dff29b4e0bbc5badcae7ac88dab7d8100142240a20b61e9b13e849779be8f5378a670d61991875d8e091f359384d7be73d02d50693100142240a2070ba6e5985990ba1b1392713236f1b50df750f10f744cc6eb95fa7c5cfbc6876100142240a2057ae0c85ebaf333a6ec3251f577cab910cce072f238d1e50046322b83bbc0dd5100142240a20bdae4bc60b03f8fbc14299f0449301ef7f423a4cd15c6f6a35ea6a44fc5bcb0f100142240a2072c09c9e2dc9ab20a65cf70e951352f554442514ba2ac7063966ea7bbe3e6cf0100142240a200139b946b46bee4c61be50c986f2a9ef2e76917644e2ecd842c59d61a94ee612100142240a2076aa25ab781816fdfabc9051149c3db958bc09d2e8f0e10a4d91e0df8c79ec9b100142240a2007937ef39e4d18ffbf6d7343df62fcf74378aed34a9d3483bbfea77046d6f023100142240a2053352e16d2a27fb3525d5d8750d6ad7f46b303a45e434a409ac727d5cb80432b100142240a208c54298c6f8704498112ecb24f64f7a28ed2289ec573620b0d1e5a9a44f5fa78100142240a2041ef4a31eb7dc1e01f4630604e1908e644d7cdee3f66a60f98d6d59605326f8b100142240a20c4ab2451500e321f187140d3aeed09d5cdf67f09d9997b966c86dd74714233db100142240a20049cb09a86d544249f9e11cf6c348bc0561600343ebee89d1c90bbef3d13e517100142240a20d04d64a07f144638f20f081de122b88fed0cc8bda57e4810e7afb38036dd8bbf100142240a20f26945a8a64032a1defa76e720a99649125b55751b6088205e7acab901de670b100142240a20684768723f0efc2001562c3b2156b987601ae9431f8aaa6f4199649cba487aea100142240a209b94bc45bf79bb33f70430c82880456cd7bd5fb3ea131c1b26d493d01c5fe710100142240a20d7e8ce7ff46c836473acb0c3016feab586b5c4a6b529f7cedef01c6727dbbcf6100142240a20ce8d085d4b0a9502a680280ecc162bd33533a5ebada62f3be19427542189cb5a100142240a206f4855725ebb9eced957bae43ffa7d30657ce7d06b251ce20e2218bcf929e405100142240a20b046cf7d24d49b720b28dd2476b36c157262208fb065c077b1b94db1e9ffa605100142240a20f2f0f84855c6bce35c42d537fb4022662272203cc4985767ad4ef8ef1bbb7165100142240a20ff0bf39c82ed573d48585448b0fc19ba3f6203806cb6b4a230848892c097a26f100142240a209c27510363c46fb84c22765901302fa26af266da5a22099368fce05562ce97eb100142240a200ce4d93e25fa5302f4b059456f680047f87be428ae88579e0b862502b5ceed60100142240a203478e6a5b64e3aaaa6795e6536a12728a20af7a8d44c75f5e981c6d2aee45177100142240a203b252801bf8a2f4797699a0d95e8fad8c8fb42cf6f6a33f4555bde1c7fefd449100142240a20c1c1ff5f66fefb5c0849e3b2333411d1f7e9dd4888600e334e7a33623f15fa36100142240a202acce562c859bcb760e355961fee5bd27b9b52a83ded5d7a340ad21f90935be9100142240a20fe240e80ff8a48a77947b394979cf361017cd906ddaab637d72cf72e729d79bf100142240a2006854c73f5703242d31d2576ac2297c93f0b5dc16ade0097c59d5dc0faa43c55100142240a208aabd4f6d7ecec7a749c83fc7be33f3b4818cca6f93a4927b4964900df9454e5100142240a20b9186b95c90d2d00a31e7c68066bd37d73408271762604e3608e2f2c983f8309100142240a203948bfd09bbbc2fe6ac5dfe53146e32df50828eecf7b2b18cdcff8dc385cbd75100142240a201ba602a293ccdb7a77042fb486c6e010d48b13dff2bef0a26c53637312aac7f7100142240a20f5260c9ada18534d79be08836484429e686985c8241e113307792aa1255b1cb1100142240a20e730007e26ea493e96a65b30d278e92ef85444e61fee3606729a5e699b74f110100142240a20d4d8cfaa23b3a1a89a7f6a90cdbfdea15c87ad2ca4ae77deb3389406ad3feffe100142240a20dc1427d14e83926d22f6be25d1bd39cbc82a194f5e9373ecae644f156ae9c62e100142240a200339088379600f5bd507277eb894aba34d2a8f27ce1964f41e60ef2a4142dc6e100142240a2003d105a30087d96b5f0684f6ded76f826b01dab61e4136e1d851a24f0088b5ed100142240a2020d4e595c50bb9558dad9be6a8784492bbbfc754b9c5fae17edf4f8a84e8b471100142240a206ba031ff146af5199eeb335a4a9c2fd487685a489e1a154ba1d012e965fdd5ba100142240a20608f61824d3ae16d443cd0fec15c18590c890d4105e9402cad8743fb423e52b4100142240a20a8bec5971a19af91454989be32455a1302dd8169636cd9116910260f6a5bdcf2100142240a20fe95aa161a01b3473ed9dc7f4e2219528d8be86530bcfa7095086ed61e3d7a03100142240a200ccee96768f9b1d29d5bd9e63afd2be78a90cc89d19653ff1910cb36000b4853100142240a20818a546c630b881c2161588866965649678cda0f4110cccc3533d0f20e5e4120100142240a20088f8736e1cf2ea3d102f0a96ccf51222b8aa1f93d8e42947892f4395cb5477c100142240a20cbfefdf389bc341a7e17139f61146f9fb3a9d7bb84fe93c1f771eafd4d4d9d46100142240a20c3ff25a1743a9df92af4ccd9a7aed5cbc90f64fd538c3df0a9539128f5965267100142240a204e3fb8c16ad624c1852777456805e5352d0221269de66949c95da4076206b35a100142240a207ad55381f9de00220678c95a19489640fc61331e2a40d927e3d833555575c6fc100142240a20cb92ffeace78dbafe6fbf275741b4b38657fb81590712aa0bca7877931f6ad39100142240a207ac1f611f90df5fb222924b2c6683062db933d6083922c4a17ce29f21298a768100142240a2007727680e6caac998c03e3702c7f84e558b592dd76ab2a49cdd3dae16e7e853f100142240a200a0182091a12fec9482a149b62e1b9c15aa5dfc7b013f602ce1e68c636c88749100142240a20c6b03ba3484a8eb6518858d525b5a64f5b9db650bcc7c0df788ade370714d222100142240a2059bba625d971d505a9c7d7c2f3ad69203e69e3e5e1fc1e4905fae7703fa19e03100142240a20e9cd3df644306d83a34a04682f264d8db5da92f0d48a9f8fa7a48caefa65a25b100142240a20ba95f5d0e987305bb794dfeae69d7dd6a7987aa66cd7c9d42aca15e9160ec6a1100142240a2057d2c2934f6896f927b91431398ee0ee820f35d13959ffdb77961c86893c7fea100142240a2054efa280ecec9c8ce569b293e92733e2fd4a884a551241d92b042be850cca5b9100142240a2016278af083f6d261762f8a97b6a09ff83e2f5927ce26c2e48f3c7e771dc7e695100142240a206a4dd2a5375c3ed37eca0a5ecda6ad6d7d1dc9818a588129c9529b41706c28b5100142240a20c49e3cd2c701bb845963c2870dcca12ed070c3f67ffc20144327a93aa6e896ec100142240a208041ec7547fadc33aa19a785dbd1a7058b0147f2d9736b9dc4dbc6ce454bd7ef100142240a2082a1428bcd525a2e77b8f5fb89c73e5fe1261e96e93b383dd2d66767c855c8f0100142240a202dad5b2212ee688f2eeb9ca1fb6a90574f006dc1c6680ac3a8523363a248940b100142240a20be0a28e5615d9ce3b866b5cf0babf6a7f0e6de06119463382e715b47c89853e4100142240a20b7c13f1239888cda5c8e6ac9ea10675df17633368906e66a487f91ddd3268ca6100142240a20a7c97a8b886fb36c1cd26977af2dabcabc59e565fb2ac4662ea3e825ba05eaf7100142240a208cc5497465564e76909d43a2baeb935c9c3ecba972164f0817940b51b83f9507100142240a20f3b1c0e43606bd49bc84e403db685b577e898b5f7c5db9052bdc3dacf38dc7ad100142240a20596004e838b7d90408d46de62b01e13e631575f348bc59f926286aae4c88702a100142240a203a511828c0cdc0fd257387843fdbf6f19d5ca1ef1c29ec7e971fc9f1195eb60c100142240a20ee5605c1a9ed9484a70b8a591339830f24530eefc4599a7c6188a78896908e2c100142240a2058a618057cf738d5fe0ecd46959bc90fc35e75abfabdad47466882af2328f451100142240a20d1c146d2429a5a827660008721c7a880e71f44feaa3dc75524c1a9281bac48cd100142240a20aa8539bb50cbc77bd9dd6ebe43e8d63547b5eab8c7d592f5f4ae8820dfc5161b100142240a20b28ba3f09e3521f7d09f0098c6743bc218ee2970068adfa5d908d9d06e627ae5100142240a202bb7b11dfe79a12110b0370510ef20d40ba8a563f517310f8bf37cf7f0403de4100142240a2040533a786cd0be9618126633512666b1b1b9ab3b436fa653c93e2f65da9c196b100142240a203fbbcb8e681f33040bd41a37ffd6997c61ebf8584923e25e89a6a067edb844be100142240a206a5f902cdeda2c2161d16bd21d586720c08c0bbb7b463d9c59147338d26a8a96100142240a20f459c0d1b1e42bf6c6f976b70e15e2842d753200ce2f901a7cf1b8c9bfb10c04100142240a2078ffabbea4ce0d6c8208eacda97bd6aa8b91fe5fd1226bbb08c63c12dfd416aa100142240a20fd4ae377dbe850ae5bdd50c71831ccf889ede1785d4011f20e3ca6edf0b93900100142240a20fcd1a93deb820043c92b87798d9aa88f699bf721daa89f143f90c800d906d3d1100142240a20e8e197cc511ee16a75a247697c20173114def67c902aa28673022e98dd36d339100142240a2056682332630f5dd42f160f1e3475d1881b2adab83023216892f538efa1e0e66f100142240a20c7d9cab0fb88dccd0cf4abbf5df9d7f36a3de439c5070c599d397e74133bdfed100142240a20866fe4ae134938ccf4c5caf01358881c75cfde473b0bb8a2623cc231db2ea3b9100142240a205cecf03ce3d4f89afc378058886f46a745f1672efea7f0a01160cf2df98bade0100142240a202892778d5f2c58f231c340c9ac3fcc339abe45e6cf51d207a3cf36f854643ca2100142240a20fc555c014d927fe7838e840609916b9af5d93892ce6f394398858286802b8c61100142240a200aa2bb3872ae3916648f22ce359f3953ecd1ff971c0f265328e049dbad9e935b100142240a208c3dbd5a1b928cb51e918452191e5328b0f6e8de0259517831a90b02dba3e611100142240a20aa6fd06015b8309ea9103f4b0e29bfa7e21ab5bb1431c3a76433f591c739f838100142240a20fbe806f96dcc137dcaea26125661f35d36aba1e2b2d7c89cf6a536c1a721c3a0100142240a20a892342c56149812a43e98c82b6a3dfee7eff384d9b6d832002ea63838f64900100142240a205cd92a7f0b1a7ded1de21ee73439f8851622b8d3e4664f68db20269cf0ede9ef100142240a2023f07bcb8dade90cefd08732183d5fe32b89ad8a4f84987b9e4e7e767c258756100142240a20be6b59cb464fadec4e0b22a9f12857ba5cf7a14c4f9551439d36305d582dd7fb100142240a2056cd1acff4de0c0d403c9f34bcafd74cb9e50e47b0b2ffda16f3db9a5fa12d48100142240a20acc5d703d7819d7993784e6ce107cd4c5625954d3baab0bcc07afe7d8c473a22100142240a20300c6f7d60252ecda817a4161b72e77445ef0f95874503ed01533a78a917411a100142240a205ed0b6dcf93bd071118859eb275ddf122a7aacadcb6c46cd518846f0d4733a15100142240a204b98c985398b5d64b7c1cc430cae9fe4a7bb24e6051db62555a83c9662660704100142240a206ba7fd1d9a17418b9d8a66d934fe866d69d2065d7d8ba1f6dc051fd4a2f6de01100142240a206ade09347d6072d154bdf0b13d0814ce9bf2a3b163c7092d71f0ed5cdcc9e1c9100142240a20f733feae629c4d654a9f1e5de09e422de82ff8aaed17a0701ab43450e63720f6100142240a208cfea50157362012d46ebbd35d758d9e7e583192a19ccd9fa44e8e566a2f5f61100142240a20d0b4aada93fb3d0b358e8d59f8914f5987acbc6f5b4f3e12a1ed8d962eded827100142240a204f950be5bf61958a0ae6e6a036b85b33ba838de8bd5508b558cbe2602d341168100142240a2082e6b6bf1f275c59fbde0dad91393ae690d5e644d11d23898a9ccfdda025920a100142240a206f7a1a4f9d8c054907f3b6b946a59525199dbcffe9fd3ba89612a5d4b548bede100142240a20664321f9b5f8f1154b1f6ecf936629232bca97b4e6cb7b15b18f6788df57acf2100142240a2074b60dfb12745236ba9dc8f925efa21482b50c870c567638ba3240a05fe363d9100142240a205b379072ec1f3f70b4650979a47b24f9b080c03450f7e9587d92cb599fcf4d6b100142240a2018b45f8c44b86e28967b6a1d16b7ceef81ba4dab940af7f431e2c362779bf8f6100142240a208934b9a6c38dd420aefa1e115c839fad7a71147a6efc77ab593485c3b07576ac100142240a20d6dfcd2fd4743915bd90a97b149d985bab1371d416ea1c94878f425b2a312a9c100142240a20d3b3d9703a265a51a137b9de348587c645a168981ef3e01c9dc482856751b8ef100142240a20fc787e0cc80b880cec43b8186eabcaf905c6df87582986fe724884d8f50addfa100142240a20c0444bc7cff2374b76ba49e5ce25b632b0db77f08f6c40799077ce3f5b035f3f100142240a20a2ee8d9c22eba97998d63854409d99c87b21d079f5c888d863a81d41d6abf011100142240a20c2344c6fb787621295335b6b5acbab9531199901492d41e13a64dfb2927f83be100142240a2093d7130f787b66a92d9fa3d9abd8047e5a9cb2dbd639f5bac360155b8c909f58100142240a20d7a02b1ba41b222313975f17552d85546feb24331205aeb12d664a0b31597a2f100142240a204c51b84496e533a7f1bb2b806f3733a7f0a48543976d0a4e5410e831f93e0417100142240a20615d64d17a182504db609d14dc6a45a109817cfaaad1608b4c48382bee48d378100142240a201d704a1e9e8ceb87d7ebf583e96a6318ce17bf25ffdedbe57a4fb200f9c8f206100142240a20155763a153e0c02b2eef1d8a9cd8a50a9eaac9d3af7ad1e559b1e2320b521e1c100142240a2098ec45e2f815f8bcb99d5006439ac4418667db79c3abf07c8b34e6568c288142100142240a205208fe27477acbc6750d0caa831c0b26801c58eca046a6cf9d4143f85d877e83100142240a20a3d4641c0c5c4fc1012044d8daa6853b70e6dad8178fbafa2d93046db48a4ca0100142240a203773b05d2e2660498c8d407a1d6223423c26aa08b9b3656af94d78b861fba22f100142240a203ca72e96f3893cd5250ff6ba60a9cdfa049945a3740ad6338398d70a1355e06f100142240a20500a9e1ea37636c24ebf4e32668897dae936eef42b6da49027f606d4f24a7e96100142240a20ed171997fcc2549c08a39a0367b723c68434e08b7c104320f410a6da9a6d822a100142240a2039712f967c4e06d284d6da4735cf2823c1f770674b668f7bc896071c3fd41eda100142240a20d42a22861f9f410c2f0733d0aba4ce6ba6cfa9c69ad5b6c6502a2b99ae8c0b49100142240a2095fcf1464d41feaa1f7659756d5278e7a42dac7951ed211f6caeb3e75285c75b100142240a204df6e4d7da2c11ddb9153c714699f9ab625bbcd5e8e38f1cdeee567dc04abc61100142240a20f2ecf2265889c11a724e4fdb0e9f0fd9905b776989014222b047ff9de5fe4014100142240a20b8cf76a8082d518016ea5588be57b64e156ed23b0f72248975522c23d15ef161100142240a20a3d9e6ea3bf5bbaeb654fa0667fa89d6333e1bda95488c5e0cf70fc3a6f5f354100142240a202565e04c6e0b665fac87f8238bee74837f8f621ed63409f3838d263603400a98100142240a20c2e6a39d423f82b034e5370932f02653ee45254ced3ec2094640508e3d04d91b100142240a202eede43f3ecd9490e538e4caf623079ca8ac9f6c7ede20686ac4ed6737608e43100142240a20f25a93ddbbd03cf6db28c430de1bc7c0cc4ab2912f9df692da294a3d97032ec3100142240a202a38a4dcdffc571d547ee3e59121fdd378335df7c65877f983f0957cdc982413100142240a2013bb7d0d3ad02c62ffe9dd420451792fe18502818635ca189cdfb27cee4e0ae7100142240a203a76a7eebe9edea2fcfb70ed044111a1e8f8aa85ee30efe364586845cf28c478100142240a2053db8eab8282ff36ab195de04f694c726085d18722e02ad3713c7e0a64353d00100142240a204dc2e72680dbf0156818114fa34d3591f8e6948df25a128e191190c66f6ac353100142240a2081c5c3bebb947d1716d5f6f325afdc71c953c23fd85a716dc082790bbb8d1915100142240a20121e86ff9fde3cc3d2ae12c7346e007c7e025dc2dbb9474494659d76903fdf4f100142240a207af0706d921a331b5c1f957a8d1decf15aa0e3377a80d72107693afd8614ef8f100142240a20ea7689b86efdd05afd7e5cd450b13583220b4428e26547f6203b4f9fa5bf7e9c100142240a20f5e2c973d4ccbdd36d8ed4e3dae75d3b50002cc1c74eb0a1fe29ef1ef8e3f0cc100142240a20258f23d60623fcb1d699a35f79776a9e81e979150ba5098dcc8e3a69a407ff9a100142240a20f56f5b3de4f46bcde88569888783fd60646806795807edb1e04ec3dff85ff3e1100142240a20b1f748c87b12edf7242b4719cfe68e6761acaa4329beee538d6ff740941d3cd3100142240a20a3dbfbf4eba67c252cf10f45008eff4581f97a5c4f377260de7538aac5a422cb100142240a208b35aa0d212e4032ede352d5426696c260c9ecffa632a2e14e6a6629a6c9f637100142240a2072341ca1eb3a0fd4e790318b51ace114315573d5355c9d5336e39382ba064256100142240a2057cdefa56a9a1df54da7c8b7220e8e1932f5d1a414a74daba754c0072f1dffdb100142240a20b14911cb101f45b88c437a36bd82a5f8641b1c55052060ea46ea2baea570e8bf100142240a209d31c78ebe9664e2bd8ba41cddcd9c5b9fb0cc6f8be0a2e5b91f6bcf66a5c753100142240a20290be52ae5259f3e8bd0cdf454004607867a97c1db86d0c9ff16c8c83625bc83100142240a20c91199db47b7591bf1a928534cdbd3e5979a0d09d6a464f2a9607ec444bd8a97100142240a205e0fad9f1e6837994babd7d55b2d4079af785ec48568cd1c4efd60ea28162e8e100142240a204c43a374e6b3e6f7d6c5a6b8518ae0d0d232a1c46d63af8fbc800045a14cbe93100142240a2045983f6d17109e1957bb37362f40a1eb0c8d5259091ee9f8a1693dd821e926eb100142240a2054ad6303d60c57a12c38568dae27449da3a436a1ee225e76ae219c618adf7fdb100142240a2022edc626e6c76166af032395ade8b874ac872074c72e3e6a70c8f668993d19e8100142240a20ee73c2912d35fe8d3f5bd51303d89bbd8500cd12b3a17594ab3e9ac4cee92220100142240a2081738498a35450b3e36b0a8af266f887bbc3205c00aa46b04fab7f47bc0dc775100142240a20194d95608f8340cab24e8bb302f1e4b02803ebe8f750888aa5da5508019f5f26100142240a20ac0ef45b1c0f242a4aeb0ef03a9c0d9e3ae2a4bee8cb4c35f0293bac4331e765100142240a201887f7937a8ca4ec67ceaedebfad3d307b10f416219de1ed4f8646409eab730a100142240a20a3392683550322454a4b5745afbe051746c4e34170ab2af0d1ce40307c7b7f9a100142240a202c203d61f409a43e44773a1e7024ebb67b45b489e675ef384334e073bb935ad2100142240a20e1c1327187f940cd73d5afb676b4b28399109aa6a6c3868707df1dc3d62a8382100142240a20fc2ff0598fd334e425949d552cd31b59e87365b05d6151aa2e8e982e44f29609100142240a20ee40b85aa515fdbe6e559f815edc7ec1e321e0d152dd54258382ba31b54e447a100142240a201267b8cba49007ec111d8d037e52e3e32c7ad6c2f41d6b45e90b8402a24005d3100142240a20c3e904b307c280271b2a2bca82cefc7e14d75f776e08ffe58fc4f7fa3d1d0a61100142240a207cd2af9dc098c7fa40243b775ab92a9c5cb3a31380bb3d5779e8b7015c9cf189100142240a208dc1439b28d488419535c2fea9c821023f7bce423fa02d9f42e60c07c3d6ab56100142240a20c3150e4ca47e7bc6486f198bf0a742a18ab0871bb1efa34e82c4ce81f2792667100142240a202e8f05a1e5ba68d6e4b93c289e885f630cf27f720181060c437869ab9e839391100142240a207a92827270cfa82f16145a44f9bdd9ff5038ef1b665dd520a2e61db974909496100142240a20ebb1834a568ea7653bc66f394937a0aa4f2f617118d2161e2c11c0d7d2febb9d100142240a2054840bd4fe6fbabe28e2e1e54cebb2002e9523d2e1543b50f22dad31a07bc700100142240a202ea29e0897dea1ebd35d08aaf170ffbcef9b1add7a80aa3afdd9161fdd4329e0100142240a20d49bb29ccd716262579a5a448c377aa16ff969d5b2c282a43de87abaf4174e93100142240a20fd2d8b8e265ca440581956a7e36b57eba685631193bc4fe0a2b638887ca5cc0b100142240a20b6d7ac98d6667fe8f4a186ef12a66b017df05a54e11b45d3f2967cb3d6b6827f100142240a2012e9b2bfe3d95c70fe8036a6cc8f677d67c9c65724aef87cc2a2eba5555d8107100142240a20e5f305cd6d930d41209036f066a86d3724aad2f388d354ac4979cc763c220a56100142240a202605f256953a6cd468cb45aa10509abd14d606166578147b8f4d2c70e3c5a4d3100142240a2045e03689170b9fc027db927a99842e2c13f9fcbd875a74e2b54a2663f3c23dfc100142240a204f623eddef2d19ee27b9e5270a51290741fb7953f3465ecbe521ced55d97b9af100142240a2098a6defcfeb3dfca36ec46eb373734627a2c395cdd2d886046a11788ed6467af100142240a2035db4f9ab121f213b695b982f50c47ac04a14c54214e831ed1bbef7b9542310c100142240a20496826b538d97906e14d417b48598d7d591a483ab5f4c6786cd0b96239c2f4cf100142240a20ef27d1d5573e6419ffcc7c4372668105118256b810ab2e2dbb5e7d8280ee9425100142240a20ed3f97f83ee7799c4aeba9f40bd18134c862c1a30875fc1e038cd27bae3a739a100142240a20348e8bacc2d40191a6a00e137c745a422ff8fb03148d2ff9cad75efbb7a67014100142240a20316d5dbafd16258b7077d46a3dc32c46986a56a3eb095094d3f8fdd6acbf91cf100142240a20bfcc366ce0bfebc76ac157ed0814e2516786e48eb1a3313b5fe37283e9cf2c5a100142240a20cafe2356af019d8b5f39c949b3dee71dd5356eb2655d819f485d0244e55b9fcd100142240a206a5dcdf2c3598c00d31a010dece5bf744f203856712531b07a31babcf06d09e3100142240a20d03903d0b6fb3ea0891ceeb3a84afdd6979f453d70ee1531168e418d0a886972100142240a2008bd9e0a5b268a1ec30a8efd34973a15e2e487c9bc7a987514f8ba28e2df81b8100142240a20e18f9d4831f99a874b84355d8404b084c06793b62e78cd05e479e7e399dc6869100142240a200938374801c6b36a9164ffdbec02bd2d54b7f927a850f46136126726774166eb100142240a20efb31c5039e45fda750af5d2ea31b3210a407b168adec8f98db1f3dbc4cd9438100142240a20e2d564796e2660ba1bf553a21581f82116f775002a0dc0f26439f54c660fd84d100142240a20bff74b6ff5ccdce9a659408871d486511938422a6e550fe0669ccd739f801081100142240a20e41a59c4d549c3e42a7a8d6dadf124bbba1e44ba8ce985aff12eca30e88f9875100142240a2082019acc07b285d7edea9f72e3d5285312e994beb056e6faf58492841a80d282100142240a2004eed32b0d6be0b60d2e33281f623471f3a65de597d2de13c0d8f887ea723aab100142240a20bc969803075efefd39758affc268d35239b19b63234bce1a8681d3fe5f454057100142240a20e01d6eda8094e949bfbb05c819b509a12eb662a414b822f841d9555d58e1ce76100142240a20c3576342cbf99792896ee5329b04ff2eee2fc2bb6d53c5c03d52c8957ee793fb100142240a20de186294778cc927cb2a24ade9ad165956f3586979f1ac5f3b4a16edd2674339100142240a2043a9a2915f377cd4943f602be38c3bd6ef39e91562c09f18fb672884b4bf8eab100142240a201a608965f3df6f3b63f91616d5c7878c3decad01ea6b3b286be37c62e30118b5100142240a2059dadf17dc0f84c1c334e6a815a04bdfcd0988a3f3bcb713c66ae29e17276cc6100142240a208d5cb94ebe135f3e9fb84ed39aa213a14c57b98f19d2cf35f2cc450b44e3792f100142240a20297dc7cf28b3d40a42c5e8faac876bf7227c670e6e031dd0e0537f4fb70e6fbe100142240a200909c1eff97923f6bd47e6ec8b5dc163b32d5b675a3b13138d3599e5cc77ab9f100142240a20f7bdbb5c598784ba157940b08b5c5aa967d0f3faf60da28b915650eb39afe47c100142240a2022b53680ec36fef4aa69033220dca0e12722cbb58cf4acf7de23b3144cdb534a100142240a2077aef3319ea0729b546a0fabba015b08e2e201014b8a6cd05e6d239d89a00872100142240a20677358fc648638cbd854d2a009dd39b8508dd3047d0c5f13bb403a64d053ad03100142240a20645b81dc1769e34a815cc5682abd3c3b3ea60afa9705a5ff2908ad354b0f9ca4100142240a2073dce1727ec50c50366378cd389246e19543d900d7e9dab8e323d04cf5b202a7100142240a202ada479e1fde7211de15900ed29035f6f07506cdbadc4c232aab96d62006b9de100142240a20ee1a56541083b083f1a13b5f8aa4042a098de2cec4c43f3250baaba8f03a52ea100142240a20e2cc4424464983ada824bbef5aaa8995e80ef1017c15ef3b13902599841637ba100142240a205246164c02e9e6bfb7a2dbf7ba8a5440a189f8048373678f5b1085f217464c05100142240a20b73fa06a1769cf81bd238e8633bb3a54432cc70408d44cd23423d586ffd711fa100142240a201889f1b031fa7d64c4b480a17cd2941be2c02681bca397c93607091f37bf5ee1100142240a20b884577e30ee6428215afe3b9bd2d7b5e50f724d74a8e11defc1b52ce411b385100142240a20fd734da3c98d7d56f8b9cc92f389350f3b6de2fb5917d338fee9fd961978e2a3100142240a20229fbcb8d2c81679fc46e983723defda5c232a98964f295b1ee0ac100f38b8df100142240a2084845f99ad8fe89b7aa136c30b71b791015e8801b1d42cbf9841a0b90e723aad100142240a208c258b4ca16dfb8e5afde4cf40fecf115307798117b4860475511b3b5d561584100142240a20ea8c8bec277832ff4f31396f76d6117026cf73407bce46de9ccfd17c686ca117100142240a200e7bc96895730726225e47e31c742fd44d3936f31ecd32658c694fc11df25224100142240a203726c5309429bb2ac37f35f67ade81f691cde2fc1e84cab80db6527b8d0dab9e100142240a2017dd31b9a0e4e050b82550ffe7920c3f53392afb5f6e546114921d5a4a2b6fce100142240a203f6aeb3ed0d67d71ee11a1c2e1ed66da77abd8cc8c0f4d995d64fe4ec8736847100142240a206375ae0c1bf2b0388e16971715fd68c31ed179f38947531c165f0da2c3fe7667100142240a2012aa455c35356255f9ba286193962f3bc23f7fff565c1995ac21015bd946fe6e100142240a204a46371fa537ccb9ad995cd03c69517dfd310fd4af9d8a95c7976cf8eb6c37fc100142240a2025196076d22b8093c7a53a1be1f9ba73d404c73dde8fd8c49465990eb7d7707a100142240a2047ede3eca5f6c8f0a51f2d36d7d1c236aab07a8b2aaa6ab08c8ccf8d87ce3dd1100142240a20a2a4f4e18a49ae5fa0170e13427a58e53779a64bdbf817183420f9abdc0cd409100142240a2017638d776ab08bfd9eab2ee19973c1cae6b2904830f14c12d675bfe88e0d7e07100142240a20d7a0c586d45638b0c2b335420a57c73b2acbe1a8038a25c800a030ffa1e62fbf100142240a20af470f2d66661e59ad85302877ba095309d859cea3d5086c5c184030c8f5bdf3100142240a2052f15f5f5cbea0ee45d19f44601ba96cd533ccdcd3b9e5f966d059294d0e0a96100142240a204485615186ae8ad33aff337594a7f8325d3a50201ee04320fb1dd2eab9643d16100142240a202b07c366c1ed899a2ac9722b76e092828714a8177368e51ca05ec05c565866b3100142240a20e81206b483fdcf1fe42145f27d1efb8178a57ee24b196285374037da2a53b323100142240a20a0586d138691972dad187e747b31698e301c3e1d58d5ff6be8a06d4ec5d6b855100142240a2027a748182c49d7c4ec091c9589da5c9c35fa99ac8ffa4d613129dd4d08e9f112100142240a202773bf65269603924e514b545eb69d988b5f6b5d94310f2eac5fe549cadad07c100142240a20ff568b8e0dbe74604c6d04b5de6f2df03b343bea074c4495c6a10e511ba473d9100142240a209337ca812d686250385c635198917e85833d7a9b0ab4c537246fb069d09f401d100142240a203a4bfd825ccd308def4a61bac80c579db42b02ea5f1a0768656a326e83a0dd88100142240a20b5e0d0ae74b0ac76c0195fecca9660a241920ba97cdc5a247c7e941ee1f5407c100142240a201b47bfdc6dc74483d9f8b2b6317c4c1a38b781bdcb8e054ba268fe470a2617a0100142240a20f33b53556f28f27d250585317cd57a18f37c5d5f2b3c1bd00a0c91ea54f1636f100142240a205b2085a051ff58153cf38a4abe8411d1c24c828d1c54779ee778eb8f9a419cb8100142240a2025017db1dce55f9e64405da1d3d61dae9b5138fd60fc9f1f3c3187ff22e78ac1100142240a2094efdddccdd25ae650a89596fe23a93801b087be8e5b380451a79d7ddf4a1d5e100142240a203ee2346e1195969237202d47b843937c1038c34c67dca5a3fcd1127b9e8aa014100142240a2080629b67e7b2d6f57e9da3cb29cc8463a65d125039628124784f944b94c29e88100142240a202c3ce4d7102f4236e0df2e8f5797f6dc3d2e6f0e57d373c9a4b89b21d4d22868100142240a20fc678679f11bc904720421273689e6826acb42b21e2c4c5c1d7bf532d8966841100142240a2030afa9ef79458f844a27a3881655b6eafb90b3912737f1bcddb95e78eb098c1d100142240a2095885aaa49faca9622ac9eab12f2efa509361511039934ffeff93eb948eb517d100142240a20c222ba3fffec09b85f1992db2dcaeaa48205903219789369855e347fd188144b100142240a2012c277c98d0593ec3d0f9d01e8ce08f67813197b67fe96d2be7c5ba6481515df1001").unwrap();
		let AnyClientStateV1::Grandpa(old_state) = AnyClientStateV1::decode_vec(&data).unwrap() else { panic!("bad grandpa data") };

		// should ignore other clients
		<ClientStates<Test>>::insert(&ClientId::from_str("00-unknown-0").unwrap(), vec![1, 2, 3]);
		<ClientStates<Test>>::insert(&client_id, data.clone());
		<ClientStates<Test>>::insert(&wasm_client_id, AnyClientStateV1::Wasm(WasmClientState {
			data,
			code_id: vec![3, 3, 3],
			latest_height: Default::default(),
			inner: Box::new(AnyClientStateV1::Grandpa(old_state.clone())),
			_phantom: Default::default(),
		}).encode_vec().unwrap());

		Pallet::<Test>::on_runtime_upgrade();

		let AnyClientState::Grandpa(new_state) = ctx.client_state(&client_id).unwrap() else { panic!("failed to decode grandpa state") };
		let AnyClientState::Wasm(new_wasm_state) = ctx.client_state(&wasm_client_id).unwrap() else { panic!("failed to decode grandpa state") };

		assert_eq!(1, new_state.authorities_changes.len());
		assert_eq!(old_state.current_set_id, new_state.authorities_changes.last().set_id);
		assert_eq!(old_state.current_authorities, new_state.authorities_changes.last().authorities);
		assert_eq!(0, new_state.authorities_changes.last().height);
		assert_eq!(Timestamp::from_nanoseconds(1).unwrap(), new_state.authorities_changes.last().timestamp);
		assert_eq!(old_state.latest_relay_height, new_state.latest_relay_height);
		assert_eq!(old_state.latest_para_height, new_state.latest_para_height);
		assert_eq!(old_state.latest_relay_hash, new_state.latest_relay_hash);
		assert_eq!(old_state.para_id, new_state.para_id);
		assert_eq!(old_state.frozen_height, new_state.frozen_height);
		assert_eq!(old_state.relay_chain, new_state.relay_chain);

		assert_eq!(*new_wasm_state.inner, AnyClientState::Grandpa(new_state));
	})
}
