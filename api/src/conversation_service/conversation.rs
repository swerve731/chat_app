use super::error::ConversationError;
use super::message::Message;
use axum::response::Result;
use chrono::NaiveDateTime;
use sqlx::PgPool;
use uuid::Uuid;

pub struct Conversation {
    id: Uuid,
    sender_id: Uuid,
    receiver_id: Uuid,
    started_at: sqlx::types::chrono::NaiveDateTime,
}

impl Conversation {
    // send message -> message_id
    // get all messages -> Vec<Message>
    // get messages after DateTime -> Vec<Message>
    // start (creates a new conversation between users) -> conversation_id
    // pair exists (to check if there is a conversation already exists between two users before starting a new one)-> Option<conversation_id>

    pub async fn get_conversations_with_user_id(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<Conversation>, ConversationError> {
        let conversations = sqlx::query_as!(
            Conversation,
            r#"
            SELECT *
            FROM conversations
            WHERE sender_id = $1 OR receiver_id = $1
            "#,
            user_id,
        )
        .fetch_all(pool)
        .await?;

        Ok(conversations)
    }

    pub async fn send_message(
        pool: &PgPool,
        sender_id: Uuid,
        conversation_id: Uuid,
        message_content: &str,
    ) -> Result<Uuid, ConversationError> {
        let message_id = Uuid::new_v4();
        sqlx::query!(
            r#"
            INSERT INTO messages (id, conversation_id, content, sent_at, sender_id)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            message_id,
            conversation_id,
            message_content,
            sqlx::types::chrono::Utc::now().naive_utc(),
            sender_id,
        )
        .execute(pool)
        .await?;

        Ok(message_id)
    }

    pub async fn get_all_messages(
        pool: &PgPool,
        conversation_id: Uuid,
    ) -> Result<Vec<Message>, ConversationError> {
        let messages = sqlx::query_as!(
            Message,
            r#"
            SELECT id, conversation_id, content, sent_at, sender_id
            FROM messages
            WHERE conversation_id = $1
            ORDER BY sent_at ASC
            "#,
            conversation_id,
        )
        .fetch_all(pool)
        .await?;

        Ok(messages)
    }

    pub async fn get_messages_after_time(
        pool: &PgPool,
        conversation_id: Uuid,
        time: NaiveDateTime,
    ) -> Result<Vec<Message>, ConversationError> {
        let messages = sqlx::query_as!(
            Message,
            r#"
            SELECT id, conversation_id, content, sent_at, sender_id
            FROM messages
            WHERE conversation_id = $1 AND sent_at > $2
            ORDER BY sent_at ASC
            "#,
            conversation_id,
            time,
        )
        .fetch_all(pool)
        .await?;

        Ok(messages)
    }
    pub async fn start(
        pool: &PgPool,
        sender_id: Uuid,
        receiver_id: Uuid,
    ) -> Result<Uuid, ConversationError> {
        // check that the sender and receiver id are not the same
        if receiver_id == sender_id {
            return Err(ConversationError::SameSenderAndReceiver);
        }

        // check if there is already a conversation with the reciever and sender
        // if there is return that conversation id
        if let Some(id) = Conversation::pair_exists(pool, sender_id, receiver_id).await? {
            return Err(ConversationError::ConversationAlreadyExists {
                conversation_id: id,
            });
        }

        let id = uuid::Uuid::new_v4();
        let conversation = Conversation {
            id,
            sender_id,
            receiver_id,
            started_at: sqlx::types::chrono::Utc::now().naive_utc(),
        };

        sqlx::query!(
            r#"
            INSERT INTO conversations (id, sender_id, receiver_id, started_at)
            VALUES ($1, $2, $3, $4)
            "#,
            conversation.id,
            conversation.sender_id,
            conversation.receiver_id,
            conversation.started_at
        )
        .execute(pool)
        .await?;

        Ok(id)
    }

    pub async fn pair_exists(
        pool: &PgPool,
        sender_id: Uuid,
        receiver_id: Uuid,
    ) -> Result<Option<Uuid>, ConversationError> {
        let rec = sqlx::query!(
            r#"
            SELECT id
            FROM conversations
            WHERE sender_id = $1 AND receiver_id = $2
            "#,
            sender_id,
            receiver_id,
        )
        .fetch_optional(pool)
        .await?;

        if let Some(record) = rec {
            return Ok(Some(record.id));
        }

        let rec = sqlx::query!(
            r#"
            SELECT id
            FROM conversations
            WHERE sender_id = $2 AND receiver_id = $1
            "#,
            sender_id,
            receiver_id,
        )
        .fetch_optional(pool)
        .await?;

        if let Some(record) = rec {
            return Ok(Some(record.id));
        }

        Ok(None)
    }
}
