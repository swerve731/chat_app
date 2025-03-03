use derive_more::From;
use uuid::Uuid;




pub struct User<'a> {
    pub id: Uuid,
    pub username: &'a str,
    pub password: &'a str,
    pub created_at: chrono::DateTime<chrono::Utc>,
}   

#[derive(Debug, From)]
pub enum AuthError {
    
    // -- Externals
    #[from]
    Signup(SignupError),
}


#[derive(Debug, From)]
pub enum SignupError {
    UsernameTaken{requested_username: String},

    PasswordTooShort{min_length: usize, actual_length: usize},
    PasswordTooWeak{ 
        has_lowercase: bool,
        has_uppercase: bool, 
        has_number: bool, 
        has_special: bool,
    },

    #[from]
    Database(sqlx::Error),    
}


impl User<'_> {



    const MIN_PASSWORD_LENGTH: usize = 6;

    pub fn validate_password_strength(password: &str) -> Result<(), SignupError> {
        let length = password.len();

        // a function to validate the password strength 
        // each password must contain 1 upper, 1 lower, 1 special, 1 number
        // the password must also be longer than 6 characters
        //if the password does not have the required parameters return Err(SignupError::PasswordTooWeak) and indicate what is needed

        todo!("set these values accordingly"); 
        let has_lowercase = false;
        let has_uppercase = false;
        let has_number = false;
        let has_special = false;


        return if length < Self::MIN_PASSWORD_LENGTH {

            Err(
                SignupError::PasswordTooShort { 
                    min_length: Self::MIN_PASSWORD_LENGTH, 
                    actual_length: length 
                }
            )

        } else if !has_lowercase || !has_uppercase || !has_number || !has_special{
            Err(
                SignupError::PasswordTooWeak { has_lowercase, has_uppercase, has_number, has_special }
            )
        } else {
            Ok(())
        }



    }

    pub fn validate_username(username: &str) -> Result<(), SignupError> {
        // a function to validate the username
        // the username must be unique in the database
        // if the username is not unique return Err(SignupError::UsernameTaken) and indicate the username that was requested
        // if the username is unique return Ok(())

        todo!("implement this function"); 
        Ok(())
    }


    pub async fn signup<'a>(username: &'a str, password: &'a str) -> Result<User<'a>, SignupError> {
        let id = Uuid::new_v4();
        let created_at = chrono::Utc::now();

        Ok(User { 
            id, 
            username,
            password, 
            created_at 
        })
    }
}


// tests
#[cfg(test)]
mod tests {
    use super::*;

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
                SignupError::PasswordTooShort { min_length, actual_length } => {
                    assert_eq!(min_length, User::MIN_PASSWORD_LENGTH);
                    assert_eq!(actual_length, password.len());
                },
                _ => panic!("unexpected error")
            }
        }

        let password = "no-upperc@s3";
        let result = User::validate_password_strength(password);
        if let Err(err) = result {
            match err {
                SignupError::PasswordTooWeak { has_lowercase, has_uppercase, has_number, has_special } => {
                    assert!(has_lowercase);
                    assert!(!has_uppercase);
                    assert!(has_number);
                    assert!(has_special);
                },
                _ => panic!("unexpected error")
            }
        }

    }

    // #[test]
    // fn test_signup() {
    //     let username = "username";
    //     let password = "password";
    //     let user = User::signup(username, password);
    //     assert!(user.is_ok());
    // }
}
