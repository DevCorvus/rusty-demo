use diesel::prelude::*;
use serde::Deserialize;

use crate::schema::users;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct CreateUserDto {
    pub email: String,
    pub password: String,
}
