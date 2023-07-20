use cdrs::authenticators::StaticPasswordAuthenticator;
use cdrs::cluster::session::{new as new_session, Session};
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::load_balancing::RoundRobin;
use cdrs::query::*;

type CurrentSession = Session<RoundRobin<TcpConnectionPool<StaticPasswordAuthenticator>>>;
use once_cell::sync::OnceCell;
static SESSION: OnceCell<CurrentSession> = OnceCell::new();

/// Init cassandra session
pub fn init() {
    let _ = SESSION.set(
        new_session(
            &ClusterTcpConfig(
                vec![
                    NodeTcpConfigBuilder::new(
                        dotenv::var("CASSANDRA_HOST").unwrap_or_else(|_| "127.0.0.1:9042".to_string()).as_str(),
                        StaticPasswordAuthenticator::new(
                            &dotenv::var("CASSANDRA_USER").unwrap_or_else(|_| "cassandra".to_string()).as_str(),
                            &dotenv::var("CASSANDRA_PASSWORD").unwrap_or_else(|_| "cassandra".to_string()).as_str()
                        )
                    ).build()
                    ]
                ),
            RoundRobin::new()
        )
        .expect("session should be created")
    );
}

/// Create tables in cassandra keyspace if not exists
pub fn create_tables() {
    //SESSION.get().unwrap().query("CREATE KEYSPACE IF NOT EXISTS signaly WITH REPLICATION = { 'class' : 'SimpleStrategy', 'replication_factor' : 1 };").expect("Keyspace create error");
    SESSION.get().unwrap().query("CREATE TABLE IF NOT EXISTS signaly.reports ( id TEXT, affected_id TEXT, author_id TEXT, platform TEXT, reason TINYINT, timestamp TIMESTAMP, PRIMARY KEY (id) ) WITH gc_grace_seconds = 0 AND default_time_to_live = 2630000;").expect("signaly.reports create error");
    SESSION.get().unwrap().query("CREATE TABLE IF NOT EXISTS signaly.punishment ( id TEXT, affected_id TEXT, mod_id TEXT, platform TEXT, reason TINYINT, punishment INT, timestamp TIMESTAMP, PRIMARY KEY (id) ) WITH gc_grace_seconds = 0;").expect("signaly.reports create error");
    SESSION.get().unwrap().query("CREATE TABLE IF NOT EXISTS signaly.suspend ( id TEXT, user_id TEXT, platform TEXT, expire_at TIMESTAMP, PRIMARY KEY (id) ) WITH gc_grace_seconds = 0;").expect("signaly.reports create error");
    SESSION.get().unwrap().query("CREATE INDEX IF NOT EXISTS ON signaly.reports ( affected_id );").expect("index (affected_id) create error");
    SESSION.get().unwrap().query("CREATE INDEX IF NOT EXISTS ON signaly.suspend ( expire_at );").expect("second index (expire_at) create error");
}

/// Make a query to cassandra
pub fn query<Q: ToString>(query: Q, params: Vec<String>) -> Result<cdrs::frame::Frame, cdrs::error::Error> {
    SESSION.get().unwrap().query_with_values(query, params)
}
