#![feature(prelude_import)]
#![allow(dead_code)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
extern crate alloc;
extern crate core;
mod context {
    use core::str::FromStr;
    use cosmwasm_std::{Api, Deps, DepsMut, Env, Storage};
    use crate::{ibc, state};
    type Result<T, E = crate::Error> = core::result::Result<T, E>;
    /// Base context for handling CosmWasm operations.
    ///
    /// It wraps together access to CosmWasm API and information about the request
    /// such as block height, current time and IBC client id corresponding to this
    /// contract.
    ///
    /// The object dereferences into [`state::Metadata`] such that metadata fields
    /// are directly accessible through this object.
    pub(crate) struct ContextBase<'a> {
        #[deref]
        pub metadata: state::Metadata,
        pub client_id: ibc::ClientId,
        pub api: &'a dyn Api,
    }
    impl<'a> ::core::ops::Deref for ContextBase<'a> {
        type Target = state::Metadata;
        #[inline]
        fn deref(&self) -> &Self::Target {
            &self.metadata
        }
    }
    /// Mutable execution context for handling CosmWasm operations.
    ///
    /// It wraps together access to CosmWasm APIs, storage and information about the
    /// request.  To construct a new context object use [`new`] function.
    pub(crate) struct ContextMut<'a> {
        #[deref]
        base: ContextBase<'a>,
        storage: &'a mut dyn Storage,
    }
    impl<'a> ::core::ops::Deref for ContextMut<'a> {
        type Target = ContextBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            &self.base
        }
    }
    /// Constructs a new mutable execution context.
    pub(crate) fn new<'a>(deps: DepsMut<'a>, env: Env) -> ContextMut<'a> {
        ContextMut {
            base: ContextBase::new(env, deps.api),
            storage: deps.storage,
        }
    }
    /// Read-only execution context for handling CosmWasm operations.
    ///
    /// It wraps together access to CosmWasm APIs, storage and information about the
    /// request.  To construct a new context object use [`new_ro`] function.
    ///
    /// The object dereferences into [`ContextBase`] which holds data common between
    /// read-only and mutable execution contexts.
    pub(crate) struct Context<'a> {
        #[deref]
        base: ContextBase<'a>,
        storage: &'a dyn Storage,
    }
    impl<'a> ::core::ops::Deref for Context<'a> {
        type Target = ContextBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            &self.base
        }
    }
    /// Constructs a new read-only execution context.
    pub(crate) fn new_ro<'a>(deps: Deps<'a>, env: Env) -> Context<'a> {
        Context {
            base: ContextBase::new(env, deps.api),
            storage: deps.storage,
        }
    }
    impl<'a> ContextBase<'a> {
        fn new(env: Env, api: &'a dyn Api) -> Self {
            let metadata = state::Metadata {
                host_timestamp_ns: env.block.time.nanos(),
                host_height: env.block.height,
            };
            let address = env.contract.address.as_str();
            let client_id = ibc::ClientId::from_str(address).unwrap();
            Self {
                client_id,
                metadata,
                api,
            }
        }
        pub fn log(&self, msg: impl alloc::string::ToString) {
            self.api.debug(&msg.to_string())
        }
    }
    pub(crate) use log;
    impl<'a> Context<'a> {
        /// Reads this light client’s client state from storage.
        pub fn client_state(&self) -> Result<state::ClientState> {
            req_client_state(&self.client_id, self.client_states().get())
        }
        /// Returns object providing access to read client state from the
        /// storage.
        pub fn client_states(&self) -> &'a state::ClientStates {
            state::ClientStates::new_ro(self.storage)
        }
        /// Reads this light client’s consensus state at given height from
        /// storage.
        pub fn consensus_state(&self, height: ibc::Height) -> Result<state::ConsensusState> {
            req_consensus_state(&self.client_id, height, self.consensus_states().get(height))
        }
        /// Returns object providing access to read consensus states from the
        /// storage.
        pub fn consensus_states(&self) -> &'a state::ConsensusStates {
            state::ConsensusStates::new_ro(self.storage)
        }
    }
    impl<'a> ContextMut<'a> {
        /// Reads this light client’s client state from storage.
        pub fn client_state(&self) -> Result<state::ClientState> {
            req_client_state(&self.client_id, self.client_states().get())
        }
        /// Returns object providing access to read client state from the
        /// storage.
        pub fn client_states(&self) -> &state::ClientStates {
            state::ClientStates::new_ro(self.storage)
        }
        /// Returns object providing access to read or write client state
        /// from/to the storage.
        pub fn client_states_mut(&mut self) -> &mut state::ClientStates {
            state::ClientStates::new(self.storage)
        }
        /// Reads this light client’s consensus state at given height from
        /// storage.
        pub fn consensus_state(&self, height: ibc::Height) -> Result<state::ConsensusState> {
            req_consensus_state(&self.client_id, height, self.consensus_states().get(height))
        }
        /// Returns object providing access to read consensus states from the
        /// storage.
        pub fn consensus_states(&self) -> &state::ConsensusStates {
            state::ConsensusStates::new_ro(self.storage)
        }
        /// Returns object providing access to read or write consensus states
        /// from/to the storage.
        pub fn consensus_states_mut(&mut self) -> &mut state::ConsensusStates {
            state::ConsensusStates::new(self.storage)
        }
    }
    /// Returns an error if client state is not present.
    fn req_client_state(
        client_id: &ibc::ClientId,
        state: Result<Option<state::ClientState>>,
    ) -> Result<state::ClientState> {
        let make_err = || ibc::ClientError::client_not_found(client_id.clone()).into();
        state.and_then(|state| state.ok_or_else(make_err))
    }
    /// Returns an error if consensus state is not present.
    fn req_consensus_state(
        client_id: &ibc::ClientId,
        height: ibc::Height,
        state: Result<Option<(state::ConsensusState, state::Metadata)>>,
    ) -> Result<state::ConsensusState> {
        let make_err =
            || ibc::ClientError::consensus_state_not_found(client_id.clone(), height).into();
        state.and_then(|state| state.map(|(state, _metadata)| state).ok_or_else(make_err))
    }
}
mod contract {
    use cosmwasm_std::{
        to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
        Uint64,
    };
    use prost::Message;
    use crate::{context, context::log, crypto::Verifier, ibc, msg, state};
    type Result<T = (), E = crate::error::Error> = core::result::Result<T, E>;
    fn instantiate(
        deps: DepsMut,
        env: Env,
        _info: MessageInfo,
        _msg: msg::InstantiateMsg,
    ) -> Result<Response> {
        Ok(Response::default())
    }
    fn sudo(deps: DepsMut, env: Env, msg: msg::SudoMsg) -> Result<Response> {
        let mut ctx = context::new(deps, env);
        ctx.log(format_args!("sudo({0:?})", msg));
        match msg {
            msg::SudoMsg::UpdateStateOnMisbehaviour(_msg) => {
                let client_state = ctx.client_state()?.frozen();
                ctx.client_states_mut().set(client_state);
            }
            msg::SudoMsg::UpdateState(msg) => process_update_state_msg(ctx, msg)?,
        }
        Ok(Response::default())
    }
    fn process_update_state_msg(mut ctx: context::ContextMut, msg: msg::UpdateStateMsg) -> Result {
        let client_state = ctx.client_state()?;
        let now_ns = ctx.host_timestamp_ns;
        ctx.consensus_states_mut()
            .prune_oldest_consensus_state(&client_state, now_ns)?;
        let new_consensus_state = state::ConsensusState::from(&msg.header);
        let new_client_state = client_state.with_header(&msg.header);
        let metadata = ctx.metadata;
        let height = ibc::Height::new(0, msg.header.block_header.block_height.into());
        ctx.client_states_mut().set(&new_client_state);
        ctx.consensus_states_mut()
            .set(height, &new_consensus_state, metadata);
        Ok(())
    }
    fn query(deps: Deps, env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
        let ctx = context::new_ro(deps, env);
        match msg {
            msg::QueryMsg::VerifyClientMessage(msg) => {
                query_verify_client_msg(ctx, msg)?;
                to_json_binary(&())
            }
            msg::QueryMsg::CheckForMisbehaviour(msg) => {
                let res = query_check_for_misbehaviour_msg(ctx, msg)?;
                to_json_binary(&res)
            }
            msg::QueryMsg::VerifyStateProof(msg) => {
                query_verify_state_proof(ctx, msg)?;
                to_json_binary(&())
            }
            msg::QueryMsg::Status(msg::StatusMsg {}) => to_json_binary(&query_status(ctx)?),
            msg::QueryMsg::TimestampAtHeight(msg) => {
                let height = msg.height.try_into()?;
                let state = ctx.consensus_state(height)?;
                to_json_binary(&Uint64::from(state.timestamp_ns.get()))
            }
            msg::QueryMsg::ExportMetadata(msg::ExportMetadataMsg {}) => {
                let meta = ctx.consensus_states().get_all_metadata()?;
                to_json_binary(&meta)
            }
        }
    }
    fn query_verify_state_proof(
        ctx: context::Context,
        msg: msg::VerifyStateProofMsg,
    ) -> StdResult<()> {
        let height = msg.height.try_into()?;
        let consensus_state = ctx.consensus_state(height)?;
        cf_guest::proof::verify(
            &ibc::CommitmentPrefix::default(),
            &msg.proof,
            &consensus_state.block_hash,
            msg.path,
            msg.value.as_deref(),
        )
        .map_err(|err| StdError::GenericErr {
            msg: err.to_string(),
        })
    }
    fn query_verify_client_msg(ctx: context::Context, msg: msg::VerifyClientMessageMsg) -> Result {
        let client_message =
            ibc::proto::google::protobuf::Any::decode(msg.client_message.as_slice())
                .map_err(crate::Error::from)?;
        ctx.client_state()?
            .verify_client_message(
                &Verifier,
                &ctx.client_id,
                client_message.try_into().unwrap(),
            )
            .map_err(crate::Error::from)
    }
    fn query_check_for_misbehaviour_msg(
        ctx: context::Context,
        msg: msg::CheckForMisbehaviourMsg,
    ) -> Result<bool> {
        let client_message =
            ibc::proto::google::protobuf::Any::decode(msg.client_message.as_slice())
                .map_err(crate::Error::from)?;
        ctx.client_state()?
            .check_for_misbehaviour(&Verifier, &ctx.client_id, client_message)
            .map_err(crate::Error::from)
    }
    fn query_status(ctx: context::Context) -> StdResult<msg::StatusResponse> {
        let client_state = ctx.client_state()?;
        if client_state.is_frozen {
            return Ok(msg::StatusResponse::Frozen);
        }
        let height = client_state.latest_height;
        let height = ibc::Height::new(0, height.into());
        let consensus_state = ctx.consensus_state(height)?;
        let age = ctx
            .host_timestamp_ns
            .saturating_sub(consensus_state.timestamp_ns.get());
        Ok(if age >= client_state.trusting_period_ns {
            msg::StatusResponse::Expired
        } else {
            msg::StatusResponse::Active
        })
    }
}
mod crypto {
    use borsh::maybestd::io;
    /// Ed25519 public key (a.k.a. verifying key).
    #[repr(transparent)]
    pub struct PubKey(ed25519_dalek::VerifyingKey);
    #[automatically_derived]
    impl ::core::clone::Clone for PubKey {
        #[inline]
        fn clone(&self) -> PubKey {
            PubKey(::core::clone::Clone::clone(&self.0))
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PubKey {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "PubKey", &&self.0)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for PubKey {}
    #[automatically_derived]
    impl ::core::cmp::Eq for PubKey {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<ed25519_dalek::VerifyingKey>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for PubKey {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for PubKey {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for PubKey {
        #[inline]
        fn eq(&self, other: &PubKey) -> bool {
            self.0 == other.0
        }
    }
    impl guestchain::PubKey for PubKey {
        type Signature = Signature;
        fn to_vec(&self) -> Vec<u8> {
            self.0.as_bytes().to_vec()
        }
        fn from_bytes(bytes: &[u8]) -> Result<Self, guestchain::BadFormat> {
            bytes
                .try_into()
                .map(Self)
                .map_err(|_| guestchain::BadFormat)
        }
    }
    impl borsh::BorshSerialize for PubKey {
        fn serialize<W: io::Write>(&self, wr: &mut W) -> io::Result<()> {
            wr.write_all(self.0.as_bytes())
        }
    }
    impl borsh::BorshDeserialize for PubKey {
        fn deserialize_reader<R: io::Read>(rd: &mut R) -> io::Result<Self> {
            let mut bytes = ed25519_dalek::pkcs8::PublicKeyBytes([0; 32]);
            rd.read_exact(&mut bytes.0[..])?;
            ed25519_dalek::VerifyingKey::try_from(bytes)
                .map(Self)
                .map_err(|_| io::Error::new(io::ErrorKind::Other, "malformed Ed25519 public key"))
        }
    }
    impl PartialOrd for PubKey {
        fn partial_cmp(&self, rhs: &Self) -> Option<core::cmp::Ordering> {
            Some(self.cmp(rhs))
        }
    }
    impl Ord for PubKey {
        fn cmp(&self, rhs: &Self) -> core::cmp::Ordering {
            self.0.as_bytes().cmp(rhs.0.as_bytes())
        }
    }
    /// Ed25519 signature.
    #[repr(transparent)]
    pub struct Signature(ed25519_dalek::Signature);
    #[automatically_derived]
    impl ::core::clone::Clone for Signature {
        #[inline]
        fn clone(&self) -> Signature {
            Signature(::core::clone::Clone::clone(&self.0))
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Signature {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Signature {
        #[inline]
        fn eq(&self, other: &Signature) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Signature {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Signature {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<ed25519_dalek::Signature>;
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Signature {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Signature", &&self.0)
        }
    }
    impl guestchain::Signature for Signature {
        fn to_vec(&self) -> Vec<u8> {
            self.0.to_vec()
        }
        fn from_bytes(bytes: &[u8]) -> Result<Self, guestchain::BadFormat> {
            ed25519_dalek::Signature::from_slice(bytes)
                .map(Self)
                .map_err(|_| guestchain::BadFormat)
        }
    }
    impl borsh::BorshSerialize for Signature {
        fn serialize<W: io::Write>(&self, wr: &mut W) -> io::Result<()> {
            wr.write_all(self.0.r_bytes())?;
            wr.write_all(self.0.s_bytes())?;
            Ok(())
        }
    }
    impl borsh::BorshDeserialize for Signature {
        fn deserialize_reader<R: io::Read>(rd: &mut R) -> io::Result<Self> {
            let mut buf = [0; 64];
            rd.read_exact(&mut buf[..])?;
            Ok(Self(ed25519_dalek::Signature::from_bytes(&buf)))
        }
    }
    impl core::hash::Hash for Signature {
        fn hash<H: core::hash::Hasher>(&self, hasher: &mut H) {
            hasher.write(self.0.r_bytes());
            hasher.write(self.0.s_bytes());
        }
    }
    impl PartialOrd for Signature {
        fn partial_cmp(&self, rhs: &Self) -> Option<core::cmp::Ordering> {
            Some(self.cmp(rhs))
        }
    }
    impl Ord for Signature {
        fn cmp(&self, rhs: &Self) -> core::cmp::Ordering {
            let lhs = (self.0.r_bytes(), self.0.s_bytes());
            let rhs = (rhs.0.r_bytes(), rhs.0.s_bytes());
            lhs.cmp(&rhs)
        }
    }
    /// Verifier for Ed25519 signatures using ed25519-dalek implementation.
    pub(crate) struct Verifier;
    impl guestchain::Verifier<PubKey> for Verifier {
        fn verify(&self, message: &[u8], pubkey: &PubKey, signature: &Signature) -> bool {
            pubkey.0.verify_strict(message, &signature.0).is_ok()
        }
    }
}
mod error {
    use cosmwasm_std::StdError;
    pub enum Error {
        Std(StdError),
        Client(crate::ibc::ClientError),
        BadProto(prost::DecodeError),
        #[from(ignore)]
        BadMessage,
        #[from(ignore)]
        BadType,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Error {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Error::Std(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Std", &__self_0)
                }
                Error::Client(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Client", &__self_0)
                }
                Error::BadProto(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "BadProto", &__self_0)
                }
                Error::BadMessage => ::core::fmt::Formatter::write_str(f, "BadMessage"),
                Error::BadType => ::core::fmt::Formatter::write_str(f, "BadType"),
            }
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(crate::ibc::ClientError)> for Error {
        #[inline]
        fn from(original: (crate::ibc::ClientError)) -> Error {
            Error::Client(original)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(StdError)> for Error {
        #[inline]
        fn from(original: (StdError)) -> Error {
            Error::Std(original)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(prost::DecodeError)> for Error {
        #[inline]
        fn from(original: (prost::DecodeError)) -> Error {
            Error::BadProto(original)
        }
    }
    impl ::core::fmt::Display for Error {
        #[allow(unused_variables)]
        #[inline]
        fn fmt(
            &self,
            _derive_more_display_formatter: &mut ::core::fmt::Formatter,
        ) -> ::core::fmt::Result {
            match self {
                Error::Std(_0) => ::core::fmt::Display::fmt(_0, _derive_more_display_formatter),
                Error::Client(_0) => ::core::fmt::Display::fmt(_0, _derive_more_display_formatter),
                Error::BadProto(_0) => {
                    ::core::fmt::Display::fmt(_0, _derive_more_display_formatter)
                }
                Error::BadMessage => _derive_more_display_formatter.write_str("BadMessage"),
                Error::BadType => _derive_more_display_formatter.write_str("BadType"),
                _ => Ok(()),
            }
        }
    }
    impl From<alloc::string::FromUtf8Error> for Error {
        fn from(err: alloc::string::FromUtf8Error) -> Self {
            Self::Std(StdError::InvalidUtf8 {
                msg: err.to_string(),
            })
        }
    }
    impl From<cf_guest::DecodeError> for Error {
        fn from(err: cf_guest::DecodeError) -> Self {
            match err {
                cf_guest::DecodeError::BadMessage => Self::BadMessage,
                cf_guest::DecodeError::BadType => Self::BadType,
                cf_guest::DecodeError::BadProto(err) => err.into(),
            }
        }
    }
    impl From<Error> for StdError {
        fn from(err: Error) -> Self {
            match err {
                Error::Std(err) => err,
                _ => StdError::GenericErr {
                    msg: err.to_string(),
                },
            }
        }
    }
}
mod ibc {
    //! A helper module which collects IBC types we’re using in a flatter namespace.
    pub use ibc::{
        core::{
            ics02_client::error::Error as ClientError,
            ics23_commitment::commitment::{CommitmentPrefix, CommitmentProofBytes},
            ics24_host::{identifier::ClientId, path},
        },
        Height,
        timestamp::Timestamp,
    };
    pub use ibc_proto as proto;
}
pub mod msg {
    use cosmwasm_schema::{cw_serde, QueryResponses};
    use cosmwasm_std::Uint64;
    use crate::{
        ibc,
        serialisation::{AsStr, Base64, MaybeBase64},
        state,
    };
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[serde(deny_unknown_fields, crate = "::cosmwasm_schema::serde")]
    #[schemars(crate = "::cosmwasm_schema::schemars")]
    pub struct InstantiateMsg {
        #[serde(with = "Base64")]
        #[schemars(with = "String")]
        pub client_state: state::ClientState,
        #[serde(with = "Base64")]
        #[schemars(with = "String")]
        pub consensus_state: state::ConsensusState,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl ::cosmwasm_schema::serde::Serialize for InstantiateMsg {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> ::cosmwasm_schema::serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: ::cosmwasm_schema::serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "InstantiateMsg",
                    false as usize + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "client_state",
                    {
                        #[doc(hidden)]
                        struct __SerializeWith<'__a> {
                            values: (&'__a state::ClientState,),
                            phantom: _serde::__private::PhantomData<InstantiateMsg>,
                        }
                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                            fn serialize<__S>(
                                &self,
                                __s: __S,
                            ) -> _serde::__private::Result<__S::Ok, __S::Error>
                            where
                                __S: _serde::Serializer,
                            {
                                Base64::serialize(self.values.0, __s)
                            }
                        }
                        &__SerializeWith {
                            values: (&self.client_state,),
                            phantom: _serde::__private::PhantomData::<InstantiateMsg>,
                        }
                    },
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "consensus_state",
                    {
                        #[doc(hidden)]
                        struct __SerializeWith<'__a> {
                            values: (&'__a state::ConsensusState,),
                            phantom: _serde::__private::PhantomData<InstantiateMsg>,
                        }
                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                            fn serialize<__S>(
                                &self,
                                __s: __S,
                            ) -> _serde::__private::Result<__S::Ok, __S::Error>
                            where
                                __S: _serde::Serializer,
                            {
                                Base64::serialize(self.values.0, __s)
                            }
                        }
                        &__SerializeWith {
                            values: (&self.consensus_state,),
                            phantom: _serde::__private::PhantomData::<InstantiateMsg>,
                        }
                    },
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl<'de> ::cosmwasm_schema::serde::Deserialize<'de> for InstantiateMsg {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> ::cosmwasm_schema::serde::__private::Result<Self, __D::Error>
            where
                __D: ::cosmwasm_schema::serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 2",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "client_state" => _serde::__private::Ok(__Field::__field0),
                            "consensus_state" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Err(_serde::de::Error::unknown_field(
                                __value, FIELDS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"client_state" => _serde::__private::Ok(__Field::__field0),
                            b"consensus_state" => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(_serde::de::Error::unknown_field(
                                    __value, FIELDS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<InstantiateMsg>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = InstantiateMsg;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct InstantiateMsg",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match {
                            #[doc(hidden)]
                            struct __DeserializeWith<'de> {
                                value: state::ClientState,
                                phantom: _serde::__private::PhantomData<InstantiateMsg>,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::__private::Ok(__DeserializeWith {
                                        value: Base64::deserialize(__deserializer)?,
                                        phantom: _serde::__private::PhantomData,
                                        lifetime: _serde::__private::PhantomData,
                                    })
                                }
                            }
                            _serde::__private::Option::map(
                                _serde::de::SeqAccess::next_element::<__DeserializeWith<'de>>(
                                    &mut __seq,
                                )?,
                                |__wrap| __wrap.value,
                            )
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct InstantiateMsg with 2 elements",
                                ))
                            }
                        };
                        let __field1 = match {
                            #[doc(hidden)]
                            struct __DeserializeWith<'de> {
                                value: state::ConsensusState,
                                phantom: _serde::__private::PhantomData<InstantiateMsg>,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::__private::Ok(__DeserializeWith {
                                        value: Base64::deserialize(__deserializer)?,
                                        phantom: _serde::__private::PhantomData,
                                        lifetime: _serde::__private::PhantomData,
                                    })
                                }
                            }
                            _serde::__private::Option::map(
                                _serde::de::SeqAccess::next_element::<__DeserializeWith<'de>>(
                                    &mut __seq,
                                )?,
                                |__wrap| __wrap.value,
                            )
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct InstantiateMsg with 2 elements",
                                ))
                            }
                        };
                        _serde::__private::Ok(InstantiateMsg {
                            client_state: __field0,
                            consensus_state: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<state::ClientState> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<state::ConsensusState> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "client_state",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some({
                                        #[doc(hidden)]
                                        struct __DeserializeWith<'de> {
                                            value: state::ClientState,
                                            phantom: _serde::__private::PhantomData<InstantiateMsg>,
                                            lifetime: _serde::__private::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::__private::Ok(__DeserializeWith {
                                                    value: Base64::deserialize(__deserializer)?,
                                                    phantom: _serde::__private::PhantomData,
                                                    lifetime: _serde::__private::PhantomData,
                                                })
                                            }
                                        }
                                        match _serde::de::MapAccess::next_value::<
                                            __DeserializeWith<'de>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__wrapper) => __wrapper.value,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    });
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "consensus_state",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some({
                                        #[doc(hidden)]
                                        struct __DeserializeWith<'de> {
                                            value: state::ConsensusState,
                                            phantom: _serde::__private::PhantomData<InstantiateMsg>,
                                            lifetime: _serde::__private::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::__private::Ok(__DeserializeWith {
                                                    value: Base64::deserialize(__deserializer)?,
                                                    phantom: _serde::__private::PhantomData,
                                                    lifetime: _serde::__private::PhantomData,
                                                })
                                            }
                                        }
                                        match _serde::de::MapAccess::next_value::<
                                            __DeserializeWith<'de>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__wrapper) => __wrapper.value,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    });
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    <__A::Error as _serde::de::Error>::missing_field(
                                        "client_state",
                                    ),
                                )
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    <__A::Error as _serde::de::Error>::missing_field(
                                        "consensus_state",
                                    ),
                                )
                            }
                        };
                        _serde::__private::Ok(InstantiateMsg {
                            client_state: __field0,
                            consensus_state: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["client_state", "consensus_state"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "InstantiateMsg",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<InstantiateMsg>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for InstantiateMsg {
        #[inline]
        fn clone(&self) -> InstantiateMsg {
            InstantiateMsg {
                client_state: ::core::clone::Clone::clone(&self.client_state),
                consensus_state: ::core::clone::Clone::clone(&self.consensus_state),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::fmt::Debug for InstantiateMsg {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "InstantiateMsg",
                "client_state",
                &self.client_state,
                "consensus_state",
                &&self.consensus_state,
            )
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for InstantiateMsg {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for InstantiateMsg {
        #[inline]
        fn eq(&self, other: &InstantiateMsg) -> bool {
            self.client_state == other.client_state && self.consensus_state == other.consensus_state
        }
    }
    const _: () = {
        use ::cosmwasm_schema::schemars;
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for InstantiateMsg {
            fn schema_name() -> std::string::String {
                "InstantiateMsg".to_owned()
            }
            fn schema_id() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed("cf_guest_cw::msg::InstantiateMsg")
            }
            fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                {
                    let mut schema_object = schemars::schema::SchemaObject {
                        instance_type: Some(schemars::schema::InstanceType::Object.into()),
                        ..Default::default()
                    };
                    let object_validation = schema_object.object();
                    object_validation.additional_properties = Some(Box::new(false.into()));
                    {
                        object_validation
                            .properties
                            .insert("client_state".to_owned(), gen.subschema_for::<String>());
                        if !<String as schemars::JsonSchema>::_schemars_private_is_option() {
                            object_validation.required.insert("client_state".to_owned());
                        }
                    }
                    {
                        object_validation
                            .properties
                            .insert("consensus_state".to_owned(), gen.subschema_for::<String>());
                        if !<String as schemars::JsonSchema>::_schemars_private_is_option() {
                            object_validation
                                .required
                                .insert("consensus_state".to_owned());
                        }
                    }
                    schemars::schema::Schema::Object(schema_object)
                }
            }
        };
    };
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[serde(
        deny_unknown_fields,
        rename_all = "snake_case",
        crate = "::cosmwasm_schema::serde"
    )]
    #[schemars(crate = "::cosmwasm_schema::schemars")]
    pub enum SudoMsg {
        UpdateState(UpdateStateMsg),
        UpdateStateOnMisbehaviour(UpdateStateOnMisbehaviourMsg),
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl ::cosmwasm_schema::serde::Serialize for SudoMsg {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> ::cosmwasm_schema::serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: ::cosmwasm_schema::serde::Serializer,
            {
                match *self {
                    SudoMsg::UpdateState(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "SudoMsg",
                            0u32,
                            "update_state",
                            __field0,
                        )
                    }
                    SudoMsg::UpdateStateOnMisbehaviour(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "SudoMsg",
                            1u32,
                            "update_state_on_misbehaviour",
                            __field0,
                        )
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl<'de> ::cosmwasm_schema::serde::Deserialize<'de> for SudoMsg {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> ::cosmwasm_schema::serde::__private::Result<Self, __D::Error>
            where
                __D: ::cosmwasm_schema::serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "variant identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"variant index 0 <= i < 2",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "update_state" => _serde::__private::Ok(__Field::__field0),
                            "update_state_on_misbehaviour" => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            _ => _serde::__private::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"update_state" => _serde::__private::Ok(__Field::__field0),
                            b"update_state_on_misbehaviour" => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<SudoMsg>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = SudoMsg;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "enum SudoMsg")
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match _serde::de::EnumAccess::variant(__data)? {
                            (__Field::__field0, __variant) => _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<UpdateStateMsg>(
                                    __variant,
                                ),
                                SudoMsg::UpdateState,
                            ),
                            (__Field::__field1, __variant) => _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<
                                    UpdateStateOnMisbehaviourMsg,
                                >(__variant),
                                SudoMsg::UpdateStateOnMisbehaviour,
                            ),
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] =
                    &["update_state", "update_state_on_misbehaviour"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "SudoMsg",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<SudoMsg>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for SudoMsg {
        #[inline]
        fn clone(&self) -> SudoMsg {
            match self {
                SudoMsg::UpdateState(__self_0) => {
                    SudoMsg::UpdateState(::core::clone::Clone::clone(__self_0))
                }
                SudoMsg::UpdateStateOnMisbehaviour(__self_0) => {
                    SudoMsg::UpdateStateOnMisbehaviour(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::fmt::Debug for SudoMsg {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                SudoMsg::UpdateState(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "UpdateState", &__self_0)
                }
                SudoMsg::UpdateStateOnMisbehaviour(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "UpdateStateOnMisbehaviour",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for SudoMsg {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for SudoMsg {
        #[inline]
        fn eq(&self, other: &SudoMsg) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (SudoMsg::UpdateState(__self_0), SudoMsg::UpdateState(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (
                        SudoMsg::UpdateStateOnMisbehaviour(__self_0),
                        SudoMsg::UpdateStateOnMisbehaviour(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
        }
    }
    const _: () = {
        use ::cosmwasm_schema::schemars;
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for SudoMsg {
            fn schema_name() -> std::string::String {
                "SudoMsg".to_owned()
            }
            fn schema_id() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed("cf_guest_cw::msg::SudoMsg")
            }
            fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                    subschemas: Some(Box::new(schemars::schema::SubschemaValidation {
                        one_of: Some(<[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                                    instance_type: Some(
                                        schemars::schema::InstanceType::Object.into(),
                                    ),
                                    object: Some(Box::new(schemars::schema::ObjectValidation {
                                        properties: {
                                            let mut props = schemars::Map::new();
                                            props.insert(
                                                "update_state".to_owned(),
                                                gen.subschema_for::<UpdateStateMsg>(),
                                            );
                                            props
                                        },
                                        required: {
                                            let mut required = schemars::Set::new();
                                            required.insert("update_state".to_owned());
                                            required
                                        },
                                        additional_properties: Some(Box::new(false.into())),
                                        ..Default::default()
                                    })),
                                    ..Default::default()
                                }),
                                schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                                    instance_type: Some(
                                        schemars::schema::InstanceType::Object.into(),
                                    ),
                                    object: Some(Box::new(schemars::schema::ObjectValidation {
                                        properties: {
                                            let mut props = schemars::Map::new();
                                            props.insert(
                                                "update_state_on_misbehaviour".to_owned(),
                                                gen.subschema_for::<UpdateStateOnMisbehaviourMsg>(),
                                            );
                                            props
                                        },
                                        required: {
                                            let mut required = schemars::Set::new();
                                            required
                                                .insert("update_state_on_misbehaviour".to_owned());
                                            required
                                        },
                                        additional_properties: Some(Box::new(false.into())),
                                        ..Default::default()
                                    })),
                                    ..Default::default()
                                }),
                            ]),
                        )),
                        ..Default::default()
                    })),
                    ..Default::default()
                })
            }
        };
    };
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[serde(deny_unknown_fields, crate = "::cosmwasm_schema::serde")]
    #[schemars(crate = "::cosmwasm_schema::schemars")]
    pub struct UpdateStateMsg {
        #[serde(with = "Base64")]
        #[schemars(with = "String")]
        pub header: state::Header,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl ::cosmwasm_schema::serde::Serialize for UpdateStateMsg {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> ::cosmwasm_schema::serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: ::cosmwasm_schema::serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "UpdateStateMsg",
                    false as usize + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "header", {
                    #[doc(hidden)]
                    struct __SerializeWith<'__a> {
                        values: (&'__a state::Header,),
                        phantom: _serde::__private::PhantomData<UpdateStateMsg>,
                    }
                    impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                        fn serialize<__S>(
                            &self,
                            __s: __S,
                        ) -> _serde::__private::Result<__S::Ok, __S::Error>
                        where
                            __S: _serde::Serializer,
                        {
                            Base64::serialize(self.values.0, __s)
                        }
                    }
                    &__SerializeWith {
                        values: (&self.header,),
                        phantom: _serde::__private::PhantomData::<UpdateStateMsg>,
                    }
                })?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl<'de> ::cosmwasm_schema::serde::Deserialize<'de> for UpdateStateMsg {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> ::cosmwasm_schema::serde::__private::Result<Self, __D::Error>
            where
                __D: ::cosmwasm_schema::serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 1",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "header" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Err(_serde::de::Error::unknown_field(
                                __value, FIELDS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"header" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(_serde::de::Error::unknown_field(
                                    __value, FIELDS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<UpdateStateMsg>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = UpdateStateMsg;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct UpdateStateMsg",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match {
                            #[doc(hidden)]
                            struct __DeserializeWith<'de> {
                                value: state::Header,
                                phantom: _serde::__private::PhantomData<UpdateStateMsg>,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::__private::Ok(__DeserializeWith {
                                        value: Base64::deserialize(__deserializer)?,
                                        phantom: _serde::__private::PhantomData,
                                        lifetime: _serde::__private::PhantomData,
                                    })
                                }
                            }
                            _serde::__private::Option::map(
                                _serde::de::SeqAccess::next_element::<__DeserializeWith<'de>>(
                                    &mut __seq,
                                )?,
                                |__wrap| __wrap.value,
                            )
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct UpdateStateMsg with 1 element",
                                ))
                            }
                        };
                        _serde::__private::Ok(UpdateStateMsg { header: __field0 })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<state::Header> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "header",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some({
                                        #[doc(hidden)]
                                        struct __DeserializeWith<'de> {
                                            value: state::Header,
                                            phantom: _serde::__private::PhantomData<UpdateStateMsg>,
                                            lifetime: _serde::__private::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::__private::Ok(__DeserializeWith {
                                                    value: Base64::deserialize(__deserializer)?,
                                                    phantom: _serde::__private::PhantomData,
                                                    lifetime: _serde::__private::PhantomData,
                                                })
                                            }
                                        }
                                        match _serde::de::MapAccess::next_value::<
                                            __DeserializeWith<'de>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__wrapper) => __wrapper.value,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    });
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    <__A::Error as _serde::de::Error>::missing_field("header"),
                                )
                            }
                        };
                        _serde::__private::Ok(UpdateStateMsg { header: __field0 })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["header"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "UpdateStateMsg",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<UpdateStateMsg>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for UpdateStateMsg {
        #[inline]
        fn clone(&self) -> UpdateStateMsg {
            UpdateStateMsg {
                header: ::core::clone::Clone::clone(&self.header),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::fmt::Debug for UpdateStateMsg {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "UpdateStateMsg",
                "header",
                &&self.header,
            )
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for UpdateStateMsg {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for UpdateStateMsg {
        #[inline]
        fn eq(&self, other: &UpdateStateMsg) -> bool {
            self.header == other.header
        }
    }
    const _: () = {
        use ::cosmwasm_schema::schemars;
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for UpdateStateMsg {
            fn schema_name() -> std::string::String {
                "UpdateStateMsg".to_owned()
            }
            fn schema_id() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed("cf_guest_cw::msg::UpdateStateMsg")
            }
            fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                {
                    let mut schema_object = schemars::schema::SchemaObject {
                        instance_type: Some(schemars::schema::InstanceType::Object.into()),
                        ..Default::default()
                    };
                    let object_validation = schema_object.object();
                    object_validation.additional_properties = Some(Box::new(false.into()));
                    {
                        object_validation
                            .properties
                            .insert("header".to_owned(), gen.subschema_for::<String>());
                        if !<String as schemars::JsonSchema>::_schemars_private_is_option() {
                            object_validation.required.insert("header".to_owned());
                        }
                    }
                    schemars::schema::Schema::Object(schema_object)
                }
            }
        };
    };
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[serde(deny_unknown_fields, crate = "::cosmwasm_schema::serde")]
    #[schemars(crate = "::cosmwasm_schema::schemars")]
    pub struct UpdateStateOnMisbehaviourMsg {
        #[serde(with = "Base64")]
        #[schemars(with = "String")]
        pub misbehaviour_message: state::Misbehaviour,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl ::cosmwasm_schema::serde::Serialize for UpdateStateOnMisbehaviourMsg {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> ::cosmwasm_schema::serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: ::cosmwasm_schema::serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "UpdateStateOnMisbehaviourMsg",
                    false as usize + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "misbehaviour_message",
                    {
                        #[doc(hidden)]
                        struct __SerializeWith<'__a> {
                            values: (&'__a state::Misbehaviour,),
                            phantom: _serde::__private::PhantomData<UpdateStateOnMisbehaviourMsg>,
                        }
                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                            fn serialize<__S>(
                                &self,
                                __s: __S,
                            ) -> _serde::__private::Result<__S::Ok, __S::Error>
                            where
                                __S: _serde::Serializer,
                            {
                                Base64::serialize(self.values.0, __s)
                            }
                        }
                        &__SerializeWith {
                            values: (&self.misbehaviour_message,),
                            phantom: _serde::__private::PhantomData::<UpdateStateOnMisbehaviourMsg>,
                        }
                    },
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl<'de> ::cosmwasm_schema::serde::Deserialize<'de> for UpdateStateOnMisbehaviourMsg {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> ::cosmwasm_schema::serde::__private::Result<Self, __D::Error>
            where
                __D: ::cosmwasm_schema::serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 1",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "misbehaviour_message" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Err(_serde::de::Error::unknown_field(
                                __value, FIELDS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"misbehaviour_message" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(_serde::de::Error::unknown_field(
                                    __value, FIELDS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<UpdateStateOnMisbehaviourMsg>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = UpdateStateOnMisbehaviourMsg;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct UpdateStateOnMisbehaviourMsg",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match {
                            #[doc(hidden)]
                            struct __DeserializeWith<'de> {
                                value: state::Misbehaviour,
                                phantom:
                                    _serde::__private::PhantomData<UpdateStateOnMisbehaviourMsg>,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::__private::Ok(__DeserializeWith {
                                        value: Base64::deserialize(__deserializer)?,
                                        phantom: _serde::__private::PhantomData,
                                        lifetime: _serde::__private::PhantomData,
                                    })
                                }
                            }
                            _serde::__private::Option::map(
                                _serde::de::SeqAccess::next_element::<__DeserializeWith<'de>>(
                                    &mut __seq,
                                )?,
                                |__wrap| __wrap.value,
                            )
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct UpdateStateOnMisbehaviourMsg with 1 element",
                                ))
                            }
                        };
                        _serde::__private::Ok(UpdateStateOnMisbehaviourMsg {
                            misbehaviour_message: __field0,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<state::Misbehaviour> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "misbehaviour_message",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some({
                                        #[doc(hidden)]
                                        struct __DeserializeWith<'de> {
                                            value: state::Misbehaviour,
                                            phantom: _serde::__private::PhantomData<
                                                UpdateStateOnMisbehaviourMsg,
                                            >,
                                            lifetime: _serde::__private::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::__private::Ok(__DeserializeWith {
                                                    value: Base64::deserialize(__deserializer)?,
                                                    phantom: _serde::__private::PhantomData,
                                                    lifetime: _serde::__private::PhantomData,
                                                })
                                            }
                                        }
                                        match _serde::de::MapAccess::next_value::<
                                            __DeserializeWith<'de>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__wrapper) => __wrapper.value,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    });
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    <__A::Error as _serde::de::Error>::missing_field(
                                        "misbehaviour_message",
                                    ),
                                )
                            }
                        };
                        _serde::__private::Ok(UpdateStateOnMisbehaviourMsg {
                            misbehaviour_message: __field0,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["misbehaviour_message"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "UpdateStateOnMisbehaviourMsg",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<UpdateStateOnMisbehaviourMsg>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for UpdateStateOnMisbehaviourMsg {
        #[inline]
        fn clone(&self) -> UpdateStateOnMisbehaviourMsg {
            UpdateStateOnMisbehaviourMsg {
                misbehaviour_message: ::core::clone::Clone::clone(&self.misbehaviour_message),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::fmt::Debug for UpdateStateOnMisbehaviourMsg {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "UpdateStateOnMisbehaviourMsg",
                "misbehaviour_message",
                &&self.misbehaviour_message,
            )
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for UpdateStateOnMisbehaviourMsg {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for UpdateStateOnMisbehaviourMsg {
        #[inline]
        fn eq(&self, other: &UpdateStateOnMisbehaviourMsg) -> bool {
            self.misbehaviour_message == other.misbehaviour_message
        }
    }
    const _: () = {
        use ::cosmwasm_schema::schemars;
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for UpdateStateOnMisbehaviourMsg {
            fn schema_name() -> std::string::String {
                "UpdateStateOnMisbehaviourMsg".to_owned()
            }
            fn schema_id() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed("cf_guest_cw::msg::UpdateStateOnMisbehaviourMsg")
            }
            fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                {
                    let mut schema_object = schemars::schema::SchemaObject {
                        instance_type: Some(schemars::schema::InstanceType::Object.into()),
                        ..Default::default()
                    };
                    let object_validation = schema_object.object();
                    object_validation.additional_properties = Some(Box::new(false.into()));
                    {
                        object_validation.properties.insert(
                            "misbehaviour_message".to_owned(),
                            gen.subschema_for::<String>(),
                        );
                        if !<String as schemars::JsonSchema>::_schemars_private_is_option() {
                            object_validation
                                .required
                                .insert("misbehaviour_message".to_owned());
                        }
                    }
                    schemars::schema::Schema::Object(schema_object)
                }
            }
        };
    };
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[serde(
        deny_unknown_fields,
        rename_all = "snake_case",
        crate = "::cosmwasm_schema::serde"
    )]
    #[schemars(crate = "::cosmwasm_schema::schemars")]
    pub enum QueryMsg {
        /// Verifies client message.
        # [returns (())]
        VerifyClientMessage(VerifyClientMessageMsg),
        /// Checks client message for misbehaviour.
        #[returns(bool)]
        CheckForMisbehaviour(CheckForMisbehaviourMsg),
        /// Checks whether provided membership or non-membership proof is valid.
        ///
        /// The proof is a membership proof is `self.0.value` field is `Some`.
        /// Otherwise, if `self.0.value` is `None`, the proof is non-membership
        /// proof.
        # [returns (())]
        VerifyStateProof(VerifyStateProofMsg),
        /// Checks status of the client.
        #[returns(StatusResponse)]
        Status(StatusMsg),
        /// Returns timestamp for consensus at given height.
        ///
        /// The timestamp is represented as nanoseconds since Unix epoch.
        #[returns(Uint64)]
        TimestampAtHeight(TimestampAtHeightMsg),
        /// Gets metadata of all consensus states.
        # [returns (Vec < ConsensusStateMetadata >)]
        ExportMetadata(ExportMetadataMsg),
    }
    #[automatically_derived]
    #[cfg(not(target_arch = "wasm32"))]
    impl ::cosmwasm_schema::QueryResponses for QueryMsg {
        fn response_schemas_impl(
        ) -> ::std::collections::BTreeMap<String, ::cosmwasm_schema::schemars::schema::RootSchema>
        {
            ::std::collections::BTreeMap::from([
                (
                    "verify_client_message".to_string(),
                    ::cosmwasm_schema::schemars::gen::SchemaGenerator::new(
                        ::cosmwasm_schema::schemars::gen::SchemaSettings::draft07(),
                    )
                    .into_root_schema_for::<()>(),
                ),
                (
                    "check_for_misbehaviour".to_string(),
                    ::cosmwasm_schema::schemars::gen::SchemaGenerator::new(
                        ::cosmwasm_schema::schemars::gen::SchemaSettings::draft07(),
                    )
                    .into_root_schema_for::<bool>(),
                ),
                (
                    "verify_state_proof".to_string(),
                    ::cosmwasm_schema::schemars::gen::SchemaGenerator::new(
                        ::cosmwasm_schema::schemars::gen::SchemaSettings::draft07(),
                    )
                    .into_root_schema_for::<()>(),
                ),
                (
                    "status".to_string(),
                    ::cosmwasm_schema::schemars::gen::SchemaGenerator::new(
                        ::cosmwasm_schema::schemars::gen::SchemaSettings::draft07(),
                    )
                    .into_root_schema_for::<StatusResponse>(),
                ),
                (
                    "timestamp_at_height".to_string(),
                    ::cosmwasm_schema::schemars::gen::SchemaGenerator::new(
                        ::cosmwasm_schema::schemars::gen::SchemaSettings::draft07(),
                    )
                    .into_root_schema_for::<Uint64>(),
                ),
                (
                    "export_metadata".to_string(),
                    ::cosmwasm_schema::schemars::gen::SchemaGenerator::new(
                        ::cosmwasm_schema::schemars::gen::SchemaSettings::draft07(),
                    )
                    .into_root_schema_for::<Vec<ConsensusStateMetadata>>(),
                ),
            ])
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl ::cosmwasm_schema::serde::Serialize for QueryMsg {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> ::cosmwasm_schema::serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: ::cosmwasm_schema::serde::Serializer,
            {
                match *self {
                    QueryMsg::VerifyClientMessage(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "QueryMsg",
                            0u32,
                            "verify_client_message",
                            __field0,
                        )
                    }
                    QueryMsg::CheckForMisbehaviour(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "QueryMsg",
                            1u32,
                            "check_for_misbehaviour",
                            __field0,
                        )
                    }
                    QueryMsg::VerifyStateProof(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "QueryMsg",
                            2u32,
                            "verify_state_proof",
                            __field0,
                        )
                    }
                    QueryMsg::Status(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "QueryMsg",
                            3u32,
                            "status",
                            __field0,
                        )
                    }
                    QueryMsg::TimestampAtHeight(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "QueryMsg",
                            4u32,
                            "timestamp_at_height",
                            __field0,
                        )
                    }
                    QueryMsg::ExportMetadata(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "QueryMsg",
                            5u32,
                            "export_metadata",
                            __field0,
                        )
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl<'de> ::cosmwasm_schema::serde::Deserialize<'de> for QueryMsg {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> ::cosmwasm_schema::serde::__private::Result<Self, __D::Error>
            where
                __D: ::cosmwasm_schema::serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "variant identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"variant index 0 <= i < 6",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "verify_client_message" => _serde::__private::Ok(__Field::__field0),
                            "check_for_misbehaviour" => _serde::__private::Ok(__Field::__field1),
                            "verify_state_proof" => _serde::__private::Ok(__Field::__field2),
                            "status" => _serde::__private::Ok(__Field::__field3),
                            "timestamp_at_height" => _serde::__private::Ok(__Field::__field4),
                            "export_metadata" => _serde::__private::Ok(__Field::__field5),
                            _ => _serde::__private::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"verify_client_message" => _serde::__private::Ok(__Field::__field0),
                            b"check_for_misbehaviour" => _serde::__private::Ok(__Field::__field1),
                            b"verify_state_proof" => _serde::__private::Ok(__Field::__field2),
                            b"status" => _serde::__private::Ok(__Field::__field3),
                            b"timestamp_at_height" => _serde::__private::Ok(__Field::__field4),
                            b"export_metadata" => _serde::__private::Ok(__Field::__field5),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<QueryMsg>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = QueryMsg;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "enum QueryMsg")
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match _serde::de::EnumAccess::variant(__data)? {
                            (__Field::__field0, __variant) => _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<VerifyClientMessageMsg>(
                                    __variant,
                                ),
                                QueryMsg::VerifyClientMessage,
                            ),
                            (__Field::__field1, __variant) => _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<CheckForMisbehaviourMsg>(
                                    __variant,
                                ),
                                QueryMsg::CheckForMisbehaviour,
                            ),
                            (__Field::__field2, __variant) => _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<VerifyStateProofMsg>(
                                    __variant,
                                ),
                                QueryMsg::VerifyStateProof,
                            ),
                            (__Field::__field3, __variant) => _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<StatusMsg>(__variant),
                                QueryMsg::Status,
                            ),
                            (__Field::__field4, __variant) => _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<TimestampAtHeightMsg>(
                                    __variant,
                                ),
                                QueryMsg::TimestampAtHeight,
                            ),
                            (__Field::__field5, __variant) => _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<ExportMetadataMsg>(
                                    __variant,
                                ),
                                QueryMsg::ExportMetadata,
                            ),
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &[
                    "verify_client_message",
                    "check_for_misbehaviour",
                    "verify_state_proof",
                    "status",
                    "timestamp_at_height",
                    "export_metadata",
                ];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "QueryMsg",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<QueryMsg>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for QueryMsg {
        #[inline]
        fn clone(&self) -> QueryMsg {
            match self {
                QueryMsg::VerifyClientMessage(__self_0) => {
                    QueryMsg::VerifyClientMessage(::core::clone::Clone::clone(__self_0))
                }
                QueryMsg::CheckForMisbehaviour(__self_0) => {
                    QueryMsg::CheckForMisbehaviour(::core::clone::Clone::clone(__self_0))
                }
                QueryMsg::VerifyStateProof(__self_0) => {
                    QueryMsg::VerifyStateProof(::core::clone::Clone::clone(__self_0))
                }
                QueryMsg::Status(__self_0) => {
                    QueryMsg::Status(::core::clone::Clone::clone(__self_0))
                }
                QueryMsg::TimestampAtHeight(__self_0) => {
                    QueryMsg::TimestampAtHeight(::core::clone::Clone::clone(__self_0))
                }
                QueryMsg::ExportMetadata(__self_0) => {
                    QueryMsg::ExportMetadata(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::fmt::Debug for QueryMsg {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                QueryMsg::VerifyClientMessage(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "VerifyClientMessage",
                        &__self_0,
                    )
                }
                QueryMsg::CheckForMisbehaviour(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "CheckForMisbehaviour",
                        &__self_0,
                    )
                }
                QueryMsg::VerifyStateProof(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "VerifyStateProof",
                        &__self_0,
                    )
                }
                QueryMsg::Status(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Status", &__self_0)
                }
                QueryMsg::TimestampAtHeight(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "TimestampAtHeight",
                        &__self_0,
                    )
                }
                QueryMsg::ExportMetadata(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ExportMetadata",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for QueryMsg {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for QueryMsg {
        #[inline]
        fn eq(&self, other: &QueryMsg) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (
                        QueryMsg::VerifyClientMessage(__self_0),
                        QueryMsg::VerifyClientMessage(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (
                        QueryMsg::CheckForMisbehaviour(__self_0),
                        QueryMsg::CheckForMisbehaviour(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (
                        QueryMsg::VerifyStateProof(__self_0),
                        QueryMsg::VerifyStateProof(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (QueryMsg::Status(__self_0), QueryMsg::Status(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (
                        QueryMsg::TimestampAtHeight(__self_0),
                        QueryMsg::TimestampAtHeight(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (QueryMsg::ExportMetadata(__self_0), QueryMsg::ExportMetadata(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
        }
    }
    const _: () = {
        use ::cosmwasm_schema::schemars;
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for QueryMsg {
            fn schema_name() -> std::string::String {
                "QueryMsg".to_owned()
            }
            fn schema_id() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed("cf_guest_cw::msg::QueryMsg")
            }
            fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                    subschemas: Some(Box::new(schemars::schema::SubschemaValidation {
                        one_of: Some(<[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                {
                                    let schema = schemars::schema::Schema::Object(
                                        schemars::schema::SchemaObject {
                                            instance_type: Some(
                                                schemars::schema::InstanceType::Object.into(),
                                            ),
                                            object: Some(Box::new(
                                                schemars::schema::ObjectValidation {
                                                    properties: {
                                                        let mut props = schemars::Map::new();
                                                        props . insert ("verify_client_message" . to_owned () , gen . subschema_for :: < VerifyClientMessageMsg > ()) ;
                                                        props
                                                    },
                                                    required: {
                                                        let mut required = schemars::Set::new();
                                                        required.insert(
                                                            "verify_client_message".to_owned(),
                                                        );
                                                        required
                                                    },
                                                    additional_properties: Some(Box::new(
                                                        false.into(),
                                                    )),
                                                    ..Default::default()
                                                },
                                            )),
                                            ..Default::default()
                                        },
                                    );
                                    schemars::_private::apply_metadata(
                                        schema,
                                        schemars::schema::Metadata {
                                            description: Some(
                                                "Verifies client message.".to_owned(),
                                            ),
                                            ..Default::default()
                                        },
                                    )
                                },
                                {
                                    let schema = schemars::schema::Schema::Object(
                                        schemars::schema::SchemaObject {
                                            instance_type: Some(
                                                schemars::schema::InstanceType::Object.into(),
                                            ),
                                            object: Some(Box::new(
                                                schemars::schema::ObjectValidation {
                                                    properties: {
                                                        let mut props = schemars::Map::new();
                                                        props . insert ("check_for_misbehaviour" . to_owned () , gen . subschema_for :: < CheckForMisbehaviourMsg > ()) ;
                                                        props
                                                    },
                                                    required: {
                                                        let mut required = schemars::Set::new();
                                                        required.insert(
                                                            "check_for_misbehaviour".to_owned(),
                                                        );
                                                        required
                                                    },
                                                    additional_properties: Some(Box::new(
                                                        false.into(),
                                                    )),
                                                    ..Default::default()
                                                },
                                            )),
                                            ..Default::default()
                                        },
                                    );
                                    schemars::_private::apply_metadata(
                                        schema,
                                        schemars::schema::Metadata {
                                            description: Some(
                                                "Checks client message for misbehaviour."
                                                    .to_owned(),
                                            ),
                                            ..Default::default()
                                        },
                                    )
                                },
                                {
                                    let schema = schemars::schema::Schema::Object(
                                        schemars::schema::SchemaObject {
                                            instance_type: Some(
                                                schemars::schema::InstanceType::Object.into(),
                                            ),
                                            object: Some(Box::new(
                                                schemars::schema::ObjectValidation {
                                                    properties: {
                                                        let mut props = schemars::Map::new();
                                                        props . insert ("verify_state_proof" . to_owned () , gen . subschema_for :: < VerifyStateProofMsg > ()) ;
                                                        props
                                                    },
                                                    required: {
                                                        let mut required = schemars::Set::new();
                                                        required.insert(
                                                            "verify_state_proof".to_owned(),
                                                        );
                                                        required
                                                    },
                                                    additional_properties: Some(Box::new(
                                                        false.into(),
                                                    )),
                                                    ..Default::default()
                                                },
                                            )),
                                            ..Default::default()
                                        },
                                    );
                                    schemars :: _private :: apply_metadata (schema , schemars :: schema :: Metadata { description : Some ("Checks whether provided membership or non-membership proof is valid.\n\nThe proof is a membership proof is `self.0.value` field is `Some`. Otherwise, if `self.0.value` is `None`, the proof is non-membership proof." . to_owned ()) , .. Default :: default () })
                                },
                                {
                                    let schema = schemars::schema::Schema::Object(
                                        schemars::schema::SchemaObject {
                                            instance_type: Some(
                                                schemars::schema::InstanceType::Object.into(),
                                            ),
                                            object: Some(Box::new(
                                                schemars::schema::ObjectValidation {
                                                    properties: {
                                                        let mut props = schemars::Map::new();
                                                        props.insert(
                                                            "status".to_owned(),
                                                            gen.subschema_for::<StatusMsg>(),
                                                        );
                                                        props
                                                    },
                                                    required: {
                                                        let mut required = schemars::Set::new();
                                                        required.insert("status".to_owned());
                                                        required
                                                    },
                                                    additional_properties: Some(Box::new(
                                                        false.into(),
                                                    )),
                                                    ..Default::default()
                                                },
                                            )),
                                            ..Default::default()
                                        },
                                    );
                                    schemars::_private::apply_metadata(
                                        schema,
                                        schemars::schema::Metadata {
                                            description: Some(
                                                "Checks status of the client.".to_owned(),
                                            ),
                                            ..Default::default()
                                        },
                                    )
                                },
                                {
                                    let schema = schemars::schema::Schema::Object(
                                        schemars::schema::SchemaObject {
                                            instance_type: Some(
                                                schemars::schema::InstanceType::Object.into(),
                                            ),
                                            object: Some(Box::new(
                                                schemars::schema::ObjectValidation {
                                                    properties: {
                                                        let mut props = schemars::Map::new();
                                                        props . insert ("timestamp_at_height" . to_owned () , gen . subschema_for :: < TimestampAtHeightMsg > ()) ;
                                                        props
                                                    },
                                                    required: {
                                                        let mut required = schemars::Set::new();
                                                        required.insert(
                                                            "timestamp_at_height".to_owned(),
                                                        );
                                                        required
                                                    },
                                                    additional_properties: Some(Box::new(
                                                        false.into(),
                                                    )),
                                                    ..Default::default()
                                                },
                                            )),
                                            ..Default::default()
                                        },
                                    );
                                    schemars :: _private :: apply_metadata (schema , schemars :: schema :: Metadata { description : Some ("Returns timestamp for consensus at given height.\n\nThe timestamp is represented as nanoseconds since Unix epoch." . to_owned ()) , .. Default :: default () })
                                },
                                {
                                    let schema = schemars::schema::Schema::Object(
                                        schemars::schema::SchemaObject {
                                            instance_type: Some(
                                                schemars::schema::InstanceType::Object.into(),
                                            ),
                                            object: Some(Box::new(
                                                schemars::schema::ObjectValidation {
                                                    properties: {
                                                        let mut props = schemars::Map::new();
                                                        props.insert(
                                                            "export_metadata".to_owned(),
                                                            gen.subschema_for::<ExportMetadataMsg>(
                                                            ),
                                                        );
                                                        props
                                                    },
                                                    required: {
                                                        let mut required = schemars::Set::new();
                                                        required
                                                            .insert("export_metadata".to_owned());
                                                        required
                                                    },
                                                    additional_properties: Some(Box::new(
                                                        false.into(),
                                                    )),
                                                    ..Default::default()
                                                },
                                            )),
                                            ..Default::default()
                                        },
                                    );
                                    schemars::_private::apply_metadata(
                                        schema,
                                        schemars::schema::Metadata {
                                            description: Some(
                                                "Gets metadata of all consensus states.".to_owned(),
                                            ),
                                            ..Default::default()
                                        },
                                    )
                                },
                            ]),
                        )),
                        ..Default::default()
                    })),
                    ..Default::default()
                })
            }
        };
    };
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[serde(deny_unknown_fields, crate = "::cosmwasm_schema::serde")]
    #[schemars(crate = "::cosmwasm_schema::schemars")]
    pub struct VerifyClientMessageMsg {
        #[serde(with = "Base64")]
        #[schemars(with = "String")]
        pub client_message: Vec<u8>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl ::cosmwasm_schema::serde::Serialize for VerifyClientMessageMsg {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> ::cosmwasm_schema::serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: ::cosmwasm_schema::serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "VerifyClientMessageMsg",
                    false as usize + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "client_message",
                    {
                        #[doc(hidden)]
                        struct __SerializeWith<'__a> {
                            values: (&'__a Vec<u8>,),
                            phantom: _serde::__private::PhantomData<VerifyClientMessageMsg>,
                        }
                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                            fn serialize<__S>(
                                &self,
                                __s: __S,
                            ) -> _serde::__private::Result<__S::Ok, __S::Error>
                            where
                                __S: _serde::Serializer,
                            {
                                Base64::serialize(self.values.0, __s)
                            }
                        }
                        &__SerializeWith {
                            values: (&self.client_message,),
                            phantom: _serde::__private::PhantomData::<VerifyClientMessageMsg>,
                        }
                    },
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl<'de> ::cosmwasm_schema::serde::Deserialize<'de> for VerifyClientMessageMsg {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> ::cosmwasm_schema::serde::__private::Result<Self, __D::Error>
            where
                __D: ::cosmwasm_schema::serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 1",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "client_message" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Err(_serde::de::Error::unknown_field(
                                __value, FIELDS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"client_message" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(_serde::de::Error::unknown_field(
                                    __value, FIELDS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<VerifyClientMessageMsg>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = VerifyClientMessageMsg;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct VerifyClientMessageMsg",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match {
                            #[doc(hidden)]
                            struct __DeserializeWith<'de> {
                                value: Vec<u8>,
                                phantom: _serde::__private::PhantomData<VerifyClientMessageMsg>,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::__private::Ok(__DeserializeWith {
                                        value: Base64::deserialize(__deserializer)?,
                                        phantom: _serde::__private::PhantomData,
                                        lifetime: _serde::__private::PhantomData,
                                    })
                                }
                            }
                            _serde::__private::Option::map(
                                _serde::de::SeqAccess::next_element::<__DeserializeWith<'de>>(
                                    &mut __seq,
                                )?,
                                |__wrap| __wrap.value,
                            )
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct VerifyClientMessageMsg with 1 element",
                                ))
                            }
                        };
                        _serde::__private::Ok(VerifyClientMessageMsg {
                            client_message: __field0,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Vec<u8>> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "client_message",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some({
                                        #[doc(hidden)]
                                        struct __DeserializeWith<'de> {
                                            value: Vec<u8>,
                                            phantom: _serde::__private::PhantomData<
                                                VerifyClientMessageMsg,
                                            >,
                                            lifetime: _serde::__private::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::__private::Ok(__DeserializeWith {
                                                    value: Base64::deserialize(__deserializer)?,
                                                    phantom: _serde::__private::PhantomData,
                                                    lifetime: _serde::__private::PhantomData,
                                                })
                                            }
                                        }
                                        match _serde::de::MapAccess::next_value::<
                                            __DeserializeWith<'de>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__wrapper) => __wrapper.value,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    });
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    <__A::Error as _serde::de::Error>::missing_field(
                                        "client_message",
                                    ),
                                )
                            }
                        };
                        _serde::__private::Ok(VerifyClientMessageMsg {
                            client_message: __field0,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["client_message"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "VerifyClientMessageMsg",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<VerifyClientMessageMsg>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for VerifyClientMessageMsg {
        #[inline]
        fn clone(&self) -> VerifyClientMessageMsg {
            VerifyClientMessageMsg {
                client_message: ::core::clone::Clone::clone(&self.client_message),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::fmt::Debug for VerifyClientMessageMsg {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "VerifyClientMessageMsg",
                "client_message",
                &&self.client_message,
            )
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for VerifyClientMessageMsg {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for VerifyClientMessageMsg {
        #[inline]
        fn eq(&self, other: &VerifyClientMessageMsg) -> bool {
            self.client_message == other.client_message
        }
    }
    const _: () = {
        use ::cosmwasm_schema::schemars;
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for VerifyClientMessageMsg {
            fn schema_name() -> std::string::String {
                "VerifyClientMessageMsg".to_owned()
            }
            fn schema_id() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed("cf_guest_cw::msg::VerifyClientMessageMsg")
            }
            fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                {
                    let mut schema_object = schemars::schema::SchemaObject {
                        instance_type: Some(schemars::schema::InstanceType::Object.into()),
                        ..Default::default()
                    };
                    let object_validation = schema_object.object();
                    object_validation.additional_properties = Some(Box::new(false.into()));
                    {
                        object_validation
                            .properties
                            .insert("client_message".to_owned(), gen.subschema_for::<String>());
                        if !<String as schemars::JsonSchema>::_schemars_private_is_option() {
                            object_validation
                                .required
                                .insert("client_message".to_owned());
                        }
                    }
                    schemars::schema::Schema::Object(schema_object)
                }
            }
        };
    };
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[serde(deny_unknown_fields, crate = "::cosmwasm_schema::serde")]
    #[schemars(crate = "::cosmwasm_schema::schemars")]
    pub struct CheckForMisbehaviourMsg {
        #[serde(with = "Base64")]
        #[schemars(with = "String")]
        pub client_message: Vec<u8>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl ::cosmwasm_schema::serde::Serialize for CheckForMisbehaviourMsg {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> ::cosmwasm_schema::serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: ::cosmwasm_schema::serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "CheckForMisbehaviourMsg",
                    false as usize + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "client_message",
                    {
                        #[doc(hidden)]
                        struct __SerializeWith<'__a> {
                            values: (&'__a Vec<u8>,),
                            phantom: _serde::__private::PhantomData<CheckForMisbehaviourMsg>,
                        }
                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                            fn serialize<__S>(
                                &self,
                                __s: __S,
                            ) -> _serde::__private::Result<__S::Ok, __S::Error>
                            where
                                __S: _serde::Serializer,
                            {
                                Base64::serialize(self.values.0, __s)
                            }
                        }
                        &__SerializeWith {
                            values: (&self.client_message,),
                            phantom: _serde::__private::PhantomData::<CheckForMisbehaviourMsg>,
                        }
                    },
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl<'de> ::cosmwasm_schema::serde::Deserialize<'de> for CheckForMisbehaviourMsg {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> ::cosmwasm_schema::serde::__private::Result<Self, __D::Error>
            where
                __D: ::cosmwasm_schema::serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 1",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "client_message" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Err(_serde::de::Error::unknown_field(
                                __value, FIELDS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"client_message" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(_serde::de::Error::unknown_field(
                                    __value, FIELDS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<CheckForMisbehaviourMsg>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = CheckForMisbehaviourMsg;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct CheckForMisbehaviourMsg",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match {
                            #[doc(hidden)]
                            struct __DeserializeWith<'de> {
                                value: Vec<u8>,
                                phantom: _serde::__private::PhantomData<CheckForMisbehaviourMsg>,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::__private::Ok(__DeserializeWith {
                                        value: Base64::deserialize(__deserializer)?,
                                        phantom: _serde::__private::PhantomData,
                                        lifetime: _serde::__private::PhantomData,
                                    })
                                }
                            }
                            _serde::__private::Option::map(
                                _serde::de::SeqAccess::next_element::<__DeserializeWith<'de>>(
                                    &mut __seq,
                                )?,
                                |__wrap| __wrap.value,
                            )
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct CheckForMisbehaviourMsg with 1 element",
                                ))
                            }
                        };
                        _serde::__private::Ok(CheckForMisbehaviourMsg {
                            client_message: __field0,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Vec<u8>> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "client_message",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some({
                                        #[doc(hidden)]
                                        struct __DeserializeWith<'de> {
                                            value: Vec<u8>,
                                            phantom: _serde::__private::PhantomData<
                                                CheckForMisbehaviourMsg,
                                            >,
                                            lifetime: _serde::__private::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::__private::Ok(__DeserializeWith {
                                                    value: Base64::deserialize(__deserializer)?,
                                                    phantom: _serde::__private::PhantomData,
                                                    lifetime: _serde::__private::PhantomData,
                                                })
                                            }
                                        }
                                        match _serde::de::MapAccess::next_value::<
                                            __DeserializeWith<'de>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__wrapper) => __wrapper.value,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    });
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    <__A::Error as _serde::de::Error>::missing_field(
                                        "client_message",
                                    ),
                                )
                            }
                        };
                        _serde::__private::Ok(CheckForMisbehaviourMsg {
                            client_message: __field0,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["client_message"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "CheckForMisbehaviourMsg",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<CheckForMisbehaviourMsg>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for CheckForMisbehaviourMsg {
        #[inline]
        fn clone(&self) -> CheckForMisbehaviourMsg {
            CheckForMisbehaviourMsg {
                client_message: ::core::clone::Clone::clone(&self.client_message),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::fmt::Debug for CheckForMisbehaviourMsg {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "CheckForMisbehaviourMsg",
                "client_message",
                &&self.client_message,
            )
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for CheckForMisbehaviourMsg {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for CheckForMisbehaviourMsg {
        #[inline]
        fn eq(&self, other: &CheckForMisbehaviourMsg) -> bool {
            self.client_message == other.client_message
        }
    }
    const _: () = {
        use ::cosmwasm_schema::schemars;
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for CheckForMisbehaviourMsg {
            fn schema_name() -> std::string::String {
                "CheckForMisbehaviourMsg".to_owned()
            }
            fn schema_id() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed("cf_guest_cw::msg::CheckForMisbehaviourMsg")
            }
            fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                {
                    let mut schema_object = schemars::schema::SchemaObject {
                        instance_type: Some(schemars::schema::InstanceType::Object.into()),
                        ..Default::default()
                    };
                    let object_validation = schema_object.object();
                    object_validation.additional_properties = Some(Box::new(false.into()));
                    {
                        object_validation
                            .properties
                            .insert("client_message".to_owned(), gen.subschema_for::<String>());
                        if !<String as schemars::JsonSchema>::_schemars_private_is_option() {
                            object_validation
                                .required
                                .insert("client_message".to_owned());
                        }
                    }
                    schemars::schema::Schema::Object(schema_object)
                }
            }
        };
    };
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[serde(deny_unknown_fields, crate = "::cosmwasm_schema::serde")]
    #[schemars(crate = "::cosmwasm_schema::schemars")]
    pub struct VerifyStateProofMsg {
        #[serde(with = "Base64")]
        #[schemars(with = "String")]
        pub proof: ibc::CommitmentProofBytes,
        #[serde(with = "AsStr")]
        #[schemars(with = "String")]
        pub path: ibc::path::Path,
        #[serde(with = "MaybeBase64", default, skip_serializing_if = "Option::is_none")]
        #[schemars(with = "String")]
        pub value: Option<Vec<u8>>,
        #[serde(flatten)]
        pub height: Height,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl ::cosmwasm_schema::serde::Serialize for VerifyStateProofMsg {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> ::cosmwasm_schema::serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: ::cosmwasm_schema::serde::Serializer,
            {
                let mut __serde_state =
                    _serde::Serializer::serialize_map(__serializer, _serde::__private::None)?;
                _serde::ser::SerializeMap::serialize_entry(&mut __serde_state, "proof", {
                    #[doc(hidden)]
                    struct __SerializeWith<'__a> {
                        values: (&'__a ibc::CommitmentProofBytes,),
                        phantom: _serde::__private::PhantomData<VerifyStateProofMsg>,
                    }
                    impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                        fn serialize<__S>(
                            &self,
                            __s: __S,
                        ) -> _serde::__private::Result<__S::Ok, __S::Error>
                        where
                            __S: _serde::Serializer,
                        {
                            Base64::serialize(self.values.0, __s)
                        }
                    }
                    &__SerializeWith {
                        values: (&self.proof,),
                        phantom: _serde::__private::PhantomData::<VerifyStateProofMsg>,
                    }
                })?;
                _serde::ser::SerializeMap::serialize_entry(&mut __serde_state, "path", {
                    #[doc(hidden)]
                    struct __SerializeWith<'__a> {
                        values: (&'__a ibc::path::Path,),
                        phantom: _serde::__private::PhantomData<VerifyStateProofMsg>,
                    }
                    impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                        fn serialize<__S>(
                            &self,
                            __s: __S,
                        ) -> _serde::__private::Result<__S::Ok, __S::Error>
                        where
                            __S: _serde::Serializer,
                        {
                            AsStr::serialize(self.values.0, __s)
                        }
                    }
                    &__SerializeWith {
                        values: (&self.path,),
                        phantom: _serde::__private::PhantomData::<VerifyStateProofMsg>,
                    }
                })?;
                if !Option::is_none(&self.value) {
                    _serde::ser::SerializeMap::serialize_entry(&mut __serde_state, "value", {
                        #[doc(hidden)]
                        struct __SerializeWith<'__a> {
                            values: (&'__a Option<Vec<u8>>,),
                            phantom: _serde::__private::PhantomData<VerifyStateProofMsg>,
                        }
                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                            fn serialize<__S>(
                                &self,
                                __s: __S,
                            ) -> _serde::__private::Result<__S::Ok, __S::Error>
                            where
                                __S: _serde::Serializer,
                            {
                                MaybeBase64::serialize(self.values.0, __s)
                            }
                        }
                        &__SerializeWith {
                            values: (&self.value,),
                            phantom: _serde::__private::PhantomData::<VerifyStateProofMsg>,
                        }
                    })?;
                }
                _serde::Serialize::serialize(
                    &&self.height,
                    _serde::__private::ser::FlatMapSerializer(&mut __serde_state),
                )?;
                _serde::ser::SerializeMap::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl<'de> ::cosmwasm_schema::serde::Deserialize<'de> for VerifyStateProofMsg {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> ::cosmwasm_schema::serde::__private::Result<Self, __D::Error>
            where
                __D: ::cosmwasm_schema::serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field<'de> {
                    __field0,
                    __field1,
                    __field2,
                    __other(_serde::__private::de::Content<'de>),
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field<'de>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_bool<__E>(
                        self,
                        __value: bool,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Bool(__value),
                        ))
                    }
                    fn visit_i8<__E>(
                        self,
                        __value: i8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(_serde::__private::de::Content::I8(
                            __value,
                        )))
                    }
                    fn visit_i16<__E>(
                        self,
                        __value: i16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I16(__value),
                        ))
                    }
                    fn visit_i32<__E>(
                        self,
                        __value: i32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I32(__value),
                        ))
                    }
                    fn visit_i64<__E>(
                        self,
                        __value: i64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I64(__value),
                        ))
                    }
                    fn visit_u8<__E>(
                        self,
                        __value: u8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(_serde::__private::de::Content::U8(
                            __value,
                        )))
                    }
                    fn visit_u16<__E>(
                        self,
                        __value: u16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U16(__value),
                        ))
                    }
                    fn visit_u32<__E>(
                        self,
                        __value: u32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U32(__value),
                        ))
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U64(__value),
                        ))
                    }
                    fn visit_f32<__E>(
                        self,
                        __value: f32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::F32(__value),
                        ))
                    }
                    fn visit_f64<__E>(
                        self,
                        __value: f64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::F64(__value),
                        ))
                    }
                    fn visit_char<__E>(
                        self,
                        __value: char,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Char(__value),
                        ))
                    }
                    fn visit_unit<__E>(self) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Unit,
                        ))
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "proof" => _serde::__private::Ok(__Field::__field0),
                            "path" => _serde::__private::Ok(__Field::__field1),
                            "value" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = _serde::__private::de::Content::String(
                                    _serde::__private::ToString::to_string(__value),
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"proof" => _serde::__private::Ok(__Field::__field0),
                            b"path" => _serde::__private::Ok(__Field::__field1),
                            b"value" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value =
                                    _serde::__private::de::Content::ByteBuf(__value.to_vec());
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_str<__E>(
                        self,
                        __value: &'de str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "proof" => _serde::__private::Ok(__Field::__field0),
                            "path" => _serde::__private::Ok(__Field::__field1),
                            "value" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = _serde::__private::de::Content::Str(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_bytes<__E>(
                        self,
                        __value: &'de [u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"proof" => _serde::__private::Ok(__Field::__field0),
                            b"path" => _serde::__private::Ok(__Field::__field1),
                            b"value" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = _serde::__private::de::Content::Bytes(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field<'de> {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<VerifyStateProofMsg>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = VerifyStateProofMsg;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct VerifyStateProofMsg",
                        )
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<ibc::CommitmentProofBytes> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<ibc::path::Path> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Option<Vec<u8>>> =
                            _serde::__private::None;
                        let mut __collect = _serde::__private::Vec::<
                            _serde::__private::Option<(
                                _serde::__private::de::Content,
                                _serde::__private::de::Content,
                            )>,
                        >::new();
                        while let _serde::__private::Some(__key) =
                            _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "proof",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some({
                                        #[doc(hidden)]
                                        struct __DeserializeWith<'de> {
                                            value: ibc::CommitmentProofBytes,
                                            phantom:
                                                _serde::__private::PhantomData<VerifyStateProofMsg>,
                                            lifetime: _serde::__private::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::__private::Ok(__DeserializeWith {
                                                    value: Base64::deserialize(__deserializer)?,
                                                    phantom: _serde::__private::PhantomData,
                                                    lifetime: _serde::__private::PhantomData,
                                                })
                                            }
                                        }
                                        match _serde::de::MapAccess::next_value::<
                                            __DeserializeWith<'de>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__wrapper) => __wrapper.value,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    });
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "path",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some({
                                        #[doc(hidden)]
                                        struct __DeserializeWith<'de> {
                                            value: ibc::path::Path,
                                            phantom:
                                                _serde::__private::PhantomData<VerifyStateProofMsg>,
                                            lifetime: _serde::__private::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::__private::Ok(__DeserializeWith {
                                                    value: AsStr::deserialize(__deserializer)?,
                                                    phantom: _serde::__private::PhantomData,
                                                    lifetime: _serde::__private::PhantomData,
                                                })
                                            }
                                        }
                                        match _serde::de::MapAccess::next_value::<
                                            __DeserializeWith<'de>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__wrapper) => __wrapper.value,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    });
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "value",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some({
                                        #[doc(hidden)]
                                        struct __DeserializeWith<'de> {
                                            value: Option<Vec<u8>>,
                                            phantom:
                                                _serde::__private::PhantomData<VerifyStateProofMsg>,
                                            lifetime: _serde::__private::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::__private::Ok(__DeserializeWith {
                                                    value: MaybeBase64::deserialize(
                                                        __deserializer,
                                                    )?,
                                                    phantom: _serde::__private::PhantomData,
                                                    lifetime: _serde::__private::PhantomData,
                                                })
                                            }
                                        }
                                        match _serde::de::MapAccess::next_value::<
                                            __DeserializeWith<'de>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__wrapper) => __wrapper.value,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    });
                                }
                                __Field::__other(__name) => {
                                    __collect.push(_serde::__private::Some((
                                        __name,
                                        _serde::de::MapAccess::next_value(&mut __map)?,
                                    )));
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    <__A::Error as _serde::de::Error>::missing_field("proof"),
                                )
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    <__A::Error as _serde::de::Error>::missing_field("path"),
                                )
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field3: Height = _serde::de::Deserialize::deserialize(
                            _serde::__private::de::FlatMapDeserializer(
                                &mut __collect,
                                _serde::__private::PhantomData,
                            ),
                        )?;
                        if let _serde::__private::Some(_serde::__private::Some((__key, _))) =
                            __collect
                                .into_iter()
                                .filter(_serde::__private::Option::is_some)
                                .next()
                        {
                            if let _serde::__private::Some(__key) = __key.as_str() {
                                return _serde::__private::Err(_serde::de::Error::custom(
                                    format_args!("unknown field `{0}`", &__key),
                                ));
                            } else {
                                return _serde::__private::Err(_serde::de::Error::custom(
                                    format_args!("unexpected map key"),
                                ));
                            }
                        }
                        _serde::__private::Ok(VerifyStateProofMsg {
                            proof: __field0,
                            path: __field1,
                            value: __field2,
                            height: __field3,
                        })
                    }
                }
                _serde::Deserializer::deserialize_map(
                    __deserializer,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<VerifyStateProofMsg>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for VerifyStateProofMsg {
        #[inline]
        fn clone(&self) -> VerifyStateProofMsg {
            VerifyStateProofMsg {
                proof: ::core::clone::Clone::clone(&self.proof),
                path: ::core::clone::Clone::clone(&self.path),
                value: ::core::clone::Clone::clone(&self.value),
                height: ::core::clone::Clone::clone(&self.height),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::fmt::Debug for VerifyStateProofMsg {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "VerifyStateProofMsg",
                "proof",
                &self.proof,
                "path",
                &self.path,
                "value",
                &self.value,
                "height",
                &&self.height,
            )
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for VerifyStateProofMsg {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for VerifyStateProofMsg {
        #[inline]
        fn eq(&self, other: &VerifyStateProofMsg) -> bool {
            self.proof == other.proof
                && self.path == other.path
                && self.value == other.value
                && self.height == other.height
        }
    }
    const _: () = {
        use ::cosmwasm_schema::schemars;
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for VerifyStateProofMsg {
            fn schema_name() -> std::string::String {
                "VerifyStateProofMsg".to_owned()
            }
            fn schema_id() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed("cf_guest_cw::msg::VerifyStateProofMsg")
            }
            fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                {
                    let mut schema_object = schemars::schema::SchemaObject {
                        instance_type: Some(schemars::schema::InstanceType::Object.into()),
                        ..Default::default()
                    };
                    let object_validation = schema_object.object();
                    object_validation.additional_properties = Some(Box::new(false.into()));
                    {
                        object_validation
                            .properties
                            .insert("proof".to_owned(), gen.subschema_for::<String>());
                        if !<String as schemars::JsonSchema>::_schemars_private_is_option() {
                            object_validation.required.insert("proof".to_owned());
                        }
                    }
                    {
                        object_validation
                            .properties
                            .insert("path".to_owned(), gen.subschema_for::<String>());
                        if !<String as schemars::JsonSchema>::_schemars_private_is_option() {
                            object_validation.required.insert("path".to_owned());
                        }
                    }
                    {
                        object_validation . properties . insert ("value" . to_owned () , { let schema = gen . subschema_for :: < String > () ; schemars :: _private :: apply_metadata (schema , schemars :: schema :: Metadata { default : { struct _SchemarsDefaultSerialize < T > (T) ; impl serde :: Serialize for _SchemarsDefaultSerialize < Option < Vec < u8 > > > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer { MaybeBase64 :: serialize (& self . 0 , serializer) } } { let default = < Option < Vec < u8 > > > :: default () ; if Option :: is_none (& default) { None } else { Some (default) } } . map (| d | _SchemarsDefaultSerialize (d)) } . and_then (| d | { # [allow (unused_imports)] use :: schemars :: _private :: { MaybeSerializeWrapper , NoSerialize as _ , } ; MaybeSerializeWrapper (d) . maybe_to_value () }) , .. Default :: default () }) }) ;
                    }
                    schemars::schema::Schema::Object(schema_object).flatten(
                        schemars::_private::json_schema_for_flatten::<Height>(gen, false),
                    )
                }
            }
        };
    };
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[serde(deny_unknown_fields, crate = "::cosmwasm_schema::serde")]
    #[schemars(crate = "::cosmwasm_schema::schemars")]
    pub struct StatusMsg {}
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl ::cosmwasm_schema::serde::Serialize for StatusMsg {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> ::cosmwasm_schema::serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: ::cosmwasm_schema::serde::Serializer,
            {
                let __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "StatusMsg",
                    false as usize,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl<'de> ::cosmwasm_schema::serde::Deserialize<'de> for StatusMsg {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> ::cosmwasm_schema::serde::__private::Result<Self, __D::Error>
            where
                __D: ::cosmwasm_schema::serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {}
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 0",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::__private::Err(_serde::de::Error::unknown_field(
                                __value, FIELDS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(_serde::de::Error::unknown_field(
                                    __value, FIELDS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<StatusMsg>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = StatusMsg;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct StatusMsg")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        _: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        _serde::__private::Ok(StatusMsg {})
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        _serde::__private::Option::map(
                            _serde::de::MapAccess::next_key::<__Field>(&mut __map)?,
                            |__impossible| match __impossible {},
                        );
                        _serde::__private::Ok(StatusMsg {})
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "StatusMsg",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<StatusMsg>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for StatusMsg {
        #[inline]
        fn clone(&self) -> StatusMsg {
            StatusMsg {}
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::fmt::Debug for StatusMsg {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "StatusMsg")
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for StatusMsg {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for StatusMsg {
        #[inline]
        fn eq(&self, other: &StatusMsg) -> bool {
            true
        }
    }
    const _: () = {
        use ::cosmwasm_schema::schemars;
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for StatusMsg {
            fn schema_name() -> std::string::String {
                "StatusMsg".to_owned()
            }
            fn schema_id() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed("cf_guest_cw::msg::StatusMsg")
            }
            fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                {
                    let mut schema_object = schemars::schema::SchemaObject {
                        instance_type: Some(schemars::schema::InstanceType::Object.into()),
                        ..Default::default()
                    };
                    let object_validation = schema_object.object();
                    object_validation.additional_properties = Some(Box::new(false.into()));
                    schemars::schema::Schema::Object(schema_object)
                }
            }
        };
    };
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[serde(
        deny_unknown_fields,
        rename_all = "snake_case",
        crate = "::cosmwasm_schema::serde"
    )]
    #[schemars(crate = "::cosmwasm_schema::schemars")]
    pub enum StatusResponse {
        Active,
        Expired,
        Frozen,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl ::cosmwasm_schema::serde::Serialize for StatusResponse {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> ::cosmwasm_schema::serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: ::cosmwasm_schema::serde::Serializer,
            {
                match *self {
                    StatusResponse::Active => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "StatusResponse",
                        0u32,
                        "active",
                    ),
                    StatusResponse::Expired => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "StatusResponse",
                        1u32,
                        "expired",
                    ),
                    StatusResponse::Frozen => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "StatusResponse",
                        2u32,
                        "frozen",
                    ),
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl<'de> ::cosmwasm_schema::serde::Deserialize<'de> for StatusResponse {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> ::cosmwasm_schema::serde::__private::Result<Self, __D::Error>
            where
                __D: ::cosmwasm_schema::serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "variant identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"variant index 0 <= i < 3",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "active" => _serde::__private::Ok(__Field::__field0),
                            "expired" => _serde::__private::Ok(__Field::__field1),
                            "frozen" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"active" => _serde::__private::Ok(__Field::__field0),
                            b"expired" => _serde::__private::Ok(__Field::__field1),
                            b"frozen" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<StatusResponse>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = StatusResponse;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "enum StatusResponse")
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match _serde::de::EnumAccess::variant(__data)? {
                            (__Field::__field0, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusResponse::Active)
                            }
                            (__Field::__field1, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusResponse::Expired)
                            }
                            (__Field::__field2, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusResponse::Frozen)
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &["active", "expired", "frozen"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "StatusResponse",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<StatusResponse>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for StatusResponse {
        #[inline]
        fn clone(&self) -> StatusResponse {
            match self {
                StatusResponse::Active => StatusResponse::Active,
                StatusResponse::Expired => StatusResponse::Expired,
                StatusResponse::Frozen => StatusResponse::Frozen,
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::fmt::Debug for StatusResponse {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    StatusResponse::Active => "Active",
                    StatusResponse::Expired => "Expired",
                    StatusResponse::Frozen => "Frozen",
                },
            )
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for StatusResponse {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for StatusResponse {
        #[inline]
        fn eq(&self, other: &StatusResponse) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    const _: () = {
        use ::cosmwasm_schema::schemars;
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for StatusResponse {
            fn schema_name() -> std::string::String {
                "StatusResponse".to_owned()
            }
            fn schema_id() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed("cf_guest_cw::msg::StatusResponse")
            }
            fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                    instance_type: Some(schemars::schema::InstanceType::String.into()),
                    enum_values: Some(<[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            "active".into(),
                            "expired".into(),
                            "frozen".into(),
                        ]),
                    )),
                    ..Default::default()
                })
            }
        };
    };
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[serde(deny_unknown_fields, crate = "::cosmwasm_schema::serde")]
    #[schemars(crate = "::cosmwasm_schema::schemars")]
    pub struct GetLatestHeightsMsg {}
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl ::cosmwasm_schema::serde::Serialize for GetLatestHeightsMsg {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> ::cosmwasm_schema::serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: ::cosmwasm_schema::serde::Serializer,
            {
                let __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "GetLatestHeightsMsg",
                    false as usize,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl<'de> ::cosmwasm_schema::serde::Deserialize<'de> for GetLatestHeightsMsg {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> ::cosmwasm_schema::serde::__private::Result<Self, __D::Error>
            where
                __D: ::cosmwasm_schema::serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {}
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 0",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::__private::Err(_serde::de::Error::unknown_field(
                                __value, FIELDS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(_serde::de::Error::unknown_field(
                                    __value, FIELDS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<GetLatestHeightsMsg>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = GetLatestHeightsMsg;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct GetLatestHeightsMsg",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        _: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        _serde::__private::Ok(GetLatestHeightsMsg {})
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        _serde::__private::Option::map(
                            _serde::de::MapAccess::next_key::<__Field>(&mut __map)?,
                            |__impossible| match __impossible {},
                        );
                        _serde::__private::Ok(GetLatestHeightsMsg {})
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "GetLatestHeightsMsg",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<GetLatestHeightsMsg>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for GetLatestHeightsMsg {
        #[inline]
        fn clone(&self) -> GetLatestHeightsMsg {
            GetLatestHeightsMsg {}
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::fmt::Debug for GetLatestHeightsMsg {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "GetLatestHeightsMsg")
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for GetLatestHeightsMsg {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for GetLatestHeightsMsg {
        #[inline]
        fn eq(&self, other: &GetLatestHeightsMsg) -> bool {
            true
        }
    }
    const _: () = {
        use ::cosmwasm_schema::schemars;
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for GetLatestHeightsMsg {
            fn schema_name() -> std::string::String {
                "GetLatestHeightsMsg".to_owned()
            }
            fn schema_id() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed("cf_guest_cw::msg::GetLatestHeightsMsg")
            }
            fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                {
                    let mut schema_object = schemars::schema::SchemaObject {
                        instance_type: Some(schemars::schema::InstanceType::Object.into()),
                        ..Default::default()
                    };
                    let object_validation = schema_object.object();
                    object_validation.additional_properties = Some(Box::new(false.into()));
                    schemars::schema::Schema::Object(schema_object)
                }
            }
        };
    };
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[serde(deny_unknown_fields, crate = "::cosmwasm_schema::serde")]
    #[schemars(crate = "::cosmwasm_schema::schemars")]
    pub struct TimestampAtHeightMsg {
        pub height: Height,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl ::cosmwasm_schema::serde::Serialize for TimestampAtHeightMsg {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> ::cosmwasm_schema::serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: ::cosmwasm_schema::serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "TimestampAtHeightMsg",
                    false as usize + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "height",
                    &self.height,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl<'de> ::cosmwasm_schema::serde::Deserialize<'de> for TimestampAtHeightMsg {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> ::cosmwasm_schema::serde::__private::Result<Self, __D::Error>
            where
                __D: ::cosmwasm_schema::serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 1",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "height" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Err(_serde::de::Error::unknown_field(
                                __value, FIELDS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"height" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(_serde::de::Error::unknown_field(
                                    __value, FIELDS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<TimestampAtHeightMsg>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = TimestampAtHeightMsg;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct TimestampAtHeightMsg",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match _serde::de::SeqAccess::next_element::<Height>(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct TimestampAtHeightMsg with 1 element",
                                        ),
                                    )
                                }
                            };
                        _serde::__private::Ok(TimestampAtHeightMsg { height: __field0 })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Height> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "height",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Height>(&mut __map)?,
                                    );
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("height")?
                            }
                        };
                        _serde::__private::Ok(TimestampAtHeightMsg { height: __field0 })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["height"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "TimestampAtHeightMsg",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<TimestampAtHeightMsg>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for TimestampAtHeightMsg {
        #[inline]
        fn clone(&self) -> TimestampAtHeightMsg {
            TimestampAtHeightMsg {
                height: ::core::clone::Clone::clone(&self.height),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::fmt::Debug for TimestampAtHeightMsg {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "TimestampAtHeightMsg",
                "height",
                &&self.height,
            )
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for TimestampAtHeightMsg {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for TimestampAtHeightMsg {
        #[inline]
        fn eq(&self, other: &TimestampAtHeightMsg) -> bool {
            self.height == other.height
        }
    }
    const _: () = {
        use ::cosmwasm_schema::schemars;
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for TimestampAtHeightMsg {
            fn schema_name() -> std::string::String {
                "TimestampAtHeightMsg".to_owned()
            }
            fn schema_id() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed("cf_guest_cw::msg::TimestampAtHeightMsg")
            }
            fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                {
                    let mut schema_object = schemars::schema::SchemaObject {
                        instance_type: Some(schemars::schema::InstanceType::Object.into()),
                        ..Default::default()
                    };
                    let object_validation = schema_object.object();
                    object_validation.additional_properties = Some(Box::new(false.into()));
                    {
                        object_validation
                            .properties
                            .insert("height".to_owned(), gen.subschema_for::<Height>());
                        if !<Height as schemars::JsonSchema>::_schemars_private_is_option() {
                            object_validation.required.insert("height".to_owned());
                        }
                    }
                    schemars::schema::Schema::Object(schema_object)
                }
            }
        };
    };
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[serde(deny_unknown_fields, crate = "::cosmwasm_schema::serde")]
    #[schemars(crate = "::cosmwasm_schema::schemars")]
    pub struct ExportMetadataMsg {}
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl ::cosmwasm_schema::serde::Serialize for ExportMetadataMsg {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> ::cosmwasm_schema::serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: ::cosmwasm_schema::serde::Serializer,
            {
                let __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "ExportMetadataMsg",
                    false as usize,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl<'de> ::cosmwasm_schema::serde::Deserialize<'de> for ExportMetadataMsg {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> ::cosmwasm_schema::serde::__private::Result<Self, __D::Error>
            where
                __D: ::cosmwasm_schema::serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {}
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 0",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::__private::Err(_serde::de::Error::unknown_field(
                                __value, FIELDS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(_serde::de::Error::unknown_field(
                                    __value, FIELDS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ExportMetadataMsg>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ExportMetadataMsg;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct ExportMetadataMsg",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        _: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        _serde::__private::Ok(ExportMetadataMsg {})
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        _serde::__private::Option::map(
                            _serde::de::MapAccess::next_key::<__Field>(&mut __map)?,
                            |__impossible| match __impossible {},
                        );
                        _serde::__private::Ok(ExportMetadataMsg {})
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "ExportMetadataMsg",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ExportMetadataMsg>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for ExportMetadataMsg {
        #[inline]
        fn clone(&self) -> ExportMetadataMsg {
            ExportMetadataMsg {}
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::fmt::Debug for ExportMetadataMsg {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "ExportMetadataMsg")
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ExportMetadataMsg {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for ExportMetadataMsg {
        #[inline]
        fn eq(&self, other: &ExportMetadataMsg) -> bool {
            true
        }
    }
    const _: () = {
        use ::cosmwasm_schema::schemars;
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for ExportMetadataMsg {
            fn schema_name() -> std::string::String {
                "ExportMetadataMsg".to_owned()
            }
            fn schema_id() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed("cf_guest_cw::msg::ExportMetadataMsg")
            }
            fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                {
                    let mut schema_object = schemars::schema::SchemaObject {
                        instance_type: Some(schemars::schema::InstanceType::Object.into()),
                        ..Default::default()
                    };
                    let object_validation = schema_object.object();
                    object_validation.additional_properties = Some(Box::new(false.into()));
                    schemars::schema::Schema::Object(schema_object)
                }
            }
        };
    };
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[serde(deny_unknown_fields, crate = "::cosmwasm_schema::serde")]
    #[schemars(crate = "::cosmwasm_schema::schemars")]
    pub struct ConsensusStateMetadata {
        pub height: Height,
        pub host_timestamp_ns: Uint64,
        pub host_height: Uint64,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl ::cosmwasm_schema::serde::Serialize for ConsensusStateMetadata {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> ::cosmwasm_schema::serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: ::cosmwasm_schema::serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "ConsensusStateMetadata",
                    false as usize + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "height",
                    &self.height,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "host_timestamp_ns",
                    &self.host_timestamp_ns,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "host_height",
                    &self.host_height,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use ::cosmwasm_schema::serde as _serde;
        #[automatically_derived]
        impl<'de> ::cosmwasm_schema::serde::Deserialize<'de> for ConsensusStateMetadata {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> ::cosmwasm_schema::serde::__private::Result<Self, __D::Error>
            where
                __D: ::cosmwasm_schema::serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 3",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "height" => _serde::__private::Ok(__Field::__field0),
                            "host_timestamp_ns" => _serde::__private::Ok(__Field::__field1),
                            "host_height" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Err(_serde::de::Error::unknown_field(
                                __value, FIELDS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"height" => _serde::__private::Ok(__Field::__field0),
                            b"host_timestamp_ns" => _serde::__private::Ok(__Field::__field1),
                            b"host_height" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(_serde::de::Error::unknown_field(
                                    __value, FIELDS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ConsensusStateMetadata>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ConsensusStateMetadata;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct ConsensusStateMetadata",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match _serde::de::SeqAccess::next_element::<Height>(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct ConsensusStateMetadata with 3 elements",
                                        ),
                                    )
                                }
                            };
                        let __field1 =
                            match _serde::de::SeqAccess::next_element::<Uint64>(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct ConsensusStateMetadata with 3 elements",
                                        ),
                                    )
                                }
                            };
                        let __field2 =
                            match _serde::de::SeqAccess::next_element::<Uint64>(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct ConsensusStateMetadata with 3 elements",
                                        ),
                                    )
                                }
                            };
                        _serde::__private::Ok(ConsensusStateMetadata {
                            height: __field0,
                            host_timestamp_ns: __field1,
                            host_height: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Height> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Uint64> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Uint64> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "height",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Height>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "host_timestamp_ns",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Uint64>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "host_height",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Uint64>(&mut __map)?,
                                    );
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("height")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("host_timestamp_ns")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("host_height")?
                            }
                        };
                        _serde::__private::Ok(ConsensusStateMetadata {
                            height: __field0,
                            host_timestamp_ns: __field1,
                            host_height: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] =
                    &["height", "host_timestamp_ns", "host_height"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "ConsensusStateMetadata",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ConsensusStateMetadata>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::clone::Clone for ConsensusStateMetadata {
        #[inline]
        fn clone(&self) -> ConsensusStateMetadata {
            ConsensusStateMetadata {
                height: ::core::clone::Clone::clone(&self.height),
                host_timestamp_ns: ::core::clone::Clone::clone(&self.host_timestamp_ns),
                host_height: ::core::clone::Clone::clone(&self.host_height),
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::fmt::Debug for ConsensusStateMetadata {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "ConsensusStateMetadata",
                "height",
                &self.height,
                "host_timestamp_ns",
                &self.host_timestamp_ns,
                "host_height",
                &&self.host_height,
            )
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ConsensusStateMetadata {}
    #[automatically_derived]
    #[allow(clippy::derive_partial_eq_without_eq)]
    impl ::core::cmp::PartialEq for ConsensusStateMetadata {
        #[inline]
        fn eq(&self, other: &ConsensusStateMetadata) -> bool {
            self.height == other.height
                && self.host_timestamp_ns == other.host_timestamp_ns
                && self.host_height == other.host_height
        }
    }
    const _: () = {
        use ::cosmwasm_schema::schemars;
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for ConsensusStateMetadata {
            fn schema_name() -> std::string::String {
                "ConsensusStateMetadata".to_owned()
            }
            fn schema_id() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed("cf_guest_cw::msg::ConsensusStateMetadata")
            }
            fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                {
                    let mut schema_object = schemars::schema::SchemaObject {
                        instance_type: Some(schemars::schema::InstanceType::Object.into()),
                        ..Default::default()
                    };
                    let object_validation = schema_object.object();
                    object_validation.additional_properties = Some(Box::new(false.into()));
                    {
                        object_validation
                            .properties
                            .insert("height".to_owned(), gen.subschema_for::<Height>());
                        if !<Height as schemars::JsonSchema>::_schemars_private_is_option() {
                            object_validation.required.insert("height".to_owned());
                        }
                    }
                    {
                        object_validation.properties.insert(
                            "host_timestamp_ns".to_owned(),
                            gen.subschema_for::<Uint64>(),
                        );
                        if !<Uint64 as schemars::JsonSchema>::_schemars_private_is_option() {
                            object_validation
                                .required
                                .insert("host_timestamp_ns".to_owned());
                        }
                    }
                    {
                        object_validation
                            .properties
                            .insert("host_height".to_owned(), gen.subschema_for::<Uint64>());
                        if !<Uint64 as schemars::JsonSchema>::_schemars_private_is_option() {
                            object_validation.required.insert("host_height".to_owned());
                        }
                    }
                    schemars::schema::Schema::Object(schema_object)
                }
            }
        };
    };
    fn is_zero(num: &Uint64) -> bool {
        u64::from(*num) == 0
    }
    /// IBC height.
    ///
    /// This is essentially a copy of [`ibc::Height`] which we have so that we can
    /// implement `JsonSchema` on it without having to enable `schema` feature on
    /// `ibc` which pulls in `std` which we don’t want.
    #[display(fmt = "{}-{}", revision_number, revision_height)]
    pub struct Height {
        /// Previously known as "epoch"
        #[serde(default, skip_serializing_if = "is_zero")]
        pub revision_number: Uint64,
        /// The height of a block
        pub revision_height: Uint64,
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Height {}
    #[automatically_derived]
    impl ::core::clone::Clone for Height {
        #[inline]
        fn clone(&self) -> Height {
            let _: ::core::clone::AssertParamIsClone<Uint64>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Height {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Height {
        #[inline]
        fn eq(&self, other: &Height) -> bool {
            self.revision_number == other.revision_number
                && self.revision_height == other.revision_height
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Height {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Height {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Uint64>;
        }
    }
    impl ::core::fmt::Display for Height {
        #[allow(unused_variables)]
        #[inline]
        fn fmt(
            &self,
            _derive_more_display_formatter: &mut ::core::fmt::Formatter,
        ) -> ::core::fmt::Result {
            match self {
                Height {
                    revision_number,
                    revision_height,
                } => _derive_more_display_formatter
                    .write_fmt(format_args!("{0}-{1}", revision_number, revision_height)),
                _ => Ok(()),
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Height {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Height",
                    false as usize + if is_zero(&self.revision_number) { 0 } else { 1 } + 1,
                )?;
                if !is_zero(&self.revision_number) {
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "revision_number",
                        &self.revision_number,
                    )?;
                } else {
                    _serde::ser::SerializeStruct::skip_field(
                        &mut __serde_state,
                        "revision_number",
                    )?;
                }
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "revision_height",
                    &self.revision_height,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Height {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "revision_number" => _serde::__private::Ok(__Field::__field0),
                            "revision_height" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"revision_number" => _serde::__private::Ok(__Field::__field0),
                            b"revision_height" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Height>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Height;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct Height")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match _serde::de::SeqAccess::next_element::<Uint64>(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => _serde::__private::Default::default(),
                            };
                        let __field1 =
                            match _serde::de::SeqAccess::next_element::<Uint64>(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Height with 2 elements",
                                        ),
                                    )
                                }
                            };
                        _serde::__private::Ok(Height {
                            revision_number: __field0,
                            revision_height: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Uint64> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Uint64> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "revision_number",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Uint64>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "revision_height",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Uint64>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("revision_height")?
                            }
                        };
                        _serde::__private::Ok(Height {
                            revision_number: __field0,
                            revision_height: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["revision_number", "revision_height"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Height",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Height>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    const _: () = {
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for Height {
            fn schema_name() -> std::string::String {
                "Height".to_owned()
            }
            fn schema_id() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed("cf_guest_cw::msg::Height")
            }
            fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                {
                    let schema = {
                        let mut schema_object = schemars::schema::SchemaObject {
                            instance_type: Some(schemars::schema::InstanceType::Object.into()),
                            ..Default::default()
                        };
                        let object_validation = schema_object.object();
                        {
                            object_validation
                                .properties
                                .insert("revision_number".to_owned(), {
                                    let schema = gen.subschema_for::<Uint64>();
                                    schemars::_private::apply_metadata(
                                        schema,
                                        schemars::schema::Metadata {
                                            description: Some(
                                                "Previously known as \"epoch\"".to_owned(),
                                            ),
                                            default: {
                                                let default = <Uint64>::default();
                                                if is_zero(&default) {
                                                    None
                                                } else {
                                                    Some(default)
                                                }
                                            }
                                            .and_then(|d| {
                                                #[allow(unused_imports)]
                                                use ::schemars::_private::{
                                                    MaybeSerializeWrapper, NoSerialize as _,
                                                };
                                                MaybeSerializeWrapper(d).maybe_to_value()
                                            }),
                                            ..Default::default()
                                        },
                                    )
                                });
                        }
                        {
                            object_validation
                                .properties
                                .insert("revision_height".to_owned(), {
                                    let schema = gen.subschema_for::<Uint64>();
                                    schemars::_private::apply_metadata(
                                        schema,
                                        schemars::schema::Metadata {
                                            description: Some("The height of a block".to_owned()),
                                            ..Default::default()
                                        },
                                    )
                                });
                            if !<Uint64 as schemars::JsonSchema>::_schemars_private_is_option() {
                                object_validation
                                    .required
                                    .insert("revision_height".to_owned());
                            }
                        }
                        schemars::schema::Schema::Object(schema_object)
                    };
                    schemars :: _private :: apply_metadata (schema , schemars :: schema :: Metadata { description : Some ("IBC height.\n\nThis is essentially a copy of [`ibc::Height`] which we have so that we can implement `JsonSchema` on it without having to enable `schema` feature on `ibc` which pulls in `std` which we don’t want." . to_owned ()) , .. Default :: default () })
                }
            }
        };
    };
    impl TryFrom<Height> for ibc::Height {
        type Error = cosmwasm_std::StdError;
        fn try_from(height: Height) -> Result<Self, Self::Error> {
            Ok(ibc::Height::new(
                height.revision_number.into(),
                height.revision_height.into(),
            ))
        }
    }
    impl From<ibc::Height> for Height {
        fn from(height: ibc::Height) -> Self {
            Self {
                revision_number: height.revision_number.into(),
                revision_height: height.revision_height.into(),
            }
        }
    }
    impl core::fmt::Debug for Height {
        fn fmt(&self, fmtr: &mut core::fmt::Formatter) -> core::fmt::Result {
            core::fmt::Display::fmt(self, fmtr)
        }
    }
}
mod serialisation {
    use alloc::borrow::Cow;
    use core::{fmt, marker::PhantomData, str::FromStr};
    use cosmwasm_std::Binary;
    use prost::Message;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use crate::{ibc, state};
    /// A Serde serialisation implementation which encodes binary data as
    /// base64-string (when serialising to human-readable form).
    pub struct Base64;
    /// A Serde serialisation implementation handling `Option<T>` values where `T`
    /// can be serialised using [`Base64`].
    pub struct MaybeBase64;
    /// A Serde serialisation implementation which encodes object using
    /// `Display` and deserialises using `FromStr`.
    pub struct AsStr;
    impl Base64 {
        pub fn serialize<T: BytesConv, S: Serializer>(obj: &T, ser: S) -> Result<S::Ok, S::Error> {
            let bytes = obj.to_bytes()?;
            Base64Bytes(bytes.as_ref()).serialize(ser)
        }
        pub fn deserialize<'de, T: BytesConv, D: Deserializer<'de>>(de: D) -> Result<T, D::Error> {
            T::from_bytes(Binary::deserialize(de)?.into())
        }
    }
    impl MaybeBase64 {
        pub fn serialize<T: BytesConv, S: Serializer>(
            obj: &Option<T>,
            ser: S,
        ) -> Result<S::Ok, S::Error> {
            if let Some(ref obj) = obj {
                let bytes = obj.to_bytes()?;
                ser.serialize_some(&Base64Bytes(bytes.as_ref()))
            } else {
                ser.serialize_none()
            }
        }
        pub fn deserialize<'de, T: BytesConv, D: Deserializer<'de>>(
            de: D,
        ) -> Result<Option<T>, D::Error> {
            match Option::<Binary>::deserialize(de)? {
                None => Ok(None),
                Some(bin) => T::from_bytes(bin.into()).map(Some),
            }
        }
    }
    /// Wrapper which serialised bytes slice using base64 encoding.
    struct Base64Bytes<'a>(&'a [u8]);
    impl Serialize for Base64Bytes<'_> {
        fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            use base64::engine::{general_purpose::STANDARD, Engine};
            ser.serialize_str(&STANDARD.encode(self.0))
        }
    }
    /// Trait implementing conversion to and from bytes used by [`Base64`] and
    /// [`MaybeBase64`].
    pub trait BytesConv: Sized {
        fn to_bytes<'a, E: serde::ser::Error>(&'a self) -> Result<Cow<'a, [u8]>, E>;
        fn from_bytes<E: serde::de::Error>(bytes: Vec<u8>) -> Result<Self, E>;
    }
    impl BytesConv for Vec<u8> {
        fn to_bytes<'a, E: serde::ser::Error>(&'a self) -> Result<Cow<'a, [u8]>, E> {
            Ok(Cow::Borrowed(self.as_slice()))
        }
        fn from_bytes<E: serde::de::Error>(bytes: Vec<u8>) -> Result<Self, E> {
            Ok(bytes)
        }
    }
    impl BytesConv for ibc::CommitmentProofBytes {
        fn to_bytes<'a, E: serde::ser::Error>(&'a self) -> Result<Cow<'a, [u8]>, E> {
            Ok(Cow::Borrowed(self.as_bytes()))
        }
        fn from_bytes<E: serde::de::Error>(bytes: Vec<u8>) -> Result<Self, E> {
            Self::try_from(bytes).map_err(E::custom)
        }
    }
    impl BytesConv for state::ClientState {
        fn to_bytes<'a, E: serde::ser::Error>(&'a self) -> Result<Cow<'a, [u8]>, E> {
            Ok(Cow::Owned(
                ibc::proto::google::protobuf::Any::from(self).encode_to_vec(),
            ))
        }
        fn from_bytes<E: serde::de::Error>(bytes: Vec<u8>) -> Result<Self, E> {
            let any =
                ibc::proto::google::protobuf::Any::decode(bytes.as_slice()).map_err(E::custom)?;
            <state::ClientState>::try_from(any).map_err(E::custom)
        }
    }
    impl BytesConv for state::ConsensusState {
        fn to_bytes<'a, E: serde::ser::Error>(&'a self) -> Result<Cow<'a, [u8]>, E> {
            Ok(Cow::Owned(
                ibc::proto::google::protobuf::Any::from(self).encode_to_vec(),
            ))
        }
        fn from_bytes<E: serde::de::Error>(bytes: Vec<u8>) -> Result<Self, E> {
            let any =
                ibc::proto::google::protobuf::Any::decode(bytes.as_slice()).map_err(E::custom)?;
            <state::ConsensusState>::try_from(any).map_err(E::custom)
        }
    }
    impl BytesConv for state::Header {
        fn to_bytes<'a, E: serde::ser::Error>(&'a self) -> Result<Cow<'a, [u8]>, E> {
            Ok(Cow::Owned(
                ibc::proto::google::protobuf::Any::from(self).encode_to_vec(),
            ))
        }
        fn from_bytes<E: serde::de::Error>(bytes: Vec<u8>) -> Result<Self, E> {
            let any =
                ibc::proto::google::protobuf::Any::decode(bytes.as_slice()).map_err(E::custom)?;
            <state::Header>::try_from(any).map_err(E::custom)
        }
    }
    impl BytesConv for state::Misbehaviour {
        fn to_bytes<'a, E: serde::ser::Error>(&'a self) -> Result<Cow<'a, [u8]>, E> {
            Ok(Cow::Owned(
                ibc::proto::google::protobuf::Any::from(self).encode_to_vec(),
            ))
        }
        fn from_bytes<E: serde::de::Error>(bytes: Vec<u8>) -> Result<Self, E> {
            let any =
                ibc::proto::google::protobuf::Any::decode(bytes.as_slice()).map_err(E::custom)?;
            <state::Misbehaviour>::try_from(any).map_err(E::custom)
        }
    }
    impl AsStr {
        pub fn serialize<T: fmt::Display, S: Serializer>(
            obj: &T,
            ser: S,
        ) -> Result<S::Ok, S::Error> {
            ser.serialize_str(&obj.to_string())
        }
        pub fn deserialize<'de, T, E, D>(de: D) -> Result<T, D::Error>
        where
            T: FromStr<Err = E>,
            E: fmt::Display,
            D: Deserializer<'de>,
        {
            de.deserialize_str(AsStrVisitor::<T>::default())
        }
    }
    struct AsStrVisitor<T>(PhantomData<T>);
    impl<T> Default for AsStrVisitor<T> {
        fn default() -> Self {
            Self(PhantomData)
        }
    }
    impl<'de, T, Err> serde::de::Visitor<'de> for AsStrVisitor<T>
    where
        T: FromStr<Err = Err>,
        Err: fmt::Display,
    {
        type Value = T;
        fn expecting(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
            fmtr.write_fmt(format_args!("object formatted to string"))
        }
        fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<T, E> {
            T::from_str(value).map_err(E::custom)
        }
    }
}
pub mod state {
    use cosmwasm_std::Storage;
    use prost::Message;
    use crate::{ibc, ibc::proto::google::protobuf::Any};
    type Result<T, E = crate::Error> = core::result::Result<T, E>;
    pub type ClientMessage = cf_guest::ClientMessage<crate::PubKey>;
    pub type ClientState = cf_guest::ClientState<crate::PubKey>;
    pub type ConsensusState = cf_guest::ConsensusState;
    pub type Header = cf_guest::Header<crate::PubKey>;
    pub type Misbehaviour = cf_guest::Misbehaviour<crate::PubKey>;
    pub(crate) struct Metadata {
        pub host_timestamp_ns: u64,
        pub host_height: u64,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Metadata {
        #[inline]
        fn clone(&self) -> Metadata {
            let _: ::core::clone::AssertParamIsClone<u64>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Metadata {}
    #[automatically_derived]
    impl ::core::fmt::Debug for Metadata {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Metadata",
                "host_timestamp_ns",
                &self.host_timestamp_ns,
                "host_height",
                &&self.host_height,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Metadata {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Metadata {
        #[inline]
        fn eq(&self, other: &Metadata) -> bool {
            self.host_timestamp_ns == other.host_timestamp_ns
                && self.host_height == other.host_height
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Metadata {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Metadata {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u64>;
        }
    }
    /// Wrapper for accessing client state saved in the storage.
    #[repr(transparent)]
    pub(crate) struct ClientStates(dyn Storage);
    impl ClientStates {
        pub fn new<'a>(storage: &'a mut dyn Storage) -> &'a mut Self {
            unsafe { wrap_mut(storage) }
        }
        pub fn new_ro<'a>(storage: &'a dyn Storage) -> &'a Self {
            unsafe { wrap_ref(storage) }
        }
        pub fn get<T, E>(&self) -> Result<Option<T>, E>
        where
            T: TryFrom<Any>,
            E: From<T::Error> + From<prost::DecodeError>,
        {
            self.get_impl(Self::KEY)
        }
        pub fn set(&mut self, state: impl Into<Any>) {
            self.set_impl(Self::KEY, state)
        }
        const KEY: &'static [u8] = b"clientState/";
        fn get_impl<T, E>(&self, key: &[u8]) -> Result<Option<T>, E>
        where
            T: TryFrom<Any>,
            E: From<T::Error> + From<prost::DecodeError>,
        {
            self.0
                .get(&key)
                .map(|value| {
                    let any = Any::decode(value.as_slice())?;
                    T::try_from(any).map_err(|err| err.into())
                })
                .transpose()
        }
        fn set_impl(&mut self, key: &[u8], state: impl Into<Any>) {
            self.0.set(&key, state.into().encode_to_vec().as_slice())
        }
    }
    /// Wrapper for accessing consensus state saved in the storage.
    #[repr(transparent)]
    pub(crate) struct ConsensusStates(dyn Storage);
    impl ConsensusStates {
        pub fn new<'a>(storage: &'a mut dyn Storage) -> &'a mut Self {
            unsafe { wrap_mut(storage) }
        }
        pub fn new_ro<'a>(storage: &'a dyn Storage) -> &'a Self {
            unsafe { wrap_ref(storage) }
        }
        pub fn get<T, E>(&self, height: ibc::Height) -> Result<Option<(T, Metadata)>, E>
        where
            T: TryFrom<Any>,
            E: From<T::Error> + From<prost::DecodeError>,
        {
            self.get_impl(&Self::key(height))
        }
        pub fn set(&mut self, height: ibc::Height, state: impl Into<Any>, metadata: Metadata) {
            self.set_impl(Self::key(height), state, metadata)
        }
        fn all<'a>(
            &'a self,
        ) -> impl Iterator<Item = Result<(Vec<u8>, Any, Metadata), prost::DecodeError>> + 'a
        {
            self.0
                .range(
                    Some(Self::key_impl(0, 0).as_slice()),
                    Some(Self::key_impl(u64::MAX, u64::MAX).as_slice()),
                    cosmwasm_std::Order::Ascending,
                )
                .map(|(key, value)| {
                    let (any, metadata) =
                        ConsensusWithMetadata::decode(value.as_slice())?.into_parts();
                    Ok((key, any, metadata))
                })
        }
        pub fn prune_oldest_consensus_state(
            &mut self,
            client_state: &ClientState,
            now_ns: u64,
        ) -> Result<()> {
            let (key, any) = match self.all().next() {
                None => return Ok(()),
                Some(Err(err)) => return Err(err.into()),
                Some(Ok((key, any, _metadata))) => (key, any),
            };
            let state = ConsensusState::try_from(any)?;
            let elapsed = now_ns.saturating_sub(state.timestamp_ns.get());
            if elapsed >= client_state.trusting_period_ns {
                self.0.remove(key.as_slice());
            }
            Ok(())
        }
        pub fn get_all_metadata(&self) -> Result<Vec<crate::msg::ConsensusStateMetadata>> {
            let mut records = Vec::new();
            for record in self.all() {
                let (key, _state, metadata) = record?;
                let key = &key[key.len() - 16..];
                records.push(crate::msg::ConsensusStateMetadata {
                    height: crate::msg::Height {
                        revision_number: u64::from_be_bytes(key[..8].try_into().unwrap()).into(),
                        revision_height: u64::from_be_bytes(key[8..].try_into().unwrap()).into(),
                    },
                    host_timestamp_ns: metadata.host_timestamp_ns.into(),
                    host_height: metadata.host_height.into(),
                })
            }
            Ok(records)
        }
        pub fn del(&mut self, height: ibc::Height) {
            self.0.remove(&Self::key(height))
        }
        fn key(height: ibc::Height) -> Vec<u8> {
            Self::key_impl(height.revision_number, height.revision_height)
        }
        fn key_impl(rev_number: u64, rev_height: u64) -> Vec<u8> {
            let rev_number = rev_number.to_be_bytes();
            let rev_height = rev_height.to_be_bytes();
            [b"consensusState/", &rev_number[..], &rev_height[..]].concat()
        }
        fn get_impl<T, E>(&self, key: &[u8]) -> Result<Option<(T, Metadata)>, E>
        where
            T: TryFrom<Any>,
            E: From<T::Error> + From<prost::DecodeError>,
        {
            let value = match self.0.get(&key) {
                None => return Ok(None),
                Some(value) => value,
            };
            let (any, metadata) = ConsensusWithMetadata::decode(value.as_slice())?.into_parts();
            Ok(Some((T::try_from(any)?, metadata)))
        }
        fn set_impl(&mut self, key: Vec<u8>, state: impl Into<Any>, metadata: Metadata) {
            let state = ConsensusWithMetadata::new(state, metadata);
            self.0.set(&key, state.encode_to_vec().as_slice())
        }
    }
    /// Extension of protobuf’s Any type to include host height and host timestamp.
    struct ConsensusWithMetadata {
        #[prost(string, tag = "1")]
        pub type_url: ::prost::alloc::string::String,
        #[prost(bytes = "vec", tag = "2")]
        pub value: ::prost::alloc::vec::Vec<u8>,
        #[prost(uint64, tag = "3")]
        pub host_timestamp_ns: u64,
        #[prost(uint64, tag = "4")]
        pub host_height: u64,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ConsensusWithMetadata {
        #[inline]
        fn clone(&self) -> ConsensusWithMetadata {
            ConsensusWithMetadata {
                type_url: ::core::clone::Clone::clone(&self.type_url),
                value: ::core::clone::Clone::clone(&self.value),
                host_timestamp_ns: ::core::clone::Clone::clone(&self.host_timestamp_ns),
                host_height: ::core::clone::Clone::clone(&self.host_height),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ConsensusWithMetadata {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ConsensusWithMetadata {
        #[inline]
        fn eq(&self, other: &ConsensusWithMetadata) -> bool {
            self.type_url == other.type_url
                && self.value == other.value
                && self.host_timestamp_ns == other.host_timestamp_ns
                && self.host_height == other.host_height
        }
    }
    impl ::prost::Message for ConsensusWithMetadata {
        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::prost::bytes::BufMut,
        {
            if self.type_url != "" {
                ::prost::encoding::string::encode(1u32, &self.type_url, buf);
            }
            if self.value != b"" as &[u8] {
                ::prost::encoding::bytes::encode(2u32, &self.value, buf);
            }
            if self.host_timestamp_ns != 0u64 {
                ::prost::encoding::uint64::encode(3u32, &self.host_timestamp_ns, buf);
            }
            if self.host_height != 0u64 {
                ::prost::encoding::uint64::encode(4u32, &self.host_height, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field<B>(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::WireType,
            buf: &mut B,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError>
        where
            B: ::prost::bytes::Buf,
        {
            const STRUCT_NAME: &'static str = "ConsensusWithMetadata";
            match tag {
                1u32 => {
                    let mut value = &mut self.type_url;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx).map_err(
                        |mut error| {
                            error.push(STRUCT_NAME, "type_url");
                            error
                        },
                    )
                }
                2u32 => {
                    let mut value = &mut self.value;
                    ::prost::encoding::bytes::merge(wire_type, value, buf, ctx).map_err(
                        |mut error| {
                            error.push(STRUCT_NAME, "value");
                            error
                        },
                    )
                }
                3u32 => {
                    let mut value = &mut self.host_timestamp_ns;
                    ::prost::encoding::uint64::merge(wire_type, value, buf, ctx).map_err(
                        |mut error| {
                            error.push(STRUCT_NAME, "host_timestamp_ns");
                            error
                        },
                    )
                }
                4u32 => {
                    let mut value = &mut self.host_height;
                    ::prost::encoding::uint64::merge(wire_type, value, buf, ctx).map_err(
                        |mut error| {
                            error.push(STRUCT_NAME, "host_height");
                            error
                        },
                    )
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + if self.type_url != "" {
                ::prost::encoding::string::encoded_len(1u32, &self.type_url)
            } else {
                0
            } + if self.value != b"" as &[u8] {
                ::prost::encoding::bytes::encoded_len(2u32, &self.value)
            } else {
                0
            } + if self.host_timestamp_ns != 0u64 {
                ::prost::encoding::uint64::encoded_len(3u32, &self.host_timestamp_ns)
            } else {
                0
            } + if self.host_height != 0u64 {
                ::prost::encoding::uint64::encoded_len(4u32, &self.host_height)
            } else {
                0
            }
        }
        fn clear(&mut self) {
            self.type_url.clear();
            self.value.clear();
            self.host_timestamp_ns = 0u64;
            self.host_height = 0u64;
        }
    }
    impl ::core::default::Default for ConsensusWithMetadata {
        fn default() -> Self {
            ConsensusWithMetadata {
                type_url: ::prost::alloc::string::String::new(),
                value: ::core::default::Default::default(),
                host_timestamp_ns: 0u64,
                host_height: 0u64,
            }
        }
    }
    impl ::core::fmt::Debug for ConsensusWithMetadata {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("ConsensusWithMetadata");
            let builder = {
                let wrapper = {
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.type_url)
                };
                builder.field("type_url", &wrapper)
            };
            let builder = {
                let wrapper = {
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.value)
                };
                builder.field("value", &wrapper)
            };
            let builder = {
                let wrapper = {
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.host_timestamp_ns)
                };
                builder.field("host_timestamp_ns", &wrapper)
            };
            let builder = {
                let wrapper = {
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.host_height)
                };
                builder.field("host_height", &wrapper)
            };
            builder.finish()
        }
    }
    impl ConsensusWithMetadata {
        fn new(state: impl Into<Any>, metadata: Metadata) -> Self {
            let Any { type_url, value } = state.into();
            let Metadata {
                host_timestamp_ns,
                host_height,
            } = metadata;
            Self {
                type_url,
                value,
                host_timestamp_ns,
                host_height,
            }
        }
        fn into_parts(self) -> (Any, Metadata) {
            (
                Any {
                    type_url: self.type_url,
                    value: self.value,
                },
                Metadata {
                    host_timestamp_ns: self.host_timestamp_ns,
                    host_height: self.host_height,
                },
            )
        }
    }
    unsafe fn wrap_ref<F: ?Sized, T: ?Sized>(from: &F) -> &T {
        if !(core::mem::size_of::<*const F>() == core::mem::size_of::<*const T>()) {
            :: core :: panicking :: panic ("assertion failed: core::mem::size_of::<*const F>() == core::mem::size_of::<*const T>()")
        };
        let inner_ptr = core::mem::ManuallyDrop::new(from as *const F);
        unsafe {
            let outer_ptr: *const T = core::mem::transmute_copy(&inner_ptr);
            &*outer_ptr
        }
    }
    unsafe fn wrap_mut<F: ?Sized, T: ?Sized>(from: &mut F) -> &mut T {
        if !(core::mem::size_of::<*mut F>() == core::mem::size_of::<*mut T>()) {
            :: core :: panicking :: panic ("assertion failed: core::mem::size_of::<*mut F>() == core::mem::size_of::<*mut T>()")
        };
        let inner_ptr = core::mem::ManuallyDrop::new(from as *mut F);
        unsafe {
            let outer_ptr: *mut T = core::mem::transmute_copy(&inner_ptr);
            &mut *outer_ptr
        }
    }
}
use crate::{crypto::PubKey, error::Error};
