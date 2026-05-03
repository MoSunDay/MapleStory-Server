pub mod models;
pub mod queries;

use sqlx::MySqlPool;

pub async fn init_pool(database_url: &str) -> Result<MySqlPool, sqlx::Error> {
    queries::create_pool(database_url).await
}
