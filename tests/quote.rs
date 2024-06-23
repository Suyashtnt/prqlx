use prqlx::{query, query_as};
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

    sqlx::query("INSERT OR IGNORE INTO users (id, name) VALUES (123, 'John Doe');")
        .execute(&pool)
        .await
        .unwrap();

    pool
}

#[tokio::test]
async fn test_query() {
    let pool = setup_db().await;

    let val = query!(
        "
        from users
        select { id, name }
        "
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    println!("{:?}", val);
}

#[derive(Debug)]
#[allow(dead_code)]
struct User {
    id: i64,
    name: String,
}

#[tokio::test]
async fn test_query_as() {
    let pool = setup_db().await;

    let val = query_as!(
        User,
        "
        from users
        select { id, name }
        "
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    assert_eq!(val[0].name, "John Doe");
}

#[tokio::test]
async fn test_query_with_args() {
    let pool = setup_db().await;

    let user_id = 123i64;
    let val = query_as!(
        User,
        "
        from users
        select { id, name }
        filter id == $1
        ",
        user_id
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(val.name, "John Doe");
}
