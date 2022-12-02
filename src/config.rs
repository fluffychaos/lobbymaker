use std::{env, path::{Path, PathBuf}};

pub struct Config {
    discord_token: String,
    bot_environment: String,
    database_base_uri: String,
    database_uri: PathBuf
}

impl Config {
    pub fn new() -> Config {
        dotenv::dotenv().expect("Loading .env failed");
        let discord_token = env::var("DISCORD_TOKEN").expect("Failed to load DISCORD_TOKEN");
        let bot_environment = env::var("BOT_ENVIRONMENT").expect("Failed to load BOT_ENVIRONMENT");
        let database_base_uri = env::var("DATABASE_BASE_URI").expect("Failed to load MONGO_URI");
        let database_uri = Path::new(&database_base_uri).join(format!("{}.sqlite", &bot_environment));
        Config {
            discord_token,
            bot_environment,
            database_base_uri,
            database_uri
        }
    }

    pub fn discord_token(&self) -> &str {
        &self.discord_token
    }

    pub fn bot_environment(&self) -> &str {
        &self.bot_environment
    }

    pub fn database_base_uri(&self) -> &str {
        &self.database_base_uri
    }

    pub fn database_uri(&self) -> &Path {
        &self.database_uri
    }
}
