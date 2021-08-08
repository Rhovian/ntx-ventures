/* REQUIREMENTS
Provide a setup function to establish a connection with credentials.
Hold a persistent connection to db.
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
use sqlx::mysql::MySqlRow;
use std::error::Error;

extern crate dotenv;
use dotenv::dotenv;

// Provide a setup function to establish a connection with credentials.
pub async fn setup() -> Result<MySqlPool, Box<dyn Error>> {
    dotenv().ok();
    let pool = MySqlPool::connect(&dotenv::var("DATABASE_URL").unwrap().to_owned()).await?;
    Ok(pool)
}
// Provide a query function for sending raw queries to db.
pub async fn query(pool: &MySqlPool, query: &str) -> Result<MySqlQueryResult, Box<dyn Error>> {
    let result = sqlx::query(query).execute(pool).await?;
    Ok(result)
}

// necessary to return reads
pub async fn select_query(
    pool: &MySqlPool,
    query: &str,
) -> Result<Vec<MySqlRow>, Box<dyn std::error::Error>> {
    let result = sqlx::query(query).fetch_all(pool).await?;
    Ok(result)
}
