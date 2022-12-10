use tokio_postgres::{Client, NoTls, Error};

/// Create tables for PostgreSQL and connection between code and
/// database.
///
/// Example
/// ```rust
/// let mut client = init().await;
/// client.query("SELECT * FROM warn WHERE affected_id = ?", &[&"bad_user"]);
/// ```
pub async fn init() -> Result<Client, Error> {
    let (client, connection) = tokio_postgres::connect(&format!("host=localhost user={} password={} dbname={}",
                                                                dotenv::var("DB_USERNAME").expect("Missing env `DB_USERNAME`"),
                                                                dotenv::var("DB_PASSWORD").expect("Missing env `DB_PASSWORD`"),
                                                                dotenv::var("DB_NAME").expect("Missing env `DB_NAME`"),
    )[..], NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client.query("CREATE TABLE IF NOT EXISTS warn ( \
        id              BIGSERIAL PRIMARY KEY, \
        affected_id     TEXT NOT NULL, \
        user_id         TEXT NOT NULL, \
        ip              TEXT, \
        country         TEXT, \
        reason          SMALLINT NOT NULL \
    )", &[]).await?;

    // Date eg. 1999-01-08
    // punishment: 0 suspend account | 1 deleted post | 2 airblog suspended acc.
    client.query("CREATE TABLE IF NOT EXISTS punishment ( \
        id              BIGSERIAL PRIMARY KEY, \
        user_id         TEXT NOT NULL, \
        mod_id          TEXT NOT NULL, \
        reason          SMALLINT NOT NULL, \
        punishment      INT NOT NULL, \
        expiration      date \
    )", &[]).await?;

    Ok(client)
}