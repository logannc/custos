use crate::config::Configuration;
use crate::db::DbPool;
use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use tracing::{debug, error, info, info_span, trace, warn};

#[tracing::instrument(skip_all)]
pub async fn scene_command_group(
    ctx: &Context,
    msg: &Message,
    config: &Configuration,
    db: &DbPool,
) -> CommandResult {
    debug!("Scene command recieved");
    Ok(())
}
