use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
pub mod error;
use error::ClaimsError;
use uuid::Uuid;
pub type JwtTokenString = String;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct JwtClaims {
    pub user_id: uuid::Uuid,
    exp: usize,
}

impl JwtClaims {
    const TOKEN_LIFETIME_IN_DAYS: i64 = 1;

    // todo add this to .env
    const SECRET_KEY: &str = "Super-Secret-Key";

    pub fn new(user_id: Uuid) -> Self {
        let exp = (Utc::now() + Duration::days(Self::TOKEN_LIFETIME_IN_DAYS)).timestamp() as usize;

        Self {
            user_id: user_id,
            exp,
        }
    }

    pub fn encode(&self) -> Result<JwtTokenString, ClaimsError> {
        let token = encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(Self::SECRET_KEY.as_ref()),
        )?;

        Ok(token.to_string())
    }

    pub fn decode(encoded_token: &JwtTokenString) -> Result<Self, ClaimsError> {
        // `token` is a struct with 2 fields: `header` and `claims` where `claims` is your own struct.
        let token = decode::<JwtClaims>(
            encoded_token,
            &DecodingKey::from_secret(Self::SECRET_KEY.as_ref()),
            &Validation::default(),
        )?;

        // jsonweb token will return ErrorKind Expired Signature if the request token is expired
        let claims = token.claims;
        Ok(claims)
    }
}
