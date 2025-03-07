use std::str::FromStr;

use api::{
    auth_service::{
        claims::*,
        user::{error::*, *},
    },
    db_service::get_connection_pool,
};

use chrono::Timelike;
use uuid::Uuid;

pub fn truncate_created_at(user: &mut User) {
    // This is for testing the time of creation of the user
    // use it when comparing results so that the precision matches
    user.created_at = user.created_at.with_nanosecond(0).unwrap();
}

#[test]
fn test_jwt() {
    let user_id = "1234";
    let user_claims = JwtClaims::new(user_id);

    let token = user_claims.encode().expect("error encoding jwt");

    let decoded_claims = JwtClaims::decode(&token).expect("error decoding jwt token to claims");

    assert_eq!(user_claims, decoded_claims);
    println!("{}", token)
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
            SignUpError::PasswordTooShort {
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
            SignUpError::PasswordTooWeak {
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
async fn test_signup_signin() {
    let pool = get_connection_pool().await.expect("error getting pg pool");

    // test sign up signin and check that the db matches
    // test creating a user with the same email
    // test signing in user
    // delete the test user

    // test sign up
    let email: String = format!("TestUser{}@email.com", Uuid::new_v4().to_string());
    let password = "Password123#";
    let token = User::signup(&pool, &email, password)
        .await
        .expect("error signing up user");

    let claims = JwtClaims::decode(&token).expect("error decoding jwt token");
    let mut user = User::get_user_by_id(
        &pool,
        Uuid::from_str(&claims.user_id).expect("error converting to uuid"),
    )
    .await
    .expect("error getting user by id");
    truncate_created_at(&mut user); // to set the precision so that the tests match in precision

    // check that the user was created
    let mut get_user_res = User::get_user_by_email(&pool, &email)
        .await
        .expect("Error getting user by email");

    truncate_created_at(&mut get_user_res);
    assert_eq!(user, get_user_res);

    // check that you cannot create a user with the same email
    let signup_same_email_res = User::signup(&pool, &email, password).await;
    assert!(signup_same_email_res.is_err());
    if let Err(err) = signup_same_email_res {
        match err {
            SignUpError::EmailTaken { requested_email } => {
                assert_eq!(requested_email, email);
            }
            err => panic!("unexpected error (should be email_taken): {:?}", err),
        }
    }

    //successful signin with correct credentials
    let signin_jwt = User::signin(&pool, &user.email, password)
        .await
        .expect("error signing in user");
    let claims = JwtClaims::decode(&signin_jwt).expect("Error decoding jwt to claims");

    assert_eq!(user.id.to_string(), claims.user_id);

    // test for wrong password
    let wrong_password_signin_res = User::signin(&pool, &user.email, "WrongPassword").await;
    assert!(wrong_password_signin_res.is_err());
    if let Err(err) = wrong_password_signin_res {
        match err {
            SignInError::WrongPassword => {}
            err => panic!("unexpected error (should be wrong_password): {:?}", err),
        }
    }

    // test for email that does not exist
    let not_found_test_email = format!("invalid_email_{}@email.com", Uuid::new_v4().to_string());
    let not_found_test_user = User::signin(&pool, &not_found_test_email, &user.password).await;
    assert!(not_found_test_user.is_err());
    if let Err(err) = not_found_test_user {
        match err {
            SignInError::EmailNotFound { requested_email } => {
                assert_eq!(requested_email, not_found_test_email);
            }
            err => panic!("unexpected error (should be email_not_found): {:?}", err),
        }
    }

    let mut delete_user_res = User::delete_user_by_id(&pool, user.id)
        .await
        .expect("Error deleting test user");
    truncate_created_at(&mut delete_user_res);
    assert_eq!(user, delete_user_res);
}
