use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Default, PartialEq, Serialize, Deserialize, Store, Clone)]
#[store(storage = "local", storage_tab_sync)]
pub struct Store {
    pub token: Option<String>,
}

pub fn set_token(token: Option<String>, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.token = token;
    });
}
