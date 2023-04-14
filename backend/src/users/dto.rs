use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct CreateUserDto {
    pub email: String,
    pub password: String,
}
