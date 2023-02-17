use core::marker::PhantomData;

use composable_traits::dex::Amm;
use ibc::applications::transfer::{Amount, Coin, PrefixedDenom};
use sp_runtime::Percent;

pub struct FeeCharger<Amm> {
	// default_fee is the fallback percentage to be charged in case
	// it's not possible to get prices of the asset.
	default_fee: Percent,
	_p: PhantomData<Amm>,
}

pub struct ChargeFeeOutput {
	pub prefixed_coin_fee: Coin<PrefixedDenom>,
	pub remaining_amount: Amount,
}

impl<A: Amm> FeeCharger<A> {
	pub fn new(default_fee: Percent) -> Self {
		Self { default_fee, _p: PhantomData::default() }
	}
	fn pool_exists(pool_id: A::PoolId) -> bool {
		A::pool_exists(pool_id)
	}

	pub fn charge_fee(&self, packet_amount: &Coin<PrefixedDenom>) -> ChargeFeeOutput {
		// Fee _algorithm_:
		// - if there is a Pool in Pablo, then swap
		// - if no Pool in Pablo, get percentage fee

		// TODO(vim): handle the case in which the direct pool with USD{T, C} does not exist
		// through routing. Is it maybe done automatically?
		let mut prefixed_coin_fee = packet_amount.clone();
		let mut packet_remaining_amount = packet_amount;

		//SAFETY: We have ensured that token amounts larger than the max value for a u128 are
		// rejected in the ics20 on_recv_packet callback so we can multiply safely.
		// Percent does Non-Overflowing multiplication so this is infallible
		let fee = self.default_fee * packet_amount.amount.as_u256().as_u128();
		prefixed_coin_fee.amount = fee.into();
		ChargeFeeOutput {
			prefixed_coin_fee,
			remaining_amount: (packet_remaining_amount.amount.as_u256().as_u128() - fee).into(),
		}
	}
}
