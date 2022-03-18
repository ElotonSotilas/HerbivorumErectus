use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::ShardManagerContainer;

#[command]
#[aliases(turnoff, off, shutdown, poweroff)]
pub async fn quit(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        msg.reply(ctx, "🔌 Shutting down...").await?;
        manager.lock().await.shutdown_all().await;
    } else {
        msg.reply(ctx, "⚠ There was a problem getting the shard manager").await?;
        return Ok(())
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
#[description("Leaves the current guild.")]
pub async fn exit(ctx: &Context, msg: &Message) -> CommandResult {
    if let Some(guild_id) = msg.guild_id {
        msg.reply(ctx,"👋 Bye!").await?;

        if let Err(e) = guild_id.leave(ctx).await {
            msg.reply(ctx, "⚠ Must be ran in a server.").await?;
            return Err(e.into())
        }

        Ok(())          // Success
    } else {
        msg.reply(ctx, "⛔ Cannot leave guild.").await?;
        Ok(())          // Returns message for unsuccessful operation
    }
}