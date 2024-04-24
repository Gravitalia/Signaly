use signaly_db::cassandra::Manager as ScyllaManager;

#[tokio::main]
async fn main() -> signaly_error::Result<()> {
    println!("Hello, world!");

    let session = ScyllaManager::new(
        vec!["127.0.0.1:9042".to_string()],
        Some("cassandra".to_string()),
        Some("cassandra".to_string()),
        10,
    ).await;

    println!("{:?}", session);

    Ok(())
}
