use crate::database::DbError;
use diesel::{
    dsl::{exists, select},
    prelude::*,
};

use super::{dto::CreateUserDto, model::User};

pub fn get_all_users(conn: &mut SqliteConnection) -> Result<Vec<User>, DbError> {
    use crate::schema::users::dsl::*;
    let all_users = users.load::<User>(conn)?;
    Ok(all_users)
}

pub fn get_user(conn: &mut SqliteConnection, user_id: i32) -> Result<Option<User>, DbError> {
    use crate::schema::users::dsl::*;
    let user = users
        .filter(id.eq(user_id))
        .first::<User>(conn)
        .optional()?;
    Ok(user)
}

pub fn add_user(conn: &mut SqliteConnection, data: &CreateUserDto) -> Result<User, DbError> {
    use crate::schema::users::dsl::*;

    diesel::insert_into(users).values(data).execute(conn)?;
    let new_user = users.filter(email.eq(&data.email)).first::<User>(conn)?;

    Ok(new_user)
}

pub fn check_if_user_exists(conn: &mut SqliteConnection, user_id: i32) -> Result<bool, DbError> {
    use crate::schema::users::dsl::*;
    let user_exists = select(exists(users.find(user_id))).get_result::<bool>(conn)?;

    Ok(user_exists)
}
