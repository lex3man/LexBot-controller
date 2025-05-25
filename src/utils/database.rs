use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn get_connector(database_url: &String) -> Pool<Postgres> {
    let pool = match PgPoolOptions::new()
        .max_connections(16)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };
    pool
}