use std::time::Duration;
use deadpool_postgres::{Config, Pool, Runtime};

use crate::config::get_config_by_key;

#[derive(Clone)]
pub struct Database {
    pub pool: Pool,
}

impl Database {
    pub async fn new() -> Self {
        let mut config = Config::new();
        let host = get_config_by_key("DATABASE_HOST", Some("localhost"));
        let port = get_config_by_key("DATABASE_PORT", Some("5432"));
        let user = get_config_by_key("DATABASE_USER", Some("user"));
        let password = get_config_by_key("DATABASE_PASSWORD", Some("password"));
        let dbname = get_config_by_key("DATABASE_NAME", Some("postgres"));

        // config the database connection
        config.host = Some(host);
        config.port = Some(port.parse().unwrap_or(5432));
        config.user = Some(user);
        config.password = Some(password);
        config.dbname = Some(dbname);
        config.keepalives = Some(true);
        config.keepalives_idle = Some(Duration::from_secs(60));

        let pool = config
            .create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls)
            .expect("Failed to create connection pool");

        // test the connection
        let client = pool.get().await.expect("Failed to get pool");
        client
            .query("SELECT 1", &[])
            .await
            .expect("Failed to connection");

        Self { pool }
    }
}
