// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use spacetimedb_sdk::__codegen::{
    self as __sdk, __lib, __sats, __ws,
    anyhow::{self as __anyhow, Context as _},
};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub struct InsertOptionVecOptionI32 {
    pub v: Option<Vec<Option<i32>>>,
}

impl __sdk::InModule for InsertOptionVecOptionI32 {
    type Module = super::RemoteModule;
}

pub struct InsertOptionVecOptionI32CallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `insert_option_vec_option_i32`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait insert_option_vec_option_i_32 {
    /// Request that the remote module invoke the reducer `insert_option_vec_option_i32` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_insert_option_vec_option_i_32`] callbacks.
    fn insert_option_vec_option_i_32(&self, v: Option<Vec<Option<i32>>>) -> __anyhow::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `insert_option_vec_option_i32`.
    ///
    /// The [`super::EventContext`] passed to the `callback`
    /// will always have [`__sdk::Event::Reducer`] as its `event`,
    /// but it may or may not have terminated successfully and been committed.
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::EventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`InsertOptionVecOptionI32CallbackId`] can be passed to [`Self::remove_on_insert_option_vec_option_i_32`]
    /// to cancel the callback.
    fn on_insert_option_vec_option_i_32(
        &self,
        callback: impl FnMut(&super::EventContext, &Option<Vec<Option<i32>>>) + Send + 'static,
    ) -> InsertOptionVecOptionI32CallbackId;
    /// Cancel a callback previously registered by [`Self::on_insert_option_vec_option_i_32`],
    /// causing it not to run in the future.
    fn remove_on_insert_option_vec_option_i_32(&self, callback: InsertOptionVecOptionI32CallbackId);
}

impl insert_option_vec_option_i_32 for super::RemoteReducers {
    fn insert_option_vec_option_i_32(&self, v: Option<Vec<Option<i32>>>) -> __anyhow::Result<()> {
        self.imp
            .call_reducer("insert_option_vec_option_i32", InsertOptionVecOptionI32 { v })
    }
    fn on_insert_option_vec_option_i_32(
        &self,
        mut callback: impl FnMut(&super::EventContext, &Option<Vec<Option<i32>>>) + Send + 'static,
    ) -> InsertOptionVecOptionI32CallbackId {
        InsertOptionVecOptionI32CallbackId(self.imp.on_reducer::<InsertOptionVecOptionI32>(
            "insert_option_vec_option_i32",
            Box::new(move |ctx: &super::EventContext, args: &InsertOptionVecOptionI32| callback(ctx, &args.v)),
        ))
    }
    fn remove_on_insert_option_vec_option_i_32(&self, callback: InsertOptionVecOptionI32CallbackId) {
        self.imp
            .remove_on_reducer::<InsertOptionVecOptionI32>("insert_option_vec_option_i32", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `insert_option_vec_option_i32`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_insert_option_vec_option_i_32 {
    /// Set the call-reducer flags for the reducer `insert_option_vec_option_i32` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn insert_option_vec_option_i_32(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_insert_option_vec_option_i_32 for super::SetReducerFlags {
    fn insert_option_vec_option_i_32(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("insert_option_vec_option_i32", flags);
    }
}
