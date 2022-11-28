use serenity::async_trait;
use serenity::framework::standard::macros::group;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use std::env;

struct Handler;

#[group]
struct General;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, _msg: Message) {}
    async fn ready(&self, _: Context, _ready: Ready) {}
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("No discord token in the environment");
    println!("Hello, world!");
}
