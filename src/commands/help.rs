use std::{
    collections::{HashSet}
};

use serenity::prelude::*;
use serenity::{
    framework::standard::{
        help_commands,
        macros::{help},
        Args,
        CommandGroup,
        CommandResult,
        HelpOptions,
    },
    model::{
        channel::{Message},
        id::UserId,
    },
};

#[help]
#[individual_command_tip = "**Help**\n\n To list command options and usage, use `.help <command>`."]
#[command_not_found_text = "Could not find: `{}`."]
#[lacking_permissions = "Nothing"]
#[strikethrough_commands_tip_in_guild("\nStricken commands must be used in DM.\n\n")]
#[strikethrough_commands_tip_in_dm("\nStricken commands must be used in a guild.\n\n")]
#[embed_error_colour(red)]
#[embed_success_colour(blue)]
async fn my_help (
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}