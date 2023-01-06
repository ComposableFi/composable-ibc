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
use sp_runtime::traits::Get;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::OriginFor;
	use ibc_primitives::IbcAccount;
	use sp_runtime::{traits::Get, Percent};

	#[pallet::config]
	pub trait Config: frame_system::Config + crate::Config {
		#[pallet::constant]
		type ServiceCharge: Get<Percent>;
		type PalletAccount: Get<IbcAccount<Self::AccountId>>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub type ServiceCharge<T: Config> = StorageValue<_, Percent, OptionQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn set_charge(origin: OriginFor<T>, charge: Percent) -> DispatchResult {
			<T as crate::Config>::AdminOrigin::ensure_origin(origin)?;
			<ServiceCharge<T>>::put(charge);
			Ok(())
		}
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ics20ServiceCharge<T: Config> {
	pub inner: crate::ics20::IbcModule<T>,
}

impl<T: Config> Default for Ics20ServiceCharge<T> {
	fn default() -> Self {
		Self { inner: Default::default() }
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
		packet: &Packet,
		relayer: &Signer,
	) -> Result<Acknowledgement, Ics04Error> {
		// Module ModuleCallbackContext does not have the ics20 context as part of its trait bounds
		// so we define a new context
		let mut ctx = Context::<T>::default();
		let packet_data =
			serde_json::from_slice::<PacketData>(packet.data.as_slice()).map_err(|e| {
				Ics04Error::implementation_specific(format!("Failed to decode packet data {:?}", e))
			})?;
		let percent = ServiceCharge::<T>::get().unwrap_or(T::ServiceCharge::get());
		// Send full amount to receiver using the default ics20 logic
		let ack = self.inner.on_recv_packet(&mut ctx, output, packet, relayer)?;
		// We only take the fee charge if the acknowledgement is not an error
		if ack.as_ref() ==
			ibc::applications::transfer::acknowledgement::Acknowledgement::success().as_ref()
		{
			// We have ensured that token amounts larger than the max value for a u128 are rejected
			// in the ics20 on_recv_packet callback so we can multiply safely.
			// Percent does Non-Overflowing multiplication so this is infallible
			let fee = percent * packet_data.token.amount.as_u256().low_u128();
			// This account conversion is infallible if the ics20 on_recv_packet callback executed
			// successfully
			let receiver = <T as crate::Config>::AccountIdConversion::try_from(
				packet_data.receiver,
			)
			.map_err(|_| Ics04Error::implementation_specific("Failed to parse account".to_string()))
			.expect(
				"Account Id conversion should not fail, it has been validated in a previous step",
			);
			let pallet_account = T::PalletAccount::get();
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
			// Now we proceed to send the service fee from the receiver's account to the pallet
			// account
			ctx.send_coins(&receiver, &(pallet_account.into()), &prefixed_coin)
				.map_err(|e| Ics04Error::app_module(e.to_string()))?;
		}
		Ok(ack)
	}

	fn on_acknowledgement_packet(
		&mut self,
		ctx: &dyn ModuleCallbackContext,
		output: &mut ModuleOutputBuilder,
		packet: &Packet,
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
		packet: &Packet,
		relayer: &Signer,
	) -> Result<(), Ics04Error> {
		self.inner.on_timeout_packet(ctx, output, packet, relayer)
	}
}
