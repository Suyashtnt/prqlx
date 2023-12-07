use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
const DB_URL: &str = "sqlite://sqlite.db";

async fn setup_db() -> SqlitePool {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        Sqlite::create_database(DB_URL).await.unwrap()
    } else {
        println!("Database already exists");
    }

    let pool = SqlitePool::connect(DB_URL).await.unwrap();
    sqlx::query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY NOT NULL, name VARCHAR(250) NOT NULL);")
        .execute(&pool)
        .await
        .unwrap();

    pool
}

#[tokio::main]
async fn main() {
    println!("Setting up database...");
    setup_db().await;
    println!("Database setup complete");
}
