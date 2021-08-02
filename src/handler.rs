/// For the Handler (impl EventHandler)

use std::collections::HashSet;

use serenity::{async_trait, prelude::*};
use serenity::framework::standard::*;
use serenity::framework::standard::macros::*;
use serenity::model::prelude::*;


use crate::config::*;
use crate::helpers::*;
use crate::commands::*;


/// `!help` function
#[help]
async fn my_help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let mut new_options = help_options.clone();
    new_options.strikethrough_commands_tip_in_dm = None;
    new_options.strikethrough_commands_tip_in_guild = None;

    let _help = help_commands::with_embeds(ctx, msg, args, &new_options, groups, owners).await;
    Ok(())
}


#[group]
#[commands(info)]
/// Get server info
struct General;
/// Discord handler
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}