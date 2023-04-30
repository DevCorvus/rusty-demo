use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};
use yewdux::prelude::use_store;

use crate::{
    api::api_get_user_profile,
    store::{set_profile, Store, UserProfile},
};

#[hook]
pub fn use_profile() -> SuspensionResult<UserProfile> {
    let (store, dispatch) = use_store::<Store>();

    if let Some(data) = store.profile.to_owned() {
        return Ok(data);
    }

    let (s, handle) = Suspension::new();

    if let Some(t) = store.token.to_owned() {
        spawn_local(async move {
            let res = api_get_user_profile(t.as_str()).await;

            if let Ok(data) = res {
                set_profile(Some(data), dispatch);
            }

            handle.resume();
        });

        if let Some(data) = store.profile.to_owned() {
            Ok(data)
        } else {
            Err(s)
        }
    } else {
        handle.resume();
        Err(s)
    }
}
