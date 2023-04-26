use crate::{ics20::HandleMemo, Config};
use alloc::format;
use core::fmt::Debug;
use ibc::{
	applications::transfer::packet::PacketData,
	core::{
		ics04_channel::{
			channel::{Counterparty, Order},
			error::Error,
			msgs::acknowledgement::{Acknowledgement as GenericAcknowledgement, Acknowledgement},
			packet::Packet,
			Version,
		},
		ics24_host::identifier::{ChannelId, ConnectionId, PortId},
		ics26_routing::context::{Module, ModuleCallbackContext, ModuleOutputBuilder},
	},
	signer::Signer,
};
use sp_runtime::traits::IdentifyAccount;

/// This middleware should be used to wrap ics20 to execute memo
/// We chose to use this as a middleware so that we can easily choose
/// at what layer of the ics20 stack the memo should be executed.
/// For example ics20 fees are meant to be collected before memo is executed, so
/// this allows an ics20 fee middleware to be executed before the memo is executed
/// USAGE:
/// pub struct Router {
/// 	ics20: crate::ics20::memo::Memo<
/// 		Runtime,
/// 		crate::ics20_fee::Ics20ServiceCharge<Runtime, crate::ics20::IbcModule<Runtime>>,
/// 	>,
/// }
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Memo<T: Config, S: Module + Clone + Default + PartialEq + Eq + Debug> {
	inner: S,
	_phantom: core::marker::PhantomData<T>,
}

impl<T: Config + Send + Sync, S: Module + Clone + Default + PartialEq + Eq + Debug> Default
	for Memo<T, S>
{
	fn default() -> Self {
		Self { inner: S::default(), _phantom: Default::default() }
	}
}

impl<T: Config + Send + Sync, S: Module + Clone + Default + PartialEq + Eq + Debug> Module
	for Memo<T, S>
{
	fn on_chan_open_init(
		&mut self,
		ctx: &dyn ModuleCallbackContext,
		output: &mut ModuleOutputBuilder,
		order: Order,
		connection_hops: &[ConnectionId],
		port_id: &PortId,
		channel_id: &ChannelId,
		counterparty: &Counterparty,
		version: &Version,
		relayer: &Signer,
	) -> Result<(), Error> {
		self.inner.on_chan_open_init(
			ctx,
			output,
			order,
			connection_hops,
			port_id,
			channel_id,
			counterparty,
			version,
			relayer,
		)
	}

	fn on_chan_open_try(
		&mut self,
		ctx: &dyn ModuleCallbackContext,
		output: &mut ModuleOutputBuilder,
		order: Order,
		connection_hops: &[ConnectionId],
		port_id: &PortId,
		channel_id: &ChannelId,
		counterparty: &Counterparty,
		version: &Version,
		counterparty_version: &Version,
		relayer: &Signer,
	) -> Result<Version, Error> {
		self.inner.on_chan_open_try(
			ctx,
			output,
			order,
			connection_hops,
			port_id,
			channel_id,
			counterparty,
			version,
			counterparty_version,
			relayer,
		)
	}

	fn on_chan_open_ack(
		&mut self,
		ctx: &dyn ModuleCallbackContext,
		output: &mut ModuleOutputBuilder,
		port_id: &PortId,
		channel_id: &ChannelId,
		counterparty_version: &Version,
		relayer: &Signer,
	) -> Result<(), Error> {
		self.inner
			.on_chan_open_ack(ctx, output, port_id, channel_id, counterparty_version, relayer)
	}

	fn on_chan_open_confirm(
		&mut self,
		ctx: &dyn ModuleCallbackContext,
		output: &mut ModuleOutputBuilder,
		port_id: &PortId,
		channel_id: &ChannelId,
		relayer: &Signer,
	) -> Result<(), Error> {
		self.inner.on_chan_open_confirm(ctx, output, port_id, channel_id, relayer)
	}

	fn on_chan_close_init(
		&mut self,
		ctx: &dyn ModuleCallbackContext,
		output: &mut ModuleOutputBuilder,
		port_id: &PortId,
		channel_id: &ChannelId,
		relayer: &Signer,
	) -> Result<(), Error> {
		self.inner.on_chan_close_init(ctx, output, port_id, channel_id, relayer)
	}

	fn on_chan_close_confirm(
		&mut self,
		ctx: &dyn ModuleCallbackContext,
		output: &mut ModuleOutputBuilder,
		port_id: &PortId,
		channel_id: &ChannelId,
		relayer: &Signer,
	) -> Result<(), Error> {
		self.inner.on_chan_close_confirm(ctx, output, port_id, channel_id, relayer)
	}

	fn on_recv_packet(
		&self,
		ctx: &dyn ModuleCallbackContext,
		output: &mut ModuleOutputBuilder,
		packet: &mut Packet,
		relayer: &Signer,
	) -> Result<Acknowledgement, Error> {
		let ack = self.inner.on_recv_packet(ctx, output, packet, relayer)?;
		// We want the whole chain of calls to fail only if the ics20 transfer fails, because
		// the other modules are not part of ics-20 standard
		let _ = Self::process_memo(packet).map_err(|e| {
			log::error!(target: "pallet_ibc", "Error while handling memo: {:?}", e);
		});
		Ok(ack)
	}

	fn on_acknowledgement_packet(
		&mut self,
		ctx: &dyn ModuleCallbackContext,
		output: &mut ModuleOutputBuilder,
		packet: &mut Packet,
		acknowledgement: &GenericAcknowledgement,
		relayer: &Signer,
	) -> Result<(), Error> {
		self.inner
			.on_acknowledgement_packet(ctx, output, packet, acknowledgement, relayer)
	}

	fn on_timeout_packet(
		&mut self,
		ctx: &dyn ModuleCallbackContext,
		output: &mut ModuleOutputBuilder,
		packet: &mut Packet,
		relayer: &Signer,
	) -> Result<(), Error> {
		self.inner.on_timeout_packet(ctx, output, packet, relayer)
	}
}

impl<T: Config + Send + Sync, S: Module + Clone + Default + PartialEq + Eq + Debug> Memo<T, S> {
	fn process_memo(packet: &mut Packet) -> Result<(), Error> {
		let packet_data: PacketData =
			serde_json::from_slice(packet.data.as_slice()).map_err(|e| {
				Error::implementation_specific(format!("Failed to decode packet data {:?}", e))
			})?;
		let receiver = <T as Config>::AccountIdConversion::try_from(packet_data.receiver.clone())
			.map_err(|_| {
				Error::implementation_specific(format!("Failed to parse receiver account"))
			})?
			.into_account();
		<T as Config>::HandleMemo::execute_memo(&packet_data, receiver).map_err(|e| {
			Error::implementation_specific(format!("Failed to execute memo {:?}", e))
		})?;
		Ok(())
	}
}
