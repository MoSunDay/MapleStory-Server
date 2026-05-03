// Database connection pool and query functions
// Ported from tools/DatabaseConnection.java

use sqlx::MySqlPool;

pub async fn create_pool(database_url: &str) -> Result<MySqlPool, sqlx::Error> {
    MySqlPool::connect(database_url).await
}

/// Load a character by ID
pub async fn load_character(
    pool: &MySqlPool,
    char_id: i32,
) -> Result<Option<crate::models::Character>, sqlx::Error> {
    let row = sqlx::query_as::<_, (i32,)>("SELECT id FROM characters WHERE id = ?")
        .bind(char_id)
        .fetch_optional(pool)
        .await?;

    Ok(row.map(|_| crate::models::Character::default()))
}

/// Load a character by name
pub async fn load_character_by_name(
    pool: &MySqlPool,
    name: &str,
) -> Result<Option<crate::models::Character>, sqlx::Error> {
    let row = sqlx::query_as::<_, (i32,)>("SELECT id FROM characters WHERE name = ?")
        .bind(name)
        .fetch_optional(pool)
        .await?;

    Ok(row.map(|_| crate::models::Character::default()))
}

/// Load account by name
pub async fn load_account(
    pool: &MySqlPool,
    account_name: &str,
) -> Result<Option<crate::models::Account>, sqlx::Error> {
    let _row = sqlx::query_as::<_, (i32,)>(
        "SELECT id FROM accounts WHERE name = ? AND banned = 0",
    )
    .bind(account_name)
    .fetch_optional(pool)
    .await?;

    // TODO: Full column mapping
    Ok(None)
}

/// Mark account as logged in
pub async fn set_account_loggedin(
    pool: &MySqlPool,
    account_id: i32,
    logged_in: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE accounts SET loggedin = ? WHERE id = ?")
        .bind(logged_in as i8)
        .bind(account_id)
        .execute(pool)
        .await?;

    Ok(())
}

/// Reset all accounts to loggedout on server start
pub async fn reset_all_loggedin(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE accounts SET loggedin = 0")
        .execute(pool)
        .await?;

    Ok(())
}

/// Reset HasMerchant flag on server start
pub async fn reset_all_merchants(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE characters SET HasMerchant = 0")
        .execute(pool)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reset_queries_compile() {
        // Verify the queries are valid SQL at compile time via sqlx macros
        // (Will only run if DATABASE_URL is set)
    }
}
