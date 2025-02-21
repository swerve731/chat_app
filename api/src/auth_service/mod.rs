use axum::Json;
use derive_more::From;
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::{Record, Root},
    Surreal,
};

#[derive(Debug, From)]
pub enum Error {
    Signup(SignupError),
    Signin(SigninError),
    Initialize,

    #[from]
    DB(surrealdb::Error),
}

#[derive(Debug)]
pub enum SignupError {
    EmailExists,
    PasswordNotSafe,
}

#[derive(Debug)]
pub enum SigninError {
    EmailNotFound,
    PasswordIncorrect,
}

pub async fn initialize() -> crate::Result<surrealdb::Surreal<Client>> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    db.use_ns("test").use_db("test").await?;
    // todo!()

    db.query(
        "
    DEFINE TABLE IF NOT EXISTS user SCHEMALESS
        PERMISSIONS FOR
            CREATE, SELECT WHERE $auth,
            FOR UPDATE, DELETE WHERE created_by = $auth;
    DEFINE FIELD IF NOT EXISTS email ON TABLE user TYPE string;
    DEFINE FIELD IF NOT EXISTS created_by ON TABLE user VALUE $auth READONLY;

    DEFINE INDEX IF NOT EXISTS unique_name ON TABLE user FIELDS name UNIQUE;
    DEFINE ACCESS IF NOT EXISTS account ON DATABASE TYPE RECORD
    SIGNUP ( CREATE user SET email = $email, pass = crypto::argon2::generate($pass) )
    SIGNIN ( SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(pass, $pass) )
    DURATION FOR TOKEN 15m, FOR SESSION 12h
    ;",
    )
    .await?;

    Ok(db)
}

#[derive(Serialize, Deserialize)]
struct User<'a> {
    id: &'a str,
    email: &'a str,
    pass: &'a str,
}

pub async fn get_session(db: &surrealdb::Surreal<Client>, jwt: String) -> Result<String, Error> {
    db.authenticate(jwt).await?;

    let res: Option<String> = db.query("RETURN <string>$session").await?.take(0)?;
    Ok(res.unwrap_or("No session data found!".to_string()).into())
}

pub async fn signup(
    db: &surrealdb::Surreal<Client>,
    email: &str,
    password: &str,
) -> Result<String, Error> {
    let id = &uuid::Uuid::new_v4().to_string();
    let jwt = db
        .signup(Record {
            access: "account",
            namespace: "test",
            database: "test",
            params: User {
                email: &email,
                pass: &password,
                id,
            },
        })
        .await?
        .into_insecure_token();

    println!("id:::: {}", id);

    Ok(jwt)
}

pub async fn signin(email: String, password: String) -> crate::Result<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn initialize_signup_signin() {
        let db = initialize().await.expect("Error initializing DB");
        let jwt = signup(&db, "testuser2@gmail.com", "234Ee!")
            .await
            .expect("error signing in");
        let session = get_session(&db, jwt).await.expect("err getting session");

        println!("session: {:?}", session);
    }
}
