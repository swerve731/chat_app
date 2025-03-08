use crate::auth_service::claims::JwtClaims;
use crate::auth_service::user::*;
use crate::db_service;
use api::*;
use conversation_service::conversation::Conversation;
use uuid::Uuid;
#[tokio::test]
async fn conversation_and_messaging() {
    let pool = db_service::get_connection_pool()
        .await
        .expect("error gettign connection pool");

    let jwt = &User::signup(
        &pool,
        &format!("TestUser01{}@gmail.com", Uuid::new_v4().to_string()),
        "123456Ee!",
    )
    .await
    .expect("error creating test user");
    let test_user_one_id = JwtClaims::decode(jwt)
        .expect("error getting claims")
        .user_id;

    let test_user_two_id = JwtClaims::decode(
        &User::signup(
            &pool,
            &format!("TestUser01{}@gmail.com", Uuid::new_v4().to_string()),
            "123456Ee!",
        )
        .await
        .expect("error creating test user"),
    )
    .expect("error getting claims")
    .user_id;

    let conversation_id = Conversation::start(&pool, test_user_one_id, test_user_two_id)
        .await
        .expect("Error starting conversation");

    let message_one_content = "this is a test message";
    let _send_message = Conversation::send_message(
        &pool,
        test_user_one_id,
        conversation_id,
        message_one_content,
    )
    .await
    .expect("error sending message");

    let messages = Conversation::get_all_messages(&pool, conversation_id)
        .await
        .expect("error getting messages");

    assert_eq!(messages.len(), 1)
}
