use std::env;

pub struct Config;

impl Config {
    pub fn new() -> Config {
        dotenv::dotenv().expect("Loading .env failed");
        Config
    }

    pub fn discord_token(&self) -> String {
        env::var("DISCORD_TOKEN").expect("Failed to load DISCORD_TOKEN")
    }

    pub fn bot_environment(&self) -> String {
        env::var("BOT_ENVIRONMENT").expect("Failed to load BOT_ENVIRONMENT")
    }
}
