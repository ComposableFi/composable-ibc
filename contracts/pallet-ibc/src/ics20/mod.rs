pub mod context;
pub mod memo;

use crate::{routing::Context, ChannelIds, Config, DenomToAssetId, Event, Pallet, WeightInfo};
use alloc::{
	format,
	str::FromStr,
	string::{String, ToString},
};

use frame_support::weights::Weight;
pub use ibc::applications::transfer::{MODULE_ID_STR, PORT_ID_STR};
use ibc::{
	applications::transfer::{
		acknowledgement::{Acknowledgement as Ics20Acknowledgement, ACK_ERR_STR},
		context::{
			on_chan_close_confirm, on_chan_close_init, on_chan_open_ack, on_chan_open_confirm,
			on_chan_open_init, on_chan_open_try,
		},
		is_receiver_chain_source, is_sender_chain_source,
		packet::PacketData,
		relay::{
			on_ack_packet::process_ack_packet, on_recv_packet::process_recv_packet,
			on_timeout_packet::process_timeout_packet,
		},
		PrefixedCoin, PrefixedDenom, TracePrefix,
	},
	core::{
		ics04_channel::{
			channel::{Counterparty, Order},
			error::Error as Ics04Error,
			msgs::acknowledgement::Acknowledgement,
			packet::Packet,
			Version,
		},
		ics24_host::identifier::{ChannelId, ConnectionId, PortId},
		ics26_routing::context::{Module, ModuleCallbackContext, ModuleOutputBuilder},
	},
	signer::Signer,
};
use ibc_primitives::{CallbackWeight, HandlerMessage, IbcHandler};
use sp_core::crypto::AccountId32;
use sp_std::marker::PhantomData;

pub type Ics20TransferMsg = ibc::applications::transfer::msgs::transfer::MsgTransfer<
	ibc::applications::transfer::Coin<ibc::applications::transfer::PrefixedDenom>,
>;

#[derive(Debug, Clone, Copy)]
pub enum FlowType {
	Transfer,
	Deliver,
}

pub trait Ics20RateLimiter {
	fn allow(msg: &Ics20TransferMsg, flow_type: FlowType) -> Result<(), ()>;
}

impl Ics20RateLimiter for frame_support::traits::Everything {
	fn allow(_msg: &Ics20TransferMsg, _flow_type: FlowType) -> Result<(), ()> {
		Ok(())
	}
}

#[derive(Clone, Eq, Debug, PartialEq)]
pub struct IbcModule<T: Config>(PhantomData<T>);

impl<T: Config> Default for IbcModule<T> {
	fn default() -> Self {
		Self(PhantomData::default())
	}
}

impl<T: Config + Send + Sync> Module for IbcModule<T>
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
	AccountId32: From<<T as frame_system::Config>::AccountId>,
{
	fn on_chan_open_init(
		&mut self,
		_ctx: &dyn ModuleCallbackContext,
		output: &mut ModuleOutputBuilder,
		order: Order,
		connection_hops: &[ConnectionId],
		port_id: &PortId,
		channel_id: &ChannelId,
		counterparty: &Counterparty,
		version: &Version,
		_relayer: &Signer,
	) -> Result<(), Ics04Error> {
		let mut ctx = Context::<T>::default();
		on_chan_open_init(
			&mut ctx,
			output,
			order,
			connection_hops,
			port_id,
			channel_id,
			counterparty,
			version,
		)
		.map_err(|e| Ics04Error::implementation_specific(e.to_string()))
	}

	fn on_chan_open_try(
		&mut self,
		_ctx: &dyn ModuleCallbackContext,
		output: &mut ModuleOutputBuilder,
		order: Order,
		connection_hops: &[ConnectionId],
		port_id: &PortId,
		channel_id: &ChannelId,
		counterparty: &Counterparty,
		version: &Version,
		counterparty_version: &Version,
		_relayer: &Signer,
	) -> Result<Version, Ics04Error> {
		let mut ctx = Context::<T>::default();
		on_chan_open_try(
			&mut ctx,
			output,
			order,
			connection_hops,
			port_id,
			channel_id,
			counterparty,
			version,
			counterparty_version,
		)
		.map_err(|e| Ics04Error::implementation_specific(e.to_string()))
	}

	fn on_chan_open_ack(
		&mut self,
		_ctx: &dyn ModuleCallbackContext,
		output: &mut ModuleOutputBuilder,
		port_id: &PortId,
		channel_id: &ChannelId,
		counterparty_version: &Version,
		_relayer: &Signer,
	) -> Result<(), Ics04Error> {
		let _ = ChannelIds::<T>::try_mutate::<_, (), _>(|channels| {
			channels.push(channel_id.to_string().as_bytes().to_vec());
			Ok(())
		});
		let mut ctx = Context::<T>::default();
		on_chan_open_ack(&mut ctx, output, port_id, channel_id, counterparty_version)
			.map_err(|e| Ics04Error::implementation_specific(e.to_string()))
	}

	fn on_chan_open_confirm(
		&mut self,
		_ctx: &dyn ModuleCallbackContext,
		output: &mut ModuleOutputBuilder,
		port_id: &PortId,
		channel_id: &ChannelId,
		_relayer: &Signer,
	) -> Result<(), Ics04Error> {
		let _ = ChannelIds::<T>::try_mutate::<_, (), _>(|channels| {
			channels.push(channel_id.to_string().as_bytes().to_vec());
			Ok(())
		});
		let mut ctx = Context::<T>::default();
		on_chan_open_confirm(&mut ctx, output, port_id, channel_id)
			.map_err(|e| Ics04Error::implementation_specific(e.to_string()))
	}

	fn on_chan_close_init(
		&mut self,
		_ctx: &dyn ModuleCallbackContext,
		output: &mut ModuleOutputBuilder,
		port_id: &PortId,
		channel_id: &ChannelId,
		_relayer: &Signer,
	) -> Result<(), Ics04Error> {
		let _ = ChannelIds::<T>::try_mutate::<_, (), _>(|channels| {
			let rem = channels
				.iter()
				.filter(|chan| chan.as_slice() != channel_id.to_string().as_bytes())
				.cloned()
				.collect();
			*channels = rem;
			Ok(())
		});
		// Remove escrow address for closed channel if it exists
		let _ = Pallet::<T>::remove_channel_escrow_address(port_id, *channel_id);
		let mut ctx = Context::<T>::default();
		on_chan_close_init(&mut ctx, output, port_id, channel_id)
			.map_err(|e| Ics04Error::implementation_specific(e.to_string()))
	}

	fn on_chan_close_confirm(
		&mut self,
		_ctx: &dyn ModuleCallbackContext,
		output: &mut ModuleOutputBuilder,
		port_id: &PortId,
		channel_id: &ChannelId,
		_relayer: &Signer,
	) -> Result<(), Ics04Error> {
		let _ = ChannelIds::<T>::try_mutate::<_, (), _>(|channels| {
			let rem = channels
				.iter()
				.filter(|chan| chan.as_slice() != channel_id.to_string().as_bytes())
				.cloned()
				.collect();
			*channels = rem;
			Ok(())
		});
		// Remove escrow address for closed channel if it exists
		let _ = Pallet::<T>::remove_channel_escrow_address(port_id, *channel_id);
		let mut ctx = Context::<T>::default();
		on_chan_close_confirm(&mut ctx, output, port_id, channel_id)
			.map_err(|e| Ics04Error::implementation_specific(e.to_string()))
	}

	fn on_recv_packet(
		&self,
		_ctx: &dyn ModuleCallbackContext,
		output: &mut ModuleOutputBuilder,
		packet: &mut Packet,
		_relayer: &Signer,
	) -> Result<Acknowledgement, Ics04Error> {
		let mut ctx = Context::<T>::default();
		let result = serde_json::from_slice(packet.data.as_slice())
			.map_err(|e| {
				Ics04Error::implementation_specific(format!("Failed to decode packet data {:?}", e))
			})
			.and_then(|packet_data: PacketData| {
				// We need to reject transaction amounts that are larger than u128 since we expect
				// the balance type of the runtime to be a u128; For a U256 to be converted to a
				// u128 without truncating, the last two words should be zero
				let msg = Ics20TransferMsg {
					source_port: packet.source_port.clone(),
					memo: packet_data.memo.clone(),
					sender: packet_data.sender.clone(),
					receiver: packet_data.receiver.clone(),
					source_channel: packet.source_channel.clone(),
					token: packet_data.token.clone(),
					timeout_height: packet.timeout_height,
					timeout_timestamp: packet.timeout_timestamp,
				};
				T::Ics20RateLimiter::allow(&msg, FlowType::Deliver)
					.map_err(|_| Ics04Error::implementation_specific("rate limiter".to_string()))?;
				let amount = packet_data.token.amount.as_u256();
				u128::try_from(amount)
					.map_err(|e| Ics04Error::implementation_specific(format!("{:?}", e)))?;
				process_recv_packet(&mut ctx, output, packet, packet_data.clone())
					.map(|_| packet_data)
					.map_err(|e| {
						log::trace!(target: "pallet_ibc", "[on_recv_packet]: {:?}", e);
						Ics04Error::implementation_specific(e.to_string())
					})
			});

		let ack = match result {
			Err(err) => {
				let ack = Ics20Acknowledgement::Error(format!("{}: {:?}", ACK_ERR_STR, err))
					.to_string()
					.into_bytes();
				Pallet::<T>::handle_message(HandlerMessage::WriteAck {
					packet: packet.clone(),
					ack: ack.clone(),
				})
				.map_err(|e| {
					Ics04Error::implementation_specific(format!("[on_recv_packet] {:#?}", e))
				})?;
				ack
			},
			Ok(packet_data) => {
				let denom = full_ibc_denom(packet, packet_data.token.clone());
				let prefixed_denom = PrefixedDenom::from_str(&denom).map_err(|_| {
					Ics04Error::implementation_specific("Failed to parse token denom".to_string())
				})?;
				Pallet::<T>::deposit_event(Event::<T>::TokenReceived {
					from: packet_data.sender.to_string().as_bytes().to_vec(),
					to: packet_data.receiver.to_string().as_bytes().to_vec(),
					ibc_denom: denom.as_bytes().to_vec(),
					local_asset_id: T::IbcDenomToAssetIdConversion::from_denom_to_asset_id(&denom)
						.ok(),
					amount: packet_data.token.amount.as_u256().as_u128().into(),
					is_receiver_source: is_receiver_chain_source(
						packet.source_port.clone(),
						packet.source_channel.clone(),
						&prefixed_denom,
					),
					source_channel: packet.source_channel.to_string().as_bytes().to_vec(),
					destination_channel: packet.destination_channel.to_string().as_bytes().to_vec(),
				});
				let packet = packet.clone();
				Pallet::<T>::handle_message(HandlerMessage::WriteAck {
					packet,
					ack: Ics20Acknowledgement::success().to_string().into_bytes(),
				})
				.map_err(|e| {
					Ics04Error::implementation_specific(format!("[on_recv_packet] {:#?}", e))
				})?;
				Ics20Acknowledgement::success().to_string().into_bytes()
			},
		};
		Ok(Acknowledgement::from_bytes(ack))
	}

	fn on_acknowledgement_packet(
		&mut self,
		_ctx: &dyn ModuleCallbackContext,
		_output: &mut ModuleOutputBuilder,
		packet: &mut Packet,
		acknowledgement: &Acknowledgement,
		_relayer: &Signer,
	) -> Result<(), Ics04Error> {
		let mut ctx = Context::<T>::default();
		let packet_data: PacketData =
			serde_json::from_slice(packet.data.as_slice()).map_err(|e| {
				Ics04Error::implementation_specific(format!("Failed to decode packet data {:?}", e))
			})?;
		let ack = serde_json::from_slice::<Ics20Acknowledgement>(&acknowledgement.as_ref())
			.map_err(|e| {
				Ics04Error::implementation_specific(format!(
					"Failed to decode acknowledgement data {:?}",
					e
				))
			})?;
		process_ack_packet(&mut ctx, packet, &packet_data, &ack)
			.map_err(|e| Ics04Error::implementation_specific(e.to_string()))?;
		match ack.into_result() {
			Ok(_) => Pallet::<T>::deposit_event(Event::<T>::TokenTransferCompleted {
				from: packet_data.sender.to_string().as_bytes().to_vec(),
				to: packet_data.receiver.to_string().as_bytes().to_vec(),
				ibc_denom: packet_data.token.denom.to_string().as_bytes().to_vec(),
				local_asset_id: T::IbcDenomToAssetIdConversion::from_denom_to_asset_id(
					&packet_data.token.denom.to_string(),
				)
				.ok(),
				amount: packet_data.token.amount.as_u256().as_u128().into(),
				is_sender_source: is_sender_chain_source(
					packet.source_port.clone(),
					packet.source_channel.clone(),
					&packet_data.token.denom,
				),
				source_channel: packet.source_channel.to_string().as_bytes().to_vec(),
				destination_channel: packet.destination_channel.to_string().as_bytes().to_vec(),
			}),
			Err(e) => {
				log::trace!(
					target: "pallet_ibc",
					"[transfer] error: acknowledgement error: {e}",
				);
				Pallet::<T>::deposit_event(Event::<T>::TokenTransferFailed {
					from: packet_data.sender.to_string().as_bytes().to_vec(),
					to: packet_data.receiver.to_string().as_bytes().to_vec(),
					ibc_denom: packet_data.token.denom.to_string().as_bytes().to_vec(),
					local_asset_id: T::IbcDenomToAssetIdConversion::from_denom_to_asset_id(
						&packet_data.token.denom.to_string(),
					)
					.ok(),
					amount: packet_data.token.amount.as_u256().as_u128().into(),
					is_sender_source: is_sender_chain_source(
						packet.source_port.clone(),
						packet.source_channel.clone(),
						&packet_data.token.denom,
					),
					source_channel: packet.source_channel.to_string().as_bytes().to_vec(),
					destination_channel: packet.destination_channel.to_string().as_bytes().to_vec(),
				})
			},
		}

		Ok(())
	}

	fn on_timeout_packet(
		&mut self,
		_ctx: &dyn ModuleCallbackContext,
		_output: &mut ModuleOutputBuilder,
		packet: &mut Packet,
		_relayer: &Signer,
	) -> Result<(), Ics04Error> {
		let mut ctx = Context::<T>::default();
		let packet_data: PacketData = serde_json::from_slice(packet.data.as_slice())
			.map_err(|e| Ics04Error::app_module(format!("Failed to decode packet data {:?}", e)))?;
		process_timeout_packet(&mut ctx, packet, &packet_data)
			.map_err(|e| Ics04Error::app_module(e.to_string()))?;

		Ok(())
	}
}

pub struct WeightHandler<T: Config>(PhantomData<T>);

impl<T: Config> Default for WeightHandler<T> {
	fn default() -> Self {
		Self(PhantomData::default())
	}
}

impl<T: Config> CallbackWeight for WeightHandler<T> {
	fn on_chan_open_init(&self) -> Weight {
		<T as Config>::WeightInfo::on_chan_open_init()
	}

	fn on_chan_open_try(&self) -> Weight {
		<T as Config>::WeightInfo::on_chan_open_try()
	}

	fn on_chan_open_ack(&self, _port_id: &PortId, _channel_id: &ChannelId) -> Weight {
		<T as Config>::WeightInfo::on_chan_open_ack()
	}

	fn on_chan_open_confirm(&self, _port_id: &PortId, _channel_id: &ChannelId) -> Weight {
		<T as Config>::WeightInfo::on_chan_open_confirm()
	}

	fn on_chan_close_init(&self, _port_id: &PortId, _channel_id: &ChannelId) -> Weight {
		<T as Config>::WeightInfo::on_chan_close_init()
	}

	fn on_chan_close_confirm(&self, _port_id: &PortId, _channel_id: &ChannelId) -> Weight {
		<T as Config>::WeightInfo::on_chan_close_confirm()
	}

	fn on_recv_packet(&self, _packet: &Packet) -> Weight {
		<T as Config>::WeightInfo::on_recv_packet()
	}

	fn on_acknowledgement_packet(
		&self,
		_packet: &Packet,
		_acknowledgement: &Acknowledgement,
	) -> Weight {
		<T as Config>::WeightInfo::on_acknowledgement_packet()
	}

	fn on_timeout_packet(&self, _packet: &Packet) -> Weight {
		<T as Config>::WeightInfo::on_timeout_packet()
	}
}

pub fn full_ibc_denom(packet: &Packet, mut token: PrefixedCoin) -> String {
	if is_receiver_chain_source(packet.source_port.clone(), packet.source_channel, &token.denom) {
		let prefix = TracePrefix::new(packet.source_port.clone(), packet.source_channel);

		token.denom.remove_trace_prefix(&prefix);
		token.denom.to_string()
	} else {
		let prefix = TracePrefix::new(packet.destination_port.clone(), packet.destination_channel);

		token.denom.add_trace_prefix(prefix);
		token.denom.to_string()
	}
}

use ibc::applications::transfer::error::Error as Ics20Error;

pub trait HandleMemo<T: Config> {
	fn execute_memo(packet_data: &PacketData, receiver: T::AccountId) -> Result<(), Ics20Error>;
}

impl<T: Config> HandleMemo<T> for () {
	fn execute_memo(_packet_data: &PacketData, _receiver: T::AccountId) -> Result<(), Ics20Error> {
		Ok(())
	}
}
