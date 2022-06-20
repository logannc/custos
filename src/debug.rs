use std::collections::VecDeque;
use std::str::FromStr;

use crate::config::Configuration;
use crate::db::DbPool;
use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use strum::{AsRefStr, Display, EnumString, IntoStaticStr};
use tracing::{debug, error, info, info_span, trace, warn};

#[derive(Debug, EnumString, Display, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
enum DebugCommand {
    PrintChannel,
    PrintParentChannel,
}

#[tracing::instrument(skip_all)]
async fn print_channel(ctx: &Context, msg: &Message) -> CommandResult {
    debug!("Attempting to send channel info [{}]", msg.channel_id);
    let channel_id = msg.channel_id;
    if let Ok(channel) = channel_id.to_channel(ctx).await {
        match channel {
            serenity::model::channel::Channel::Guild(guild_channel) => {
                debug!("Found channel info [{}]", &guild_channel);
                let message = format!(
                    "Channel Name: {}, Channel ID: {}",
                    guild_channel.name, guild_channel.id
                );
                if let Err(why) = msg.channel_id.say(&ctx.http, message).await {
                    error!("Error sending message to channel [{}]", why);
                }
                // let parent_channel_id = guild_channel.parent_id.unwrap();
                // if let Ok(parent_channel) = parent_channel_id.to_channel(ctx).await {
                //     match parent_channel {
                //         serenity::model::channel::Channel::Category(parent_category) => {
                //             dbg!(parent_category);
                //         }
                //         _ => println!("unexpected channel kind"),
                //     }
                // }
            }
            _ => println!("Unexpected channel kind"),
        }
    }
    Ok(())
}

#[tracing::instrument(skip_all)]
fn parse_command(msg: &Message) -> Result<DebugCommand, ()> {
    let mut args: VecDeque<String> = msg
        .content
        .split_whitespace()
        .map(|s| s.to_owned())
        .collect();
    assert_eq!(args.pop_front().unwrap(), "!debug_info");
    trace!("args = {:?}", args);
    if let Some(first_arg) = args.front() {
        if let Ok(command) = DebugCommand::from_str(first_arg.as_str()) {
            Ok(command)
        } else {
            warn!("No DebugCommand matched by [{}]", first_arg);
            Err(())
        }
    } else {
        warn!("No further arguments available for parsing.");
        Err(())
    }
}

#[tracing::instrument(skip_all)]
pub async fn debug_info(
    ctx: &Context,
    msg: &Message,
    config: &Configuration,
    db: &DbPool,
) -> CommandResult {
    debug!("Debug command recieved");
    if let Ok(command) = parse_command(msg) {
        match command {
            DebugCommand::PrintChannel => print_channel(ctx, msg).await,
            _ => Ok(()),
        }
        // if let Err(why) = msg.channel_id.say(&ctx.http, command.to_string()).await {
        //     error!("Error sending message to channel [{}]", why);
        // }
    } else {
        Ok(())
    }
    // let channel_id = msg.channel_id;
    // dbg!(channel_id.name(ctx).await);
    // if let Ok(channel) = channel_id.to_channel(ctx).await {
    //     match channel {
    //         serenity::model::channel::Channel::Guild(guild_channel) => {
    //             dbg!(&guild_channel);
    //             let parent_channel_id = guild_channel.parent_id.unwrap();
    //             if let Ok(parent_channel) = parent_channel_id.to_channel(ctx).await {
    //                 match parent_channel {
    //                     serenity::model::channel::Channel::Category(parent_category) => {
    //                         dbg!(parent_category);
    //                     }
    //                     _ => println!("unexpected channel kind"),
    //                 }
    //             }
    //         }
    //         _ => println!("Unexpected channel kind"),
    //     }
    // }
    // Ok(())
}
