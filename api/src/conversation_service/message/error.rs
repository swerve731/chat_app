#[derive(Debug, derive_more::From)]
pub enum MessageError {
    #[from]
    Database(sqlx::Error),
    ReceiverDoesNotExist,
}
