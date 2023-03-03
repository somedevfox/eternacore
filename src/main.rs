#[macro_use]
extern crate log;

use figment::{
    providers::{Env, Format, Json, Serialized, Toml},
    Figment,
};
use serenity::{gateway::GatewayError, prelude::GatewayIntents, Client};
use std::{env, process};

use eternacore::config::Config;
use eternacore::handler::Handler;

pub mod logger;

fn get_privileged_intents(intents: GatewayIntents) -> GatewayIntents {
    let mut privileged_intents = GatewayIntents::empty();
    if intents.message_content() {
        privileged_intents.insert(GatewayIntents::MESSAGE_CONTENT);
    }
    if intents.guild_presences() {
        privileged_intents.insert(GatewayIntents::GUILD_PRESENCES)
    }
    if intents.guild_members() {
        privileged_intents.insert(GatewayIntents::GUILD_MEMBERS);
    }
    privileged_intents
}

#[tokio::main]
async fn main() {
    let config: Config = match Figment::from(Serialized::defaults(Config::default()))
        .merge(Toml::file("eternacore.toml"))
        .merge(Env::prefixed("ECORE_"))
        .join(Json::file("eternacore.json"))
        .extract()
    {
        Ok(conf) => conf,
        Err(err) => {
            eprintln!("Couldn't get the configuration file due to `{}`", err);
            process::exit(1);
        }
    };
    // We can safely unwrap, because the only time it will
    // throw an [error](https://docs.rs/log/latest/log/struct.SetLoggerError.html)
    // is if a logger was already set
    logger::Logger::from_config(config.log).unwrap();
    info!(
        "Eternacore Discord Bot Revision {}",
        env!("CARGO_PKG_VERSION")
    );

    let mut intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::DIRECT_MESSAGES;
    if config.discord.intents.message_content {
        intents |= GatewayIntents::MESSAGE_CONTENT;
    }
    if config.discord.intents.presence {
        intents |= GatewayIntents::GUILD_PRESENCES;
    }
    if config.discord.intents.server_members {
        intents |= GatewayIntents::GUILD_MEMBERS;
    }

    let mut client = match Client::builder(&config.discord.token, intents)
        .event_handler(Handler)
        .await
    {
        Ok(client) => client,
        Err(err) => {
            error!("Error while connecting to the Discord Gateway: {:?}", err);
            process::exit(1);
        }
    };

    if let Err(why) = client.start_shards(config.discord.shards).await {
        // TODO: add more cases with messages which user can understand
        match why {
            serenity::Error::Gateway(gateway_error) => match gateway_error {
                GatewayError::InvalidAuthentication =>
                    error!("Authentication failure, did you provide an invalid token?"),
                GatewayError::DisallowedGatewayIntents => error!("Tried using privileged intents without them being enabled in bot dashboard. 
                \tEither disable them in the configuration file
                \tOr visit https://discord.com/developers/applications/{}/bot and enable these intents:
                \t{:?}", client.cache_and_http.http.get_current_user().await.unwrap().id, get_privileged_intents(intents)),
                gateway_error => error!("Gateway error: {}", gateway_error.to_string()),
            },
            why => error!("Client error: {}", why.to_string()),
        }
    }
}
