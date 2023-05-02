use anyhow;
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,
    pub exp: usize,
}

pub fn encode_jwt(user_id: i32) -> anyhow::Result<(String, DateTime<Utc>)> {
    let exp_date = Utc::now() + Duration::hours(24);

    let claims = JwtClaims {
        sub: user_id.to_string(),
        exp: exp_date.timestamp() as usize,
    };

    let header = Header::default(); // HS256
    let secret = env::var("JWT_SECRET")?;
    let encoding_key = EncodingKey::from_secret(secret.as_bytes());

    let jwt = encode(&header, &claims, &encoding_key)?;

    Ok((jwt, exp_date))
}

pub fn decode_jwt(token: String) -> anyhow::Result<TokenData<JwtClaims>> {
    let secret = env::var("JWT_SECRET")?;
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::default();

    let data = decode::<JwtClaims>(&token, &decoding_key, &validation)?;

    Ok(data)
}

#[cfg(test)]
mod tests {
    use dotenvy::dotenv;

    use super::*;

    #[test]
    fn test_encode_jwt() -> anyhow::Result<()> {
        dotenv()?;

        assert!(encode_jwt(1).is_ok());
        Ok(())
    }

    #[test]
    fn test_decode_jwt() -> anyhow::Result<()> {
        dotenv()?;

        let (token, _) = encode_jwt(1)?;
        let jwt = decode_jwt(token)?;
        assert_eq!(jwt.claims.sub, "1");
        Ok(())
    }
}
