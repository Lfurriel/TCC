use std::collections::HashMap;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn create_connection_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .max_size(20)                                        
        .min_idle(Some(5))                                   
        .connection_timeout(Duration::from_secs(30))         
        .idle_timeout(Some(Duration::from_secs(300)))        
        .max_lifetime(Some(Duration::from_secs(1800)))       
        .test_on_check_out(true)                             
        .build(manager)
        .expect("Failed to create connection pool")
}

pub async fn get_connection_with_retry(pool: &DbPool, max_retries: u32) -> Result<DbConnection, String> {
    for attempt in 1..=max_retries {
        match pool.get() {
            Ok(conn) => return Ok(conn),
            Err(e) if attempt == max_retries => return Err(format!("Failed after {} attempts: {}", max_retries, e)),
            Err(_) => {
                log::warn!("DB connection attempt {} failed, retrying...", attempt);
                tokio::time::sleep(Duration::from_millis(100 * attempt as u64)).await;
            }
        }
    }
    unreachable!()
}

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<DbPool>,
    pub cache: Arc<RwLock<HashMap<String, (String, std::time::Instant)>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            db_pool: Arc::new(create_connection_pool()),
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_cached<T>(&self, key: &str, ttl_secs: u64) -> Option<T>
    where
        T: serde::de::DeserializeOwned
    {
        let cache = self.cache.read().await;
        if let Some((value, timestamp)) = cache.get(key) {
            if timestamp.elapsed().as_secs() < ttl_secs {
                return serde_json::from_str(value).ok();
            }
        }
        None
    }

    pub async fn set_cached<T>(&self, key: String, value: &T) -> Result<(), String>
    where
        T: serde::Serialize
    {
        let serialized = serde_json::to_string(value)
            .map_err(|e| format!("Serialization error: {}", e))?;

        let mut cache = self.cache.write().await;
        cache.insert(key, (serialized, std::time::Instant::now()));
        Ok(())
    }
}