use super::*;
use crate::errors::IbcError;
use ibc::{
	core::ics26_routing::error::Error as RoutingError,
	events::{IbcEvent as RawIbcEvent, IbcEvent},
};

impl<T: Config> From<Vec<RawIbcEvent>> for Event<T> {
	fn from(events: Vec<RawIbcEvent>) -> Self {
		let events: Vec<Result<IbcEvent, _>> = events.into_iter().map(|ev| Ok(ev.into())).collect();
		Event::Events { events }
	}
}

impl<T: Config> From<Vec<Result<RawIbcEvent, RoutingError>>> for Event<T> {
	fn from(events: Vec<Result<RawIbcEvent, RoutingError>>) -> Self {
		let events: Vec<Result<IbcEvent, IbcError>> = events
			.into_iter()
			.map(|result| result.map(|ev| ev.into()).map_err(|err| err.into()))
			.collect();
		Event::Events { events }
	}
}
