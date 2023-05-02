use diesel::prelude::*;
use serde::Deserialize;
use validator::Validate;

use crate::schema::users;

#[derive(Deserialize, Insertable, Validate, Clone)]
#[diesel(table_name = users)]
pub struct CreateUserDto {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
}
