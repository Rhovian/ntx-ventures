/* REQUIREMENTS
* Provide a setup function to establish a connection with credentials.
* Hold a persistent connection to db.
Provide a query function for sending raw queries to db.
Provide tests for testing query function for SELECT, ADD, UPDATE, and DELETE functionality.
Run setup tests to verify that
connection has been successfully setup
Insert function is working
Read function is working
Delete function is working
*/

use sqlx::mysql::MySqlPool;
use sqlx::mysql::MySqlQueryResult;
use std::env;

// Provide a setup function to establish a connection with credentials.
pub async fn setup() -> Result<MySqlPool, Box<dyn std::error::Error>> {
    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;
    Ok(pool)
}

pub async fn query(pool: MySqlPool, query: &str) -> Result<MySqlQueryResult, Box<dyn std::error::Error>> {
    let result = sqlx::query(query).execute(&pool).await?;
    Ok(result)
}