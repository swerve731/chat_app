use sqlx::PgPool;

pub mod messages;

const DATABASE_URL: &str = "postgres://chat_user:chat_password@localhost:5432/chat_db";

pub async fn get_connection_pool() -> Result<PgPool, sqlx::Error> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(DATABASE_URL)
        .await?;

    Ok(pool)
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_connection_pool() {
        let pool = get_connection_pool().await;
        assert!(pool.is_ok());
    }
}
