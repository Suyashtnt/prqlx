# PRQLX

Combining the query language of [PRQL](https://prql-lang.org) with the macro powers of [SQLX](https://docs.rs/sqlx).

PRQL is an amazing DSL for sql, but it doesn't have native support in sqlx. This crate bridges the gap by compiling prql _in the rust macro itself_ before it's sent to sqlx (and your database). This means you can use the prql syntax with:

- All supported sqlx database
- Nice compiler errors
- Fully typed rust

## Usage

```sh
cargo add prqlx
cargo add sqlx # requires sqlx to be installed by you so you can configure it correctly
```

```rs
use prqlx::{query, query_as};

async fn test_query() {
    // get your sqlx pool in whatever way
    let pool = get_database_pool().await;

    // simply use it like a regular sqlx query, except you now use PRQL!
    let val = query!(
        "
        from users
        select { id, name }
        "
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    // Same thing with query_as!
    // Bindings also just work:tm:
    let val2 = query_as!(
        User,
        "
        from users
        select { id, name }
        filter id == $1
        ",
        123
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    println!("{:?}", val2);
}
```

## Caveats

PRQL doesn't natively support `?` for bindings. You will have to use `s"?"` for that. Rather use `${num}` for bindings.

## Todos

Not all macros have been implemented. The only ones are `query` and `query_as`. If you would like another to be added, please file an issue (or make a PR!).

## Testing/Contributing

Before running `cargo test`, the database must first be setup. To do so, run `cargo run --example prepare` to prepare the sqlite database. After that, you can develop as usual.
