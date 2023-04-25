#![allow(unused)] // silence unused warnings while exploring (to comment out)

use std::str::FromStr;

use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::types::Uuid;
use sqlx::{FromRow, Row};

use chrono::{DateTime, Local, NaiveDateTime};

#[derive(Debug, FromRow)]
struct Users {
    id: uuid::Uuid,
    name: String,
    password: String,
    email: String,
    created_at: NaiveDateTime,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@db:5432/postgres")
        .await?;

    sqlx::migrate!("./migrations/").run(&pool).await?;

    sqlx::query!(
        r#"
INSERT INTO users (id, name, password, email, created_at)
VALUES ($1, $2, $3, $4, $5);
        "#,
        Uuid::new_v4(),
        "Mickel",
        "password",
        "mickel@localhost",
        chrono::Local::now().naive_local()
    )
    .execute(&pool)
    .await?;

    const ALL_USERS_SQL: &str = r#"
SELECT * FROM users;
        "#;

    let users: Vec<Users> = sqlx::query(ALL_USERS_SQL)
        .map(|row: PgRow| {
            let id: Uuid = row.try_get("id").unwrap();
            let id = uuid::Uuid::from_u128(id.as_u128());
            Users {
                id,
                name: row.try_get("name").unwrap(),
                password: row.try_get("password").unwrap(),
                email: row.try_get("email").unwrap(),
                created_at: row.try_get("created_at").unwrap(),
            }
        })
        .fetch_all(&pool)
        .await?;

    println!("here");
    println!("{:?}", users);

    Ok(())
}
