use serenity::client::{Client, Context};
use serenity::framework::standard::{
    help_commands,
    macros::{command, group, help},
    Args, CommandGroup, CommandResult, HelpOptions, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::model::prelude::UserId;
use serenity::Result;
use std::collections::HashSet;
use std::env;

#[group]
#[commands(init, scene, beat)]
struct General;

#[command]
#[allowed_roles(Storyteller)]
async fn init(ctx: &Context, msg: &Message) -> CommandResult {
    dbg!(msg);
    Ok(())
}

#[command]
#[allowed_roles(Storyteller)]
async fn scene(ctx: &Context, msg: &Message) -> CommandResult {
    dbg!(msg);
    Ok(())
}

#[command]
#[allowed_roles(Player)]
#[sub_commands(list)]
async fn beat(ctx: &Context, msg: &Message) -> CommandResult {
    dbg!(msg);
    Ok(())
}

#[command]
#[allowed_roles(Player)]
async fn list(ctx: &Context, msg: &Message) -> CommandResult {
    dbg!(msg);
    Ok(())
}

#[help]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    // https://docs.rs/serenity/latest/serenity/framework/standard/help_commands/fn.searched_lowercase.html
    // let _ = dbg!(help_commands::create_customised_help_data(context, msg, &args, groups, &owners, help_options).await);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP)
        .help(&MY_HELP);

    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN env not set");
    let mut client = Client::builder(token)
        .framework(framework)
        .await
        .expect("Error creating client");

    println!("Creating/Opening DB Pool...");
    use sqlx::sqlite::SqliteConnectOptions;
    use std::str::FromStr;
    // let pool = SqlitePool::connect(db_path).await.unwrap();
    // let conn = SqliteConnectOptions::from_str("sqlite://data.db")
    //     .unwrap()
    //     // .journal_mode(SqliteJournalMode::Wal)
    //     .create_if_missing(true)
    //     .connect()
    //     .await
    //     .unwrap();
    let connection_options = SqliteConnectOptions::from_str("sqlite://data.db").unwrap().create_if_missing(true);
    let pool = sqlx::sqlite::SqlitePool::connect_with(connection_options).await.unwrap();
    println!("Starting client...");
    client.start().await
}
