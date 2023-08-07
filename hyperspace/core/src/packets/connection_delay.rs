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

use ibc::{timestamp::Timestamp, Height};
use primitives::error::Error;
use std::time::Duration;

/// Verify the time and height delays
pub fn has_delay_elapsed(
	current_time: Timestamp,
	current_height: Height,
	client_update_time: Timestamp,
	client_update_height: Height,
	delay_period_time: Duration,
	delay_period_blocks: u64,
) -> Result<bool, anyhow::Error> {
	log::trace!(target: "hyperspace",
		"Checking if delay has elapsed: current_time: {}, current_height: {}, client_update_time: {}, client_update_height: {}, delay_period_time: {:?}, delay_period_blocks: {}",
		current_time, current_height, client_update_time, client_update_height, delay_period_time, delay_period_blocks
	);
	let earliest_time = (client_update_time + delay_period_time)
		.map_err(|_| Error::Custom("Timestamp overflow".to_string()))?;
	if !(current_time == earliest_time || current_time.after(&earliest_time)) {
		return Ok(false)
	}

	let earliest_height = client_update_height.add(delay_period_blocks);
	if current_height < earliest_height {
		return Ok(false)
	}

	Ok(true)
}
