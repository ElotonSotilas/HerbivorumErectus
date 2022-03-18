use lazy_static::lazy_static;
use serenity::{
    framework::standard::{
        macros::{command},
        CommandResult
    },
    model::prelude::*,
    prelude::*
};
use regex::Regex;

#[command]
#[only_in(guilds)]
#[description("Kick member(s) from the server. Reason is optional.")]
#[usage("<mention> [mention(s)...] [reason]")]
#[required_permissions(kick_members, administrator)]
pub async fn kick(ctx: &Context, msg: &Message) -> CommandResult {
    let author = &msg.author;
    let kicked_members = &msg.mentions;
    let content = &msg.content;

    lazy_static!{
        static ref RE: Regex = Regex::new(r".* +?<@!?\d+> +?(.+)").unwrap();        // Evaluate regex once during compile time only
    }

    let reason;

    match RE.captures(content) {
        Some(caps) => reason = caps[1].to_owned(),
        None => reason = "".to_owned()
    }

    if let Some(_guild) = msg.guild_id {
        let this_guild = msg.guild_id.unwrap().as_ref().0 as u64;
        let guild_name = msg.guild_id.unwrap().name(&ctx).await.unwrap();

        if kicked_members.is_empty() {                                                      // MENTIONS ARE MISSING
            msg.reply(ctx, "⚠ You need to kick at least **one** member.").await?;
        }

        else if kicked_members.contains(author) {
            msg.reply(&ctx,"😕 You cannot kick yourself.").await?;
        }

        else if reason.is_empty() {                                                         // REASON IS MISSING
            for member in kicked_members {
                member.direct_message(&ctx.http, |m|
                    m.content(
                            format!("👊 You have been kicked from **{}**.", guild_name)
                    )).await?;                                                              // sends DM to member

                ctx.http.kick_member
                (this_guild, member.id.0 as u64).await?;                // Kicks the member
            }
        }

        else if !reason.is_empty() {                                                        // REASON IS PRESENT
            for member in kicked_members {
                member.direct_message(&ctx.http, |m|
                    m.content(
                        format!("👊 You have been kicked from **{}**. \n\n**Reason:** \n> {}", guild_name, reason)
                    )).await?;                                                              // sends DM to member with reason message

                ctx.http.
                    kick_member_with_reason
                        (this_guild, member.id.0 as u64, &reason)
                    .await?;                                                                // Kicks member with reason
            }
        }

        else {
            msg.reply(&ctx,"🚫 Some error has occurred.").await?;
        }
    } else {
        msg.reply(&ctx, "Must be ran in a guild").await?;
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
#[description("Ban member(s) from the server. The purge time and reason are optional.")]
#[usage("<mention> [mention(s)...] [purge days: number only] [reason]")]
#[required_permissions(ban_members, administrator)]
pub async fn ban(ctx: &Context, msg: &Message) -> CommandResult {
    let author = &msg.author;
    let banned_members = &msg.mentions;
    let content = &msg.content;

    lazy_static!{
        static ref RE: Regex = Regex::new(r".* +?<@!?\d+> +?(.+)").unwrap();        // Evaluate regex once during compile time only
    }

    let reason;
    let mut d = "0".to_owned();

    match RE.captures(content) {
        Some(caps) => reason = caps[1].to_owned(),
        None => reason = "".to_owned()
    }

    let mut r = "".to_owned();          // some reason

    if !reason.is_empty() && reason.chars().next().unwrap().is_numeric() {
        d = reason.split_once(" ").unwrap().0.to_owned();           // take first half of the split function and set to days as d
        r = reason.split_once(" ").unwrap().1.to_owned();           // take second half of the split function and set to reason as r
    }

    let days: u8 = d.parse().unwrap();          // parse days to u8

    if let Some(guild) = msg.guild_id {
        let this_guild = guild.as_ref().0 as u64;
        let guild_name = guild.name(&ctx).await.unwrap();

        if banned_members.is_empty() {
            msg.reply(ctx, "⚠ You need to ban at least **one** member.").await?;          // Mentions are missing
        }

        else if banned_members.contains(author) {
            msg.channel_id.say(&ctx.http,"😕 You cannot ban yourself.").await?;        // Handle case where author tries to ban himself
        }

        else if r.is_empty() {                          // NO REASON PROVIDED FOR BAN
            for member in banned_members {

                member.direct_message(&ctx.http, |m|
                    m.content("🔨 You have been banned from the server.")).await?;         // sends DM to the banned user

                ctx.http.ban_user
                (this_guild, member.id.0 as u64, days, "No reason provided.").await?;   // Bans the user
            }
        }

        else if !r.is_empty() {                         // REASON HAS BEEN PROVIDED FOR BAN
            for member in banned_members {

                member.direct_message(&ctx.http, |m|
                    m.content(format!("🔨You have been banned from **{}**. \n\n**Reason:** \n> {}", guild_name, r))).await?;      // sends DM to the banned user

                ctx.http.ban_user
                (this_guild, member.id.0 as u64, days, &r).await?;  // Bans the user

            }
        }

        else {
            msg.channel_id.say(&ctx.http,"🚫 Some error has occurred.").await?;         // Handle other cases (likely cause of permissions)
        }
    } else {
        msg.reply(&ctx, "⛔ Cannot use on DM.").await?;
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn set(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx, "Hello, this section is work in progress. This will be replaced by a help message soon.").await?;

    Ok(())
}