use super::claims::*;
pub mod error;
use chrono::NaiveDateTime;
use error::{SignInError, SignUpError};
use sqlx::{query, query_as, PgPool};
use uuid::Uuid;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

#[derive(Debug, PartialEq, Eq)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

impl User {
    pub const MIN_PASSWORD_LENGTH: usize = 6;

    pub async fn signin(
        pool: &PgPool,
        email: &str,
        password: &str,
    ) -> Result<JwtTokenString, SignInError> {
        return match Self::get_user_by_email(pool, email).await {
            Ok(user) => {
                let parsed_hash = PasswordHash::new(&user.password)?;

                let verification_res =
                    Argon2::default().verify_password(password.as_bytes(), &parsed_hash);

                if Err(argon2::password_hash::Error::Password) == verification_res {
                    return Err(SignInError::WrongPassword);
                } else if let Err(e) = verification_res {
                    tracing::error!("Unexpected Error {:?} in password verification", e);
                    return Err(SignInError::PasswordHashing(e));
                } else {
                    let claims = JwtClaims::new(&user.id.to_string());
                    let token = claims.encode()?;

                    return Ok(token);
                }
            }
            Err(err) => match err {
                sqlx::Error::RowNotFound => Err(SignInError::EmailNotFound {
                    requested_email: email.to_string(),
                }),
                _ => Err(SignInError::Database(err)),
            },
        };
    }

    pub async fn signup(
        pool: &PgPool,
        email: &str,
        password: &str,
    ) -> Result<JwtTokenString, SignUpError> {
        let id = Uuid::new_v4();
        let created_at = sqlx::types::chrono::Utc::now().naive_utc();

        Self::validate_email(&pool, email).await?;
        Self::validate_password_strength(password)?;

        // encrypt password
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        query!(
            "INSERT INTO users (id, email, password, created_at) VALUES ($1, $2, $3, $4)",
            id,
            email,
            password_hash,
            created_at
        )
        .execute(pool)
        .await?;

        let jwt_token = JwtClaims::new(&id.to_string()).encode()?;

        Ok(jwt_token)
    }

    // helper functions
    pub fn validate_password_strength(password: &str) -> Result<(), SignUpError> {
        let length = password.len();

        let has_lowercase = password.chars().any(|c| c.is_lowercase());
        let has_uppercase = password.chars().any(|c| c.is_uppercase());
        let has_number = password.chars().any(|c| c.is_numeric());
        let has_special = password.chars().any(|c| !c.is_alphanumeric());

        return if length < Self::MIN_PASSWORD_LENGTH {
            Err(SignUpError::PasswordTooShort {
                min_length: Self::MIN_PASSWORD_LENGTH,
                actual_length: length,
            })
        } else if !has_lowercase || !has_uppercase || !has_number || !has_special {
            Err(SignUpError::PasswordTooWeak {
                has_lowercase,
                has_uppercase,
                has_number,
                has_special,
            })
        } else {
            Ok(())
        };
    }

    pub async fn validate_email(pool: &PgPool, email: &str) -> Result<(), SignUpError> {
        if !email_address::EmailAddress::is_valid(email) {
            return Err(SignUpError::InvalidEmail {
                requested_email: email.to_string(),
            });
        }

        let res = query!("SELECT * FROM users WHERE email = $1", email)
            .fetch_one(pool)
            .await;

        return match res {
            Ok(_) => Err(SignUpError::EmailTaken {
                requested_email: email.to_string(),
            }),
            Err(sqlx::Error::RowNotFound) => Ok(()),
            Err(err) => Err(err.into()),
        };
    }

    // crud helper functions
    pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<User, sqlx::Error> {
        query_as!(
            User,
            "SELECT id, email, password, created_at FROM users WHERE email = $1",
            email
        )
        .fetch_one(pool)
        .await
    }

    pub async fn get_user_by_id(pool: &PgPool, id: Uuid) -> Result<User, sqlx::Error> {
        query_as!(
            User,
            "SELECT id, email, password, created_at FROM users WHERE id = $1",
            id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn delete_user_by_id(pool: &PgPool, id: Uuid) -> Result<User, sqlx::Error> {
        query_as!(User, "DELETE FROM users WHERE id = $1 RETURNING *", id)
            .fetch_one(pool)
            .await
    }
}
