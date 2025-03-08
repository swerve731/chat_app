use chrono::NaiveDateTime;
use uuid::Uuid;

// id UUID PRIMARY KEY,
// conversation_id UUID NOT NULL REFERENCES conversations (id) ON DELETE CASCADE,
// sender_id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
// reciever_id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
// sent_at TIMESTAMP NOT NULL

#[derive(Debug)]
pub struct Message {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub sender_id: Uuid,
    pub sent_at: NaiveDateTime,
    pub content: String,
}
