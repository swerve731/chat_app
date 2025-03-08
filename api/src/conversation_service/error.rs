use derive_more::From;

#[derive(Debug, From)]
pub enum ConversationError {
    ConversationDoesNotExist,
    SameSenderAndReceiver,
    ConversationAlreadyExists {
        conversation_id: uuid::Uuid,
    },

    #[from]
    Database(sqlx::Error),
}
