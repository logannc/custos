use crate::config::Configuration;
use crate::db::DbPool;
use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

pub fn scene_command_group(
    ctx: &Context,
    msg: &Message,
    config: &Configuration,
    db: &DbPool,
) -> CommandResult {
    Ok(())
}
