use crate::routing::Context;
use alloc::{format, string::ToString};
use codec::{Decode, Encode};
use core::fmt::Debug;
use ibc::{
	applications::transfer::{is_receiver_chain_source, packet::PacketData},
	bigint::U256,
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
use ibc_primitives::IbcAccount;
use scale_info::TypeInfo;
use sp_core::crypto::AccountId32;

pub use pallet::*;

#[derive(Encode, Decode, TypeInfo)]
pub struct QuotaTracker {
	// defines the number of blocks for which the current quota applies
	pub current_quota_interval: u64,
	// defines the first block for which this quota is valid
	pub initial_block: u64,
	// todo: comment
	pub quota_inflow: U256,
	// todo: comment
	pub quota_outflow: U256,
}

pub enum FlowType {
	Inflow,
	Outflow,
}

const QUOTA_INTERVAL: u64 = 100;
/// Keeps track of transferred and received volume in a period of time
/// and sets the available amount that
#[derive(Encode, Decode, TypeInfo)]
pub struct DenomStatePerChannel {
	pub quota_tracker: QuotaTracker,
	pub current_volume_inflow: U256,
	pub current_volume_outflow: U256,
	pub next_period_amplifier_factor_percentage: U256,
}

impl QuotaTracker {
	pub fn new(current_block: u64) -> Self {
		Self {
			current_quota_interval: QUOTA_INTERVAL,
			initial_block: current_block,
			quota_inflow: U256::MAX,
			quota_outflow: U256::MAX, // TODO: how do we parametrize this inside the runtime?
		}
	}
}

/// RateLimiter defines the minimum interface that is expected in order to
/// 1. ensure that there is quota for this transaction
/// 2. the available quota is recalculated
/// 3. it refreshes the parameters whenever it's necessary
pub trait RateLimiter {
	fn update_volume(&mut self, amount: U256, flow_type: &FlowType);
	fn is_transfer_allowed(&self, amount: U256, flow_type: &FlowType) -> bool;
	fn calculate_new_quota_if_needed(&mut self, current_block: u64);
}

impl DenomStatePerChannel {
	pub fn new(quota_tracker: QuotaTracker) -> Self {
		Self {
			quota_tracker,
			next_period_amplifier_factor_percentage: U256::from(15), // we start with 15%
			current_volume_inflow: U256::default(),
			current_volume_outflow: U256::default(),
		}
	}
}

impl RateLimiter for DenomStatePerChannel {
	fn update_volume(&mut self, amount: U256, flow_type: &FlowType) {
		// safety: note that this will not overflow if `self.is_transfer_allowed()` is called
		// before and ensured that it's true
		match flow_type {
			FlowType::Inflow => self.current_volume_inflow += amount,
			FlowType::Outflow => self.current_volume_outflow += amount,
		}
	}

	fn is_transfer_allowed(&self, amount: U256, flow_type: &FlowType) -> bool {
		match flow_type {
			FlowType::Inflow => {
				self.current_volume_inflow.saturating_add(amount) <= self.quota_tracker.quota_inflow
			},
			FlowType::Outflow => {
				self.current_volume_outflow.saturating_add(amount)
					<= self.quota_tracker.quota_outflow
			},
		}
	}

	/// Updates the quota in case there's a new period
	fn calculate_new_quota_if_needed(&mut self, current_block: u64) {
		// if not enough period has elapsed
		if current_block
			<= self.quota_tracker.initial_block + self.quota_tracker.current_quota_interval
		{
			return;
		}

		let next_period_amplifier =
			U256::from(1) + (self.next_period_amplifier_factor_percentage / U256::from(100));
		self.quota_tracker.quota_inflow = self.current_volume_inflow * next_period_amplifier;
		self.quota_tracker.quota_outflow = self.current_volume_outflow * next_period_amplifier;
		self.current_volume_inflow = U256::zero();
		self.current_volume_outflow = U256::zero();
		self.quota_tracker.initial_block = current_block;

		unimplemented!()
	}
}

#[frame_support::pallet]
pub mod pallet {
	use alloc::string::String;
	use frame_support::{pallet_prelude::*, PalletId};
	use ibc_primitives::IbcAccount;
	use sp_runtime::traits::{AccountIdConversion, Get};

	use super::DenomStatePerChannel;

	#[pallet::config]
	pub trait Config: frame_system::Config + crate::Config {
		#[pallet::constant]
		type PalletId: Get<PalletId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub type DenomStatePerChannelStorage<T: Config> =
		StorageMap<_, Blake2_128Concat, String, DenomStatePerChannel, OptionQuery>;

	impl<T: Config> Pallet<T>
	where
		<T as crate::Config>::AccountIdConversion: From<IbcAccount<T::AccountId>>,
	{
		pub fn account_id() -> <T as crate::Config>::AccountIdConversion {
			IbcAccount(T::PalletId::get().into_account_truncating()).into()
		}
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ics20RateLimiter<T: Config, S: Module + Clone + Default + PartialEq + Eq + Debug> {
	inner: S,
	_phantom: core::marker::PhantomData<T>,
}

impl<T: Config + Send + Sync, S: Module + Clone + Default + PartialEq + Eq + Debug> Default
	for Ics20RateLimiter<T, S>
{
	fn default() -> Self {
		Self { inner: S::default(), _phantom: Default::default() }
	}
}

impl<T: Config + Send + Sync, S: Module + Clone + Default + PartialEq + Eq + Debug> Module
	for Ics20RateLimiter<T, S>
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
	AccountId32: From<<T as frame_system::Config>::AccountId>,
	<T as crate::Config>::AccountIdConversion: From<IbcAccount<T::AccountId>>,
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
	) -> Result<(), Ics04Error> {
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
	) -> Result<Version, Ics04Error> {
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
	) -> Result<(), Ics04Error> {
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
	) -> Result<(), Ics04Error> {
		self.inner.on_chan_open_confirm(ctx, output, port_id, channel_id, relayer)
	}

	fn on_chan_close_init(
		&mut self,
		ctx: &dyn ModuleCallbackContext,
		output: &mut ModuleOutputBuilder,
		port_id: &PortId,
		channel_id: &ChannelId,
		relayer: &Signer,
	) -> Result<(), Ics04Error> {
		self.inner.on_chan_close_init(ctx, output, port_id, channel_id, relayer)
	}

	fn on_chan_close_confirm(
		&mut self,
		ctx: &dyn ModuleCallbackContext,
		output: &mut ModuleOutputBuilder,
		port_id: &PortId,
		channel_id: &ChannelId,
		relayer: &Signer,
	) -> Result<(), Ics04Error> {
		self.inner.on_chan_close_confirm(ctx, output, port_id, channel_id, relayer)
	}

	fn on_recv_packet(
		&self,
		_ctx: &dyn ModuleCallbackContext,
		output: &mut ModuleOutputBuilder,
		packet: &mut Packet,
		relayer: &Signer,
	) -> Result<Acknowledgement, Ics04Error> {
		// Module ModuleCallbackContext does not have the ics20 context as part of its trait bounds
		// so we define a new context
		let mut ctx = Context::<T>::default();
		let packet_data =
			serde_json::from_slice::<PacketData>(packet.data.as_slice()).map_err(|e| {
				Ics04Error::implementation_specific(format!("Failed to decode packet data {:?}", e))
			})?;

		let flow_type = if is_receiver_chain_source(
			packet.source_port.clone(),
			packet.source_channel,
			&packet_data.token.denom,
		) {
			FlowType::Inflow
		} else {
			FlowType::Outflow
		};

		let current_block_number: u32 = <frame_system::Pallet<T>>::block_number().into();
		// 1. check if the denom on that channel is being tracker, and otherwise start trackign it
		let mut denom_state_current_channel =
			DenomStatePerChannelStorage::<T>::get(packet_data.token.denom.to_string())
				.unwrap_or_else(|| {
					let current_block_number = <frame_system::Pallet<T>>::block_number();
					DenomStatePerChannel::new(QuotaTracker::new(
						u32::from(current_block_number) as _
					))
				});

		let token_amount = packet_data.token.amount.into();
		// 2. check if there is quota for the denom on this channel
		if !denom_state_current_channel.is_transfer_allowed(token_amount, &flow_type) {
			// TODO: do we want to return an error, or is it better to send an Acknowledgement?
			return Err(Ics04Error::implementation_specific("Quota exceeded".to_string()));
		}
		// 3. if there's quota, update the volume
		denom_state_current_channel.update_volume(token_amount, &flow_type);
		denom_state_current_channel.calculate_new_quota_if_needed(current_block_number as _);
		// 4. update the quota if necessary (in case of a new period)
		let ack = self.inner.on_recv_packet(&mut ctx, output, packet, relayer)?;
		Ok(ack)
	}

	fn on_acknowledgement_packet(
		&mut self,
		ctx: &dyn ModuleCallbackContext,
		output: &mut ModuleOutputBuilder,
		packet: &mut Packet,
		acknowledgement: &Acknowledgement,
		relayer: &Signer,
	) -> Result<(), Ics04Error> {
		self.inner
			.on_acknowledgement_packet(ctx, output, packet, acknowledgement, relayer)
	}

	fn on_timeout_packet(
		&mut self,
		ctx: &dyn ModuleCallbackContext,
		output: &mut ModuleOutputBuilder,
		packet: &mut Packet,
		relayer: &Signer,
	) -> Result<(), Ics04Error> {
		self.inner.on_timeout_packet(ctx, output, packet, relayer)
	}
}

