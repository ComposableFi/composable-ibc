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

use crate::{
	applications::transfer::{
		acknowledgement::Acknowledgement, context::Ics20Context, error::Error as Ics20Error,
		packet::PacketData, relay::refund_packet_token,
	},
	core::ics04_channel::packet::Packet,
};

pub fn process_ack_packet(
	ctx: &mut impl Ics20Context,
	packet: &Packet,
	data: &PacketData,
	ack: &Acknowledgement,
) -> Result<(), Ics20Error> {
	if !ack.is_successful() {
		refund_packet_token(ctx, packet, data)?;
	}
	Ok(())
}
