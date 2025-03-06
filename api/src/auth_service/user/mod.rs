use super::claims::*;
pub mod error;
use chrono::NaiveDateTime;
use error::{SignInError, SignUpError};
use sqlx::{query, query_as, PgPool};
use uuid::Uuid;
pub mod router;

#[derive(Debug, PartialEq, Eq)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

impl User {
    pub const MIN_PASSWORD_LENGTH: usize = 6;

    pub async fn signin(
        pool: &PgPool,
        username: &str,
        password: &str,
    ) -> Result<JwtTokenString, SignInError> {
        return match Self::get_user_by_username(pool, username).await {
            Ok(user) => {
                if user.password != password {
                    return Err(SignInError::WrongPassword);
                }

                let claims = JwtClaims::new(&user.id.to_string());
                let token = claims.encode()?;

                Ok(token)
            }
            Err(err) => match err {
                sqlx::Error::RowNotFound => Err(SignInError::UsernameNotFound {
                    requested_username: username.to_string(),
                }),
                _ => Err(SignInError::Database(err)),
            },
        };
    }

    pub async fn signup(
        pool: &PgPool,
        username: &str,
        password: &str,
    ) -> Result<JwtTokenString, SignUpError> {
        // TODO ADD ARGON2 HASHING
        let id = Uuid::new_v4();
        let created_at = sqlx::types::chrono::Utc::now().naive_utc();

        Self::validate_username(&pool, username).await?;
        Self::validate_password_strength(password)?;

        query!(
            "INSERT INTO users (id, username, password, created_at) VALUES ($1, $2, $3, $4)",
            id,
            username,
            password,
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

    pub async fn validate_username(pool: &PgPool, username: &str) -> Result<(), SignUpError> {
        let res = query!("SELECT * FROM users WHERE username = $1", username)
            .fetch_one(pool)
            .await;

        return match res {
            Ok(_) => Err(SignUpError::UsernameTaken {
                requested_username: username.to_string(),
            }),
            Err(sqlx::Error::RowNotFound) => Ok(()),
            Err(err) => Err(err.into()),
        };
    }

    // crud helper functions
    pub async fn get_user_by_username(pool: &PgPool, username: &str) -> Result<User, sqlx::Error> {
        query_as!(
            User,
            "SELECT id, username, password, created_at FROM users WHERE username = $1",
            username
        )
        .fetch_one(pool)
        .await
    }

    pub async fn get_user_by_id(pool: &PgPool, id: Uuid) -> Result<User, sqlx::Error> {
        query_as!(
            User,
            "SELECT id, username, password, created_at FROM users WHERE id = $1",
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
