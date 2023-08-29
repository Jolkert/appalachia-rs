use std::fs;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

#[group]
#[commands(ping)]
struct General;

struct Handler;
#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);

    // TODO: make token passing better - morgan 2023-08-28
    let token = fs::read_to_string("./.token").expect("Unable to read token file!");
    let intents = GatewayIntents::all();
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(reason) = client.start().await {
        println!("An error occurred while running the client: {:?}", reason)
    }
    println!("how does this work?");
}

#[command]
async fn ping(context: &Context, message: &Message) -> CommandResult {
    message.reply(context, "Pong!").await?;
    Ok(())
}
