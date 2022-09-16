use super::types::CryptoHash;
use crate::ics11_beefy::client_state::ClientState;
use flex_error::define_error;
use ibc::core::ics02_client::error::Error as Ics02Error;

define_error! {
    #[derive(Debug, PartialEq, Eq)]
    Error {
        InvalidEpoch
        { epoch_id: CryptoHash }
        | _ | { "invalid epoch id" },
        HeightTooOld
        | _ | { format_args!(
            "height too old")
        },
        InvalidSignature
        | _ | { format_args!(
            "invalid signature")
        },
        InsufficientStakedAmount
        | _ | { format_args!(
            "insufficient staked amount")
        },
        SerializationError
        | _ | { format_args!(
            "serialization error")
        },
        UnavailableBlockProducers
        | _ | { format_args!(
            "unavailable block producers")
        },
    }
}

impl From<Error> for Ics02Error {
    fn from(e: Error) -> Self {
        Ics02Error::client_error(ClientState::<()>::client_type().to_owned(), e.to_string())
    }
}
