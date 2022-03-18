/*

A Discord bot written with Serenity.

*/

// Commands module
mod commands;

// Std collections::HashSet, env and sync::Arc
use std::{collections::HashSet, env, sync::Arc};

// Enabled modules
use commands::{math::*, meta::*, owner::*, help::*, admin::*};

// Serenity
use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::standard::{
        macros::{group},
        StandardFramework
    },
    http::Http,
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};

// Error tracing
use tracing::{error, info};

// Database functionality
use sqlx::*;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    /*
    async fn guild_create(&self, ctx: Context, guild: Guild, _is_new: bool) {
        let data_read = ctx.data.read().await;
        let database = data_read.get::<Database>().expect("Expected Database in TypeMap.");

        let guild_id = guild.id.0 as i64;
        let prefix = ".".to_owned();

        let _query = sqlx::query!("INSERT INTO guild_prefix (guild_id, prefix) VALUES ($1, $2)", guild_id, prefix).execute(database).await;
    }
    */

    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

struct Database;

impl TypeMapKey for Database {
    type Value = Pool<Sqlite>;
}

#[tokio::main]
async fn main() {
    // This will load the environment variables located at `./.env`, relative to
    // the CWD. See `./.env.example` for an example on how to structure this.
    dotenv::dotenv().expect("Failed to load .env file");

    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);

    // We will fetch your bot owners and id
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Initiate a connection to the database file, creating the file if required.
    let database = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename("./config/config.sqlite")
                .create_if_missing(true),
        )
        .await
        .expect("Couldn't connect to database");

    // Run migrations, which updates the database's schema to the latest version.
    sqlx::migrate!("./migrations").run(&database).await.expect("Couldn't run database migrations");

    // Create the framework
    let framework =
        StandardFramework::new().configure(|c| c
                .owners(owners)
                .dynamic_prefix(| ctx, msg | Box::pin (

                    async move {

                        let data_read = ctx.data.read().await;
                        let database = data_read.get::<Database>().expect("Expected Database in TypeMap.");

                        let mut guild_id= 0;

                        if let Some(guild) = msg.guild_id {
                            guild_id = guild.as_ref().0 as i64
                        }

                        let mut p;

                        let query = sqlx::query!(
                                "SELECT prefix FROM guild_prefix WHERE guild_id = $1",
                                guild_id)
                            .fetch_one(database)
                            .await;

                        match query {
                            Ok(query) => {
                                p = Some(query.prefix);
                            },
                            Err(e) => {
                                eprintln!("{:?}", e);
                                p = None;
                            }
                        }

                        if p.is_none() {
                            p = Some(".".to_owned())     // Create a default prefix which disappears when the server has a custom one
                        }

                        p       // Returns the prefix for the current guild or DM (always "." on DM)
                    }
                ))
                .prefixes(vec!["<@!952177125442617354> ", "<@!952177125442617354>"]))
                // Always access commands if you ping the bot

            // command groups
                .group(&GENERAL_GROUP)
                .group(&ADMIN_GROUP)
                .group(&OWNER_GROUP)

            // help command
                .help(&MY_HELP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");
    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        data.insert::<Database>(database);
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}

// Command groups

#[group]
#[commands(multiply, ping)]
#[summary("Commands without any category assigned.")]
struct General;

#[group]
#[commands(kick, ban, set)]
#[summary("Commands necessary for server administration.")]
struct Admin;

#[group]
#[owners_only]
#[summary("Commands available solely to the bot developer.")]
#[commands(quit, exit)]
struct Owner;
