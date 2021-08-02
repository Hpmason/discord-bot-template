use serenity::{client::Context, framework::standard::{Args, CommandResult, macros::command}, model::channel::Message};

use crate::helpers::*;
use crate::config::*;

#[command]
#[description("Gets info about bot")]
#[num_args(0)]
async fn info(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    msg.channel_id.say(&ctx.http, BOT_INFO).await?;
    Ok(())
}