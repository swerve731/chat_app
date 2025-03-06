pub mod claims;
pub mod error;
pub mod user;

// use chrono::{Duration, NaiveDateTime, Utc};
// use derive_more::From;
// use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
// use serde::{Deserialize, Serialize};
// use sqlx::{query, query_as, PgPool};
// use uuid::Uuid;

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use chrono::Timelike;

//     impl User {
//         pub fn truncate_created_at(&mut self) {
//             // This is for testing the time of creation of the user
//             // use it when comparing results so that the precision matches
//             self.created_at = self.created_at.with_nanosecond(0).unwrap();
//         }
//     }

//     #[test]
//     fn test_jwt() {
//         let user_id = "1234";
//         let user_claims = JwtClaims::new(user_id);

//         let token = user_claims.encode().expect("error encoding jwt");

//         let decoded_claims = JwtClaims::decode(&token).expect("error decoding jwt token to claims");

//         assert_eq!(user_claims, decoded_claims);
//         println!("{}", token)
//     }

//     #[test]
//     fn test_validate_password_strength() {
//         let password = "PerfectPassword123!";

//         let result = User::validate_password_strength(password);

//         assert!(result.is_ok());

//         let password = "short";
//         let result = User::validate_password_strength(password);

//         assert!(result.is_err());

//         if let Err(err) = result {
//             match err {
//                 SignupError::PasswordTooShort {
//                     min_length,
//                     actual_length,
//                 } => {
//                     assert_eq!(min_length, User::MIN_PASSWORD_LENGTH);
//                     assert_eq!(actual_length, password.len());
//                 }
//                 _ => panic!("unexpected error"),
//             }
//         }

//         let password = "no-upperc@s3";
//         let result = User::validate_password_strength(password);
//         if let Err(err) = result {
//             match err {
//                 SignupError::PasswordTooWeak {
//                     has_lowercase,
//                     has_uppercase,
//                     has_number,
//                     has_special,
//                 } => {
//                     assert!(has_lowercase);
//                     assert!(!has_uppercase);
//                     assert!(has_number);
//                     assert!(has_special);
//                 }
//                 _ => panic!("unexpected error"),
//             }
//         }
//     }

//     #[tokio::test]
//     async fn test_signup_signin() {
//         let pool = crate::db_service::get_connection_pool()
//             .await
//             .expect("error getting pg pool");

//         // test sign up signin and check that the db matches
//         // test creating a user with the same username
//         // test signing in user
//         // delete the test user

//         // test sign up
//         let username: String = format!("TestUser{}", Uuid::new_v4().to_string());
//         let password = "Password123#";
//         let mut user = User::signup(&pool, &username, password)
//             .await
//             .expect("error signing up user");
//         user.truncate_created_at(); // to set the precision so that the tests match in precision

//         // check that the user was created
//         let mut get_user_res = User::get_user_by_username(&pool, &username)
//             .await
//             .expect("Error getting user by username");
//         get_user_res.truncate_created_at();
//         assert_eq!(user, get_user_res);

//         // check that you cannot create a user with the same username
//         let signup_same_username_res = User::signup(&pool, &username, password).await;
//         assert!(signup_same_username_res.is_err());
//         if let Err(err) = signup_same_username_res {
//             match err {
//                 SignupError::UsernameTaken { requested_username } => {
//                     assert_eq!(requested_username, username);
//                 }
//                 err => panic!("unexpected error (should be username_taken): {:?}", err),
//             }
//         }

//         //successful signin with correct credentials
//         let signin_jwt = User::signin(&pool, &user.username, &user.password)
//             .await
//             .expect("error signing in user");
//         let claims = JwtClaims::decode(&signin_jwt).expect("Error decoding jwt to claims");

//         assert_eq!(user.id.to_string(), claims.user_id);

//         // test for wrong password
//         let wrong_password_signin_res = User::signin(&pool, &user.username, "WrongPassword").await;
//         assert!(wrong_password_signin_res.is_err());
//         if let Err(err) = wrong_password_signin_res {
//             match err {
//                 LoginError::WrongPassword => {}
//                 err => panic!("unexpected error (should be wrong_password): {:?}", err),
//             }
//         }

//         // test for username that does not exist
//         let invalid_test_username = format!("invalid_username_{}", Uuid::new_v4().to_string());
//         let invalid_username_signin_res =
//             User::signin(&pool, &invalid_test_username, &user.password).await;
//         assert!(invalid_username_signin_res.is_err());
//         if let Err(err) = invalid_username_signin_res {
//             match err {
//                 LoginError::UsernameNotFound { requested_username } => {
//                     assert_eq!(requested_username, invalid_test_username);
//                 }
//                 err => panic!("unexpected error (should be username_not_found): {:?}", err),
//             }
//         }

//         let mut delete_user_res = User::delete_user_by_id(&pool, user.id)
//             .await
//             .expect("Error deleting test user");
//         delete_user_res.truncate_created_at();
//         assert_eq!(user, delete_user_res);
//     }
// }
