mod config;

use std::collections::HashSet;
use std::sync::Arc;

use serenity::async_trait;
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

#[group]
struct General;

#[async_trait]
impl EventHandler for Handler {
    //async fn message(&self, _ctx: Context, _msg: Message) {}
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let config = config::Config::new();
    let token = config.discord_token();
    let http = Http::new(&token);
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);
            (owners, info.id)
        }
        Err(err) => panic!("Could not access application info: {:?}", err),
    };
    let framework = StandardFramework::new().configure(|c| c.owners(owners).prefix('/'));
    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = serenity::Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Failed to create client");
    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }
    let shard_manager = client.shard_manager.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });
    println!(
        "The Lobbymaker bot runs in the {} environment",
        config.bot_environment()
    );
    let database = rusqlite::Connection::open(config.database_uri()).expect(format!("Unable to open the {} database", config.bot_environment()).as_str());
    if let Err(err) = client.start().await {
        println!("Client error: {:?}", err);
    }
}
