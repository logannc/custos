use std::env;
use serenity::client::{Client, Context};
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::Result;

#[command]
async fn init(ctx: &Context, msg: &Message) -> CommandResult {
    dbg!(msg);
    msg.reply(ctx, "recieved init").await?;
    Ok(())
}

#[command]
async fn scene(ctx: &Context, msg: &Message) -> CommandResult {
    dbg!(msg);
    msg.reply(ctx, "recieved scene").await?;
    Ok(())
}

#[command]
async fn beat(ctx: &Context, msg: &Message) -> CommandResult {
    dbg!(msg);
    msg.reply(ctx, "recieved beat").await?;
    Ok(())
}

#[group]
#[commands(init, scene, beat)]
struct General;

#[tokio::main]
async fn main() -> Result<()> {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN env not set");
    let mut client = Client::builder(token)
        .framework(framework)
        .await
        .expect("Error creating client");

    println!("Starting client...");
    client.start().await
}
