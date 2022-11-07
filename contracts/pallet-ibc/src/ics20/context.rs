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

use super::super::*;
use crate::routing::Context;
use frame_support::traits::{
	fungibles::{Mutate, Transfer},
	Currency, Get,
};
use ibc::{
	applications::transfer::{
		context::{BankKeeper, Ics20Context, Ics20Keeper, Ics20Reader},
		error::Error as Ics20Error,
		PORT_ID_STR,
	},
	core::ics24_host::identifier::{ChannelId, PortId},
};
use ibc_primitives::get_channel_escrow_address;
use sp_runtime::traits::IdentifyAccount;

impl<T: Config + Send + Sync> Ics20Reader for Context<T>
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
{
	type AccountId = T::AccountIdConversion;

	fn get_port(&self) -> Result<PortId, Ics20Error> {
		PortId::from_str(PORT_ID_STR)
			.map_err(|e| Ics20Error::invalid_port_id(PORT_ID_STR.to_string(), e))
	}

	fn get_channel_escrow_address(
		&self,
		port_id: &PortId,
		channel_id: ChannelId,
	) -> Result<<Self as Ics20Reader>::AccountId, Ics20Error> {
		get_channel_escrow_address(port_id, channel_id)?.try_into().map_err(|_| {
			log::trace!(target: "pallet_ibc", "Failed to get channel escrow address");
			Ics20Error::parse_account_failure()
		})
	}

	fn is_send_enabled(&self) -> bool {
		Pallet::<T>::is_send_enabled()
	}

	fn is_receive_enabled(&self) -> bool {
		Pallet::<T>::is_receive_enabled()
	}
}

impl<T: Config + Send + Sync> Ics20Keeper for Context<T>
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
{
	type AccountId = T::AccountIdConversion;
}

impl<T: Config + Send + Sync> Ics20Context for Context<T>
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
{
	type AccountId = T::AccountIdConversion;
}

impl<T> BankKeeper for Context<T>
where
	T: Config + Send + Sync,
	u32: From<<T as frame_system::Config>::BlockNumber>,
{
	type AccountId = T::AccountIdConversion;

	fn send_coins(
		&mut self,
		from: &Self::AccountId,
		to: &Self::AccountId,
		amt: &ibc::applications::transfer::PrefixedCoin,
	) -> Result<(), Ics20Error> {
		let amount: T::Balance = amt.amount.as_u256().low_u128().into();
		let denom = amt.denom.to_string();
		// Token should be registered already if sending an ibc asset
		let asset_id = T::IbcDenomToAssetIdConversion::from_denom_to_asset_id(&denom)
			.map_err(|_| Ics20Error::invalid_token())?;
		if asset_id == T::NativeAssetId::get() {
			<T::NativeCurrency as Currency<T::AccountId>>::transfer(
				&from.clone().into_account(),
				&to.clone().into_account(),
				amount.into(),
				frame_support::traits::ExistenceRequirement::AllowDeath,
			)
			.map_err(|e| {
				log::trace!(target: "pallet_ibc", "Failed to transfer ibc tokens: {:?}", e);
				Ics20Error::invalid_token()
			})?;
		} else {
			<<T as Config>::Fungibles as Transfer<T::AccountId>>::transfer(
				asset_id.into(),
				&from.clone().into_account(),
				&to.clone().into_account(),
				amount,
				false,
			)
			.map_err(|e| {
				log::trace!(target: "pallet_ibc", "Failed to transfer ibc tokens: {:?}", e);
				Ics20Error::invalid_token()
			})?;
		}

		Ok(())
	}

	fn mint_coins(
		&mut self,
		account: &Self::AccountId,
		amt: &ibc::applications::transfer::PrefixedCoin,
	) -> Result<(), Ics20Error> {
		let amount: T::Balance = amt.amount.as_u256().low_u128().into();
		let denom = amt.denom.to_string();
		// Find existing asset or create a new one
		let asset_id =
			T::IbcDenomToAssetIdConversion::from_denom_to_asset_id(&denom).map_err(|err| {
				Ics20Error::implementation_specific(format!(
					"Failed to create or find asset: {err:?}"
				))
			})?;

		<<T as Config>::Fungibles as Mutate<T::AccountId>>::mint_into(
			asset_id.into(),
			&account.clone().into_account(),
			amount,
		)
		.map_err(|e| {
			log::trace!(target: "pallet_ibc", "Failed to mint tokens: {:?}", e);
			Ics20Error::invalid_token()
		})?;
		Ok(())
	}

	fn burn_coins(
		&mut self,
		account: &Self::AccountId,
		amt: &ibc::applications::transfer::PrefixedCoin,
	) -> Result<(), Ics20Error> {
		let amount: T::Balance = amt.amount.as_u256().low_u128().into();
		let denom = amt.denom.to_string();
		// Token should be registered already if burning a voucher
		let asset_id = T::IbcDenomToAssetIdConversion::from_denom_to_asset_id(&denom)
			.map_err(|_| Ics20Error::invalid_token())?;
		<<T as Config>::Fungibles as Mutate<T::AccountId>>::burn_from(
			asset_id.into(),
			&account.clone().into_account(),
			amount,
		)
		.map_err(|e| {
			log::trace!(target: "pallet_ibc", "Failed to burn tokens: {:?}", e);
			Ics20Error::implementation_specific("Failed to burn tokens".to_string())
		})?;
		Ok(())
	}
}
