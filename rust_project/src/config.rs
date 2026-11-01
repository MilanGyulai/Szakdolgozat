use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub uri: String,
    pub database: String,
    pub collection: String,
    pub app_name: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            uri: env_or(
                "MONGODB_URI",
                "mongodb://localhost:27017/?directConnection=true",
            ),
            database: env_or("MONGODB_DB", "School"),
            collection: env_or("MONGODB_COLLECTION", "students_imported"),
            app_name: env_or("APP_NAME", "school-app"),
        }
    }
}

fn env_or(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}
