use std::future::{ready, Ready};

use actix_web::{dev::Payload, http::header, Error as ActixWebError, FromRequest, HttpRequest};

use super::jwt::decode_jwt;
use crate::error::AppError;

pub struct JwtMiddleware {
    pub user_id: i32,
}

impl FromRequest for JwtMiddleware {
    type Future = Ready<Result<Self, Self::Error>>;
    type Error = ActixWebError;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let token = get_bearer_token(req);

        if token.is_none() {
            return ready(Err(AppError::Unauthorized.into()));
        }

        let claims = match decode_jwt(String::from(token.unwrap())) {
            Ok(jwt) => jwt.claims,
            Err(_) => {
                return ready(Err(AppError::Unauthorized.into()));
            }
        };

        let user_id = match claims.sub.parse::<i32>() {
            Ok(id) => id,
            Err(_) => {
                return ready(Err(AppError::InternalError.into()));
            }
        };

        ready(Ok(JwtMiddleware { user_id }))
    }
}

fn get_bearer_token(req: &HttpRequest) -> Option<&str> {
    req.headers()
        .get(header::AUTHORIZATION)?
        .to_str()
        .unwrap_or_default()
        .split_whitespace()
        .last()
}
