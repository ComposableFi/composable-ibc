use core::marker::PhantomData;

use composable_traits::dex::Amm;

pub struct FeeCharger<Amm> {
	_p: PhantomData<Amm>,
}

impl<A: Amm> FeeCharger<A> {
	fn pool_exists(pool_id: A::PoolId) -> bool {
		A::pool_exists(pool_id)
	}

	pub fn charge_fee() {
		// Fee _algorithm_:
		// - if there is a Pool in Pablo, then swap
		// - if no Pool in Pablo, get percentage fee

		// TODO(vim): handle the case in which the direct pool with USD{T, C} does not exist
		// through routing. Is it maybe done automatically?
	}
}
