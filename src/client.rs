use serenity::client::Client;
use serenity::framework::standard::CommandGroup;
use serenity::framework::standard::StandardFramework;
use serenity::prelude::GatewayIntents;
use std::env;
use tracing::info;

#[tracing::instrument(skip(command_group))]
pub async fn build_serenity_client(command_group: &'static CommandGroup) -> Client {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(command_group);
    let token = env::var("DISCORD_TOKEN").expect("$DISCORD_TOKEN env not set");
    info!("Creating Serenity client...");
    let client = Client::builder(
        token,
        GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT,
    )
    .framework(framework)
    .await
    .expect("Error creating client");
    client
}
