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

use crate::routing::Context;
use alloc::{format, string::ToString};
use core::fmt::Formatter;
use ibc::{
	applications::transfer::{
		context::BankKeeper, is_receiver_chain_source, packet::PacketData, TracePrefix,
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
use ibc_primitives::IbcAccount;
use sp_core::crypto::AccountId32;
use sp_runtime::traits::{AccountIdConversion, Get};
use sp_std::marker::PhantomData;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, PalletId};
	use frame_system::pallet_prelude::OriginFor;
	use sp_runtime::{traits::Get, Perbill};

	#[pallet::config]
	pub trait Config: frame_system::Config + crate::Config {
		#[pallet::constant]
		type ServiceCharge: Get<Perbill>;
		#[pallet::constant]
		type PalletId: Get<PalletId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	pub type ServiceCharge<T: Config> = StorageValue<_, Perbill, OptionQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn set_charge(origin: OriginFor<T>, charge: u32) -> DispatchResult {
			<T as crate::Config>::AdminOrigin::ensure_origin(origin)?;
			<ServiceCharge<T>>::put(Perbill::from_percent(charge));
			Ok(())
		}
	}
}

#[derive(Clone, PartialEq, Eq)]
pub struct Ics20ServiceCharge<T: Config> {
	pub inner: crate::ics20::IbcModule<T>,
	pub _phantom: PhantomData<T>,
}

impl<T: Config> Default for Ics20ServiceCharge<T> {
	fn default() -> Self {
		Self { inner: <crate::ics20::IbcModule<T>>::default(), _phantom: Default::default() }
	}
}

impl<T: Config> core::fmt::Debug for Ics20ServiceCharge<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		write!(f, "ics20-service-charge")
	}
}

impl<T: Config + Send + Sync> Module for Ics20ServiceCharge<T>
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
	AccountId32: From<<T as frame_system::Config>::AccountId>,
	<T as crate::Config>::AccountIdConversion: From<IbcAccount<T::AccountId>>,
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
		relayer: &Signer,
	) -> Result<(), Ics04Error> {
		let mut ctx = Context::<T>::default();
		self.inner.on_chan_open_init(
			&mut ctx,
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
		_ctx: &dyn ModuleCallbackContext,
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
		let mut ctx = Context::<T>::default();
		self.inner.on_chan_open_try(
			&mut ctx,
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
		_ctx: &dyn ModuleCallbackContext,
		output: &mut ModuleOutputBuilder,
		port_id: &PortId,
		channel_id: &ChannelId,
		counterparty_version: &Version,
		relayer: &Signer,
	) -> Result<(), Ics04Error> {
		let mut ctx = Context::<T>::default();
		self.inner.on_chan_open_ack(
			&mut ctx,
			output,
			port_id,
			channel_id,
			counterparty_version,
			relayer,
		)
	}

	fn on_chan_open_confirm(
		&mut self,
		_ctx: &dyn ModuleCallbackContext,
		output: &mut ModuleOutputBuilder,
		port_id: &PortId,
		channel_id: &ChannelId,
		relayer: &Signer,
	) -> Result<(), Ics04Error> {
		let mut ctx = Context::<T>::default();
		self.inner.on_chan_open_confirm(&mut ctx, output, port_id, channel_id, relayer)
	}

	fn on_chan_close_init(
		&mut self,
		_ctx: &dyn ModuleCallbackContext,
		output: &mut ModuleOutputBuilder,
		port_id: &PortId,
		channel_id: &ChannelId,
		relayer: &Signer,
	) -> Result<(), Ics04Error> {
		let mut ctx = Context::<T>::default();
		self.inner.on_chan_close_init(&mut ctx, output, port_id, channel_id, relayer)
	}

	fn on_chan_close_confirm(
		&mut self,
		_ctx: &dyn ModuleCallbackContext,
		output: &mut ModuleOutputBuilder,
		port_id: &PortId,
		channel_id: &ChannelId,
		relayer: &Signer,
	) -> Result<(), Ics04Error> {
		let mut ctx = Context::<T>::default();
		self.inner.on_chan_close_confirm(&mut ctx, output, port_id, channel_id, relayer)
	}

	fn on_recv_packet(
		&self,
		_ctx: &dyn ModuleCallbackContext,
		output: &mut ModuleOutputBuilder,
		packet: &Packet,
		relayer: &Signer,
	) -> Result<Acknowledgement, Ics04Error> {
		let mut ctx = Context::<T>::default();
		let packet_data =
			serde_json::from_slice::<PacketData>(packet.data.as_slice()).map_err(|e| {
				Ics04Error::implementation_specific(format!("Failed to decode packet data {:?}", e))
			})?;
		let percent = ServiceCharge::<T>::get().unwrap_or(T::ServiceCharge::get());
		let fee = percent * packet_data.token.amount.as_u256().as_u128();
		// Send full amount to receiver
		let ack = self.inner.on_recv_packet(&mut ctx, output, packet, relayer)?;
		// Send fee from the receiver's account to the pallet account
		let receiver = <T as crate::Config>::AccountIdConversion::try_from(packet_data.receiver)
			.map_err(|_| Ics04Error::implementation_specific("Failed to parse account".to_string()))
			.expect(
				"Account Id conversion should not fail, it has been validated in a previous step",
			);
		let pallet_account: T::AccountId = T::PalletId::get().into_account_truncating();
		let pallet_account = IbcAccount(pallet_account);
		let prefixed_coin = if is_receiver_chain_source(
			packet.source_port.clone(),
			packet.source_channel,
			&packet_data.token.denom,
		) {
			let prefix = TracePrefix::new(packet.source_port.clone(), packet.source_channel);
			let mut c = packet_data.token;
			c.denom.remove_trace_prefix(&prefix);
			c.amount = fee.into();
			c
		} else {
			let prefix =
				TracePrefix::new(packet.destination_port.clone(), packet.destination_channel);
			let mut c = packet_data.token;
			c.denom.add_trace_prefix(prefix);
			c.amount = fee.into();
			c
		};

		ctx.send_coins(&receiver, &(pallet_account.into()), &prefixed_coin)
			.map_err(|e| Ics04Error::app_module(e.to_string()))?;
		Ok(ack)
	}

	fn on_acknowledgement_packet(
		&mut self,
		_ctx: &dyn ModuleCallbackContext,
		output: &mut ModuleOutputBuilder,
		packet: &Packet,
		acknowledgement: &Acknowledgement,
		relayer: &Signer,
	) -> Result<(), Ics04Error> {
		let mut ctx = Context::<T>::default();
		self.inner
			.on_acknowledgement_packet(&mut ctx, output, packet, acknowledgement, relayer)
	}

	fn on_timeout_packet(
		&mut self,
		_ctx: &dyn ModuleCallbackContext,
		output: &mut ModuleOutputBuilder,
		packet: &Packet,
		relayer: &Signer,
	) -> Result<(), Ics04Error> {
		let mut ctx = Context::<T>::default();
		self.inner.on_timeout_packet(&mut ctx, output, packet, relayer)
	}
}
