#![feature(once_cell)]

use serenity::client::Context;
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult,
};
use serenity::model::channel::Message;
use serenity::Result;
use std::lazy::SyncLazy;
use tracing::info;

static CONFIGURATION: SyncLazy<custos::config::Configuration> =
    SyncLazy::new(|| custos::config::load_config());

static DB_POOL: SyncLazy<custos::db::DbPool> = SyncLazy::new(|| custos::db::create_pool());

#[group]
#[commands(beat, init, scene, debug_info)]
struct General;

#[command]
#[allowed_roles(Player)]
async fn beat(ctx: &Context, msg: &Message) -> CommandResult {
    custos::beat::beat_command_group(ctx, msg, &CONFIGURATION, &DB_POOL).await
}

#[command]
#[allowed_roles(Storyteller)]
async fn init(ctx: &Context, msg: &Message) -> CommandResult {
    custos::init::initiative_command_group(ctx, msg, &CONFIGURATION, &DB_POOL).await
}

#[command]
#[allowed_roles(Storyteller)]
async fn scene(ctx: &Context, msg: &Message) -> CommandResult {
    custos::scene::scene_command_group(ctx, msg, &CONFIGURATION, &DB_POOL).await
}

#[command]
#[allowed_roles(Archmage)]
async fn debug_info(ctx: &Context, msg: &Message) -> CommandResult {
    custos::debug::debug_info(ctx, msg, &CONFIGURATION, &DB_POOL).await
}

#[tracing::instrument]
fn force_lazies() {
    info!("Loading config...");
    SyncLazy::force(&CONFIGURATION);
    info!("Loading database connection...");
    SyncLazy::force(&DB_POOL);
}

#[tokio::main]
async fn main() -> Result<()> {
    // Start tracing first
    let _log_flush_guard = custos::tracing::init_tracing();
    // Force configuration and lazy loading so we error now instead of later.
    force_lazies();
    let mut client = custos::client::build_serenity_client(&GENERAL_GROUP).await;
    info!("Starting Serenity client...");
    client.start().await
}
