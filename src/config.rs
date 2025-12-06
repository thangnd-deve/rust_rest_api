// load config from .env file

pub fn load_config() {
    dotenvy::dotenv().ok();
}

pub fn get_config_by_key(key: &str, default: Option<&str>) -> String {
    std::env::var(key).unwrap_or_else(|_| default.unwrap_or_default().to_string())
}
