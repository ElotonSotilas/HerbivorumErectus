# Herbivorum Erectus
A vegan Discord bot written in Rust.

## Information

### Discord Library
The bot is written using the Serenity library.

### Frameworks Used
```toml
[dependencies.sqlx]
version = "0.5.11"
features = ["runtime-tokio-rustls", "sqlite", "offline"]

[dependencies.tokio]
version = "1.0"
features = ["macros", "signal", "rt-multi-thread"]

[dependencies]
dotenv = "0.15"
tracing = "0.1.23"
tracing-subscriber = "0.2"
regex = "1"
lazy_static = "1.4.0"
```

## Self-hosting the bot
Non-original copies of this bot must preserve the `info` command, as well as a watermark on the help command, which leads to this GitHub page. Also, self-hosted instances are ineligible of using premium features, as these will not be uploaded to this repository.

The following instructions will assume you are using a UNIX-like operating system and that you have already installed `git` on your system.

- **Step 1** Clone this repository and enter the directory produced thereof by running this command in your terminal:
```bash
git clone https://github.com/ElotonSotilas/HerbivorumErectus DiscordBot/ && cd DiscordBot/
```

- **Step 2** Create an `.env` file, containing the following configuration, replacing `token` with the respective value of your bot account's token:
```env
DISCORD_TOKEN=token
RUST_LOG=info
DATABASE_URL=sqlite:config/config.sqlite
```

- **Step 3** Source the `.env` file to your terminal session, to obtain the variables.
```bash
source .env
```

- **Step 4** Start the bot with the following command:
```bash
cargo run
```

## TODO

When a module is completed, it will be checkmarked and enabled for use globally.

- [ ] Finish the **Admin** module with the following commands:
  - `set` — per-guild settings.
    - `prefix <prefix>` — Sets bot prefix in the current server. Pass empty prefix by simply running `set prefix` to disable and revert to `.` as the default prefix.
    - `welcome <todm/tochannel/addquestion/rmquestion/onsuccess/onfailure/disable/enable> [args...]` — Let users verify by taking a quiz upon joining your server (**premium only** feature)
    - `logging <channel/embed/setproperty> <option> [option2]` — Provides a moderator logging utility based on server events. May enable/disable entire groups instead of doing it individually, too (all/message/member/role/channel/server/emoji/reactions).
    - `adminrole [role mention or ID]` - Sets Admin role for the current server. Pass empty role argument to disable.
    - `modrole [role mention or ID]` — Sets Moderator role for the current server. Pass empty role argument to disable.
    - `muterole [role mention or ID]` — Sets a role to be given to muted people on the current server. If not set, the bot will automatically create a Muted role right below its own role in hierarchy when you mute a member for the first time.
    - `embeds <always/individual/never>` — Enables/disables embeds for all sorts of messages. Default value will be `individual`. Setting to `always` or `never` will disable and override individual setting.
    - `dm <all/on_kick/on_ban/on_mute/on_warn> [true/false]` — Enables/disables direct messages on moderated people. (**premium only** feature)
    - `delinvokecmd [true/false]` — Delete the message containing the bot command automatically.
    - `penalty <list/add/remove/edit> [args...]` — Automatic penalty management on a per-warning basis.
    - `alias <add/remove> <command> [alias]` — Add/remove custom command aliases. If `remove` is cast on a command with no alias provided, it will clear all aliases related to that command.
    - `perms <command> [admin/mod/owner]` — Make command available to the admin or moderator role set to the bot. Alternatively, permit only the server owner to use the command.
    - `mod <module_name> [on/off]` — Enables/disables a command module for the current server. May not disable General and Info modules.
  - `ban <mention(s)> <purge days> <number:s/m/d/w/mo/y> [reason]` — Bans a member from the server temporarily (time in seconds, minutes, days, weeks, months, years). Example `7:days` will ban the person for 7 days. If no time identifier is provided, it will default to days. If no time is provided, the ban will be indefinite. Sends reason to DM if `set dm on_ban` has the value of true.
  - `unban <user id(s)> [reason]` — Unban people from the server and send optional reason to modlog.
  - `warn <mention(s)> [reason]` — Sends a warning to the member and stores it in the database. Sends reason to DM if `set dm on_warn` is enabled.
  - `ewarn <mention> <warning case number> [new reason]` — Edits a warning with a new reason.
  - `delwarn <mention> [warning case number]` — Deletes a warning for a member from their record.
  - `lswarn <mention> [page]` — List all warnings applied to a member.
  - `mute <mention(s)> [number:s/m/d/w/mo/y] [reason]` — Mutes a member on the server. Sends reason to DM if `set dm on_mute` is enabled.
  - `unmute <mention(s)> [reason]` — Unmutes a member with optional reason.
  - `addrole <mention(s)> <role(s)> [time]` — Add roles to members. Can be done temporarily.
  - `remrole <mention(s)> <role(s)> [time]` — Remove roles from members. Can be done temporarily.
  - `sudo <mention(s)> <command>` — Run a command as another server member. (**premium only** feature)

- [ ] Create a **Social** module.
- [ ] Create an **Info** module.
- [ ] Create a **Profile** module.

It is entirely possible I decide to implement more features later on, or drop some of the features listed here.
