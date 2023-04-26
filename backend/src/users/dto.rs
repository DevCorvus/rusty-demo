use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct CreateUserDto {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateUserPasswordDto {
    pub password: String,
}

#[derive(Serialize)]
pub struct FilteredUserDto {
    pub id: i32,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
