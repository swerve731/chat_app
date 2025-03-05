use chrono::NaiveDateTime;
use derive_more::From;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, PgPool};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustomClaims {
    user_id: String,
}

impl CustomClaims {

    const TOKEN_LIFETIME_IN_DAYS: i64 = 1;

    pub fn new(user_id: String) -> Self {
        Self { user_id }
    }

    pub fn jwt(&self) -> Result<String, ClaimsError> {
        let claims = Claims::with_custom_claims(self, Duration::from_days(Self::TOKEN_LIFETIME_IN_DAYS))?;
        // test
        
        
    }

}

#[derive(Debug, PartialEq, Eq)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, From)]
pub enum ClaimsError {
    #[from]
    Jwt(simple_jwt::Error),
}

#[derive(Debug, From)]
pub enum AuthError {
    #[from]
    Signup(SignupError),

    #[from]
    Claims(ClaimsError),
}

#[derive(Debug, From)]
pub enum SignupError {
    UsernameTaken { requested_username: String },
    PasswordTooShort { min_length: usize, actual_length: usize },
    PasswordTooWeak {
        has_lowercase: bool,
        has_uppercase: bool,
        has_number: bool,
        has_special: bool,
    },
    #[from]
    Database(sqlx::Error),
}

impl User {
    const MIN_PASSWORD_LENGTH: usize = 6;




    pub async fn signup(pool: &PgPool, username: &str, password: &str) -> Result<User, SignupError> {

        // TODO ADD ARGON2 HASHING
        let id = Uuid::new_v4();
        let created_at = sqlx::types::chrono::Utc::now().naive_utc();

        Self::validate_username(&pool, username).await?;
        Self::validate_password_strength(password)?;

        query!(
            "INSERT INTO users (id, username, password, created_at) VALUES ($1, $2, $3, $4)",
            id, username, password, created_at
        )
        .execute(pool)
        .await?;

        Ok(User {
            id,
            username: username.to_string(),
            password: password.to_string(),
            created_at,
        })
    }

    // helper functions
    pub fn validate_password_strength(password: &str) -> Result<(), SignupError> {
        let length = password.len();

        let has_lowercase = password.chars().any(|c| c.is_lowercase());
        let has_uppercase = password.chars().any(|c| c.is_uppercase());
        let has_number = password.chars().any(|c| c.is_numeric());
        let has_special = password.chars().any(|c| !c.is_alphanumeric());

        return if length < Self::MIN_PASSWORD_LENGTH {
            Err(SignupError::PasswordTooShort {
                min_length: Self::MIN_PASSWORD_LENGTH,
                actual_length: length,
            })
        } else if !has_lowercase || !has_uppercase || !has_number || !has_special {
            Err(SignupError::PasswordTooWeak {
                has_lowercase,
                has_uppercase,
                has_number,
                has_special,
            })
        } else {
            Ok(())
        };
    }

    pub async fn validate_username(pool: &PgPool, username: &str) -> Result<(), SignupError> {
        let res = query!(
            "SELECT * FROM users WHERE username = $1",
            username
        )
        .fetch_one(pool)
        .await;

        return match res {
            Ok(_) => Err(SignupError::UsernameTaken {
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

    pub async fn delete_user_by_id(pool: &PgPool, id: Uuid) -> Result<User, sqlx::Error> {
        query_as!(
            User,
            "DELETE FROM users WHERE id = $1 RETURNING *", 
            id)
            .fetch_one(pool)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Timelike;

    impl User {
        pub fn truncate_created_at(&mut self){
            // This is for testing the time of creation of the user
            // use it when comparing results so that the precision matches
            self.created_at = self.created_at.with_nanosecond(0).unwrap();
        }
    }


    #[test]
    fn test_validate_password_strength() {
        let password = "PerfectPassword123!";

        let result = User::validate_password_strength(password);

        assert!(result.is_ok());

        let password = "short";
        let result = User::validate_password_strength(password);

        assert!(result.is_err());

        if let Err(err) = result {
            match err {
                SignupError::PasswordTooShort {
                    min_length,
                    actual_length,
                } => {
                    assert_eq!(min_length, User::MIN_PASSWORD_LENGTH);
                    assert_eq!(actual_length, password.len());
                }
                _ => panic!("unexpected error"),
            }
        }

        let password = "no-upperc@s3";
        let result = User::validate_password_strength(password);
        if let Err(err) = result {
            match err {
                SignupError::PasswordTooWeak {
                    has_lowercase,
                    has_uppercase,
                    has_number,
                    has_special,
                } => {
                    assert!(has_lowercase);
                    assert!(!has_uppercase);
                    assert!(has_number);
                    assert!(has_special);
                }
                _ => panic!("unexpected error"),
            }
        }
    }

    #[tokio::test]
    async fn test_signup() {
        let pool = crate::db_service::get_connection_pool().await.expect("error getting pg pool");


        // test sign up signin and check that the db matches 
        // test creating a user with the same username 
        // test signing in user
        // delete the test user
    
        // test sign up
        let username: String = format!("TestUser{}", Uuid::new_v4().to_string());
        let password = "Password123#";
        let mut user = User::signup(&pool, &username, password).await.expect("error signing up user");
        user.truncate_created_at(); // to set the precision so that the tests match in precision

        // check that the user was created
        let mut get_user_res = User::get_user_by_username(&pool, &username).await.expect("Error getting user by username");
        get_user_res.truncate_created_at();
        assert_eq!(user, get_user_res);

        // check that you cannot create a user with the same username
        let signup_same_username_res = User::signup(&pool, &username, password).await;
        assert!(signup_same_username_res.is_err());
        if let Err(err) = signup_same_username_res {
            match err {
                SignupError::UsernameTaken { requested_username } => {
                    assert_eq!(requested_username, username);
                }
                err => panic!("unexpected error (should be username_taken): {:?}", err),
            }
        }


        // todo!("test signing in user");

        let mut delete_user_res = User::delete_user_by_id(&pool, user.id).await.expect("Error deleting test user");
        delete_user_res.truncate_created_at();
        assert_eq!(user, delete_user_res);



    }
}