use chrono::NaiveDateTime;
use common::UserResponse;
use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct UserProfile {
    pub id: i32,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Default, PartialEq, Serialize, Deserialize, Store, Clone)]
#[store(storage = "local", storage_tab_sync)]
pub struct Store {
    pub profile: Option<UserProfile>,
    pub token: Option<String>,
}

pub fn set_token(token: Option<String>, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.token = token;
    });
}

pub fn set_profile(data: Option<UserResponse>, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.profile = match data {
            Some(data) => Some(UserProfile {
                id: data.id,
                email: data.email,
                created_at: data.created_at,
                updated_at: data.updated_at,
            }),
            None => None,
        }
    });
}
