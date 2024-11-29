// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use spacetimedb_sdk::__codegen::{
    self as __sdk, __lib, __sats, __ws,
    anyhow::{self as __anyhow, Context as _},
};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub struct InsertPkI256 {
    pub n: __sats::i256,
    pub data: i32,
}

impl __sdk::InModule for InsertPkI256 {
    type Module = super::RemoteModule;
}

pub struct InsertPkI256CallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `insert_pk_i256`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait insert_pk_i_256 {
    /// Request that the remote module invoke the reducer `insert_pk_i256` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_insert_pk_i_256`] callbacks.
    fn insert_pk_i_256(&self, n: __sats::i256, data: i32) -> __anyhow::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `insert_pk_i256`.
    ///
    /// The [`super::EventContext`] passed to the `callback`
    /// will always have [`__sdk::Event::Reducer`] as its `event`,
    /// but it may or may not have terminated successfully and been committed.
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::EventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`InsertPkI256CallbackId`] can be passed to [`Self::remove_on_insert_pk_i_256`]
    /// to cancel the callback.
    fn on_insert_pk_i_256(
        &self,
        callback: impl FnMut(&super::EventContext, &__sats::i256, &i32) + Send + 'static,
    ) -> InsertPkI256CallbackId;
    /// Cancel a callback previously registered by [`Self::on_insert_pk_i_256`],
    /// causing it not to run in the future.
    fn remove_on_insert_pk_i_256(&self, callback: InsertPkI256CallbackId);
}

impl insert_pk_i_256 for super::RemoteReducers {
    fn insert_pk_i_256(&self, n: __sats::i256, data: i32) -> __anyhow::Result<()> {
        self.imp.call_reducer("insert_pk_i256", InsertPkI256 { n, data })
    }
    fn on_insert_pk_i_256(
        &self,
        mut callback: impl FnMut(&super::EventContext, &__sats::i256, &i32) + Send + 'static,
    ) -> InsertPkI256CallbackId {
        InsertPkI256CallbackId(self.imp.on_reducer::<InsertPkI256>(
            "insert_pk_i256",
            Box::new(move |ctx: &super::EventContext, args: &InsertPkI256| callback(ctx, &args.n, &args.data)),
        ))
    }
    fn remove_on_insert_pk_i_256(&self, callback: InsertPkI256CallbackId) {
        self.imp.remove_on_reducer::<InsertPkI256>("insert_pk_i256", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `insert_pk_i256`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_insert_pk_i_256 {
    /// Set the call-reducer flags for the reducer `insert_pk_i256` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn insert_pk_i_256(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_insert_pk_i_256 for super::SetReducerFlags {
    fn insert_pk_i_256(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("insert_pk_i256", flags);
    }
}
