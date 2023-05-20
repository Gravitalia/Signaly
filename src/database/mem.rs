use memcache::{Client, MemcacheError};
use once_cell::sync::OnceCell;
static SESSION: OnceCell<Client> = OnceCell::new();

pub fn init() -> Result<(), MemcacheError> {
    let _ = SESSION.set(memcache::connect(format!("memcache://{}?timeout=2&tcp_nodelay=true", dotenv::var("MEMCACHED_HOST").unwrap_or_else(|_| "127.0.0.1:11211".to_string())))?);

    Ok(())
}

pub fn set(key: String, value: u16) -> Result<String, MemcacheError> {
    SESSION.get().unwrap().set(&key, value, 300)?;

    Ok(key)
}

pub fn get(key: String) -> Result<Option<String>, MemcacheError> {
    let value: Option<String> = SESSION.get().unwrap().get(&key)?;

    Ok(value)
}
