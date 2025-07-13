use std::borrow::Cow;

use auth_handler::UserToken;
use config_handler::{AuthConfig, PokeConfigLoader};

use crate::configuration::{AUTH_CONFIG_FILE_PATH, CONFIG_FILE_PATH};
use crate::context::PokeConfig;

pub fn load_auth_config<'a>() -> Option<AuthConfig<'a>> {
    match config_handler::load_from_file::<AuthConfig>(AUTH_CONFIG_FILE_PATH) {
        Ok(conf) => Some(conf),
        Err(err) => {
            tracing::info!("Failed to load auth config: {}", err);
            None
        }
    }
}

pub fn load_config() -> PokeConfig {
    match config_handler::load_from_file::<PokeConfigLoader>(AUTH_CONFIG_FILE_PATH) {
        Ok(conf) => {
            return PokeConfig {
                missing_pokemon_ball: conf
                    .missing_pokemon_ball
                    .unwrap_or(Cow::Borrowed("ultraball")),
                pokeball_buy_amount: conf.pokeball_buy_amount.unwrap_or(10),
                default_pokemon_ball: conf
                    .default_pokemon_ball
                    .unwrap_or(Cow::Borrowed("pokeball")),
                only_catch_missing: conf.only_catch_missing.unwrap_or(false),
                should_buy_pokeball: conf.should_buy_pokeball.unwrap_or(false),
                skip_catching_pokemon: conf.skip_catching_pokemon.unwrap_or(false),
            };
        }
        Err(err) => {
            tracing::warn!("Failed to load auth config: {}", err);
        }
    };

    let default_config = PokeConfig {
        missing_pokemon_ball: Cow::Borrowed("ultraball"),
        pokeball_buy_amount: 10,
        default_pokemon_ball: Cow::Borrowed("pokeball"),
        only_catch_missing: false,
        should_buy_pokeball: false,
        skip_catching_pokemon: false,
    };

    write_poke_config(&default_config);

    default_config
}

pub fn write_auth_config(token: &UserToken) {
    config_handler::write_auth_config(
        AUTH_CONFIG_FILE_PATH,
        &AuthConfig {
            access_token: Cow::Borrowed(token.access_token.secret()),
            refresh_token: Cow::Borrowed(token.refresh_token.as_ref().unwrap().secret()),
        },
    )
    .expect("Failed to write auth config")
}

pub fn write_poke_config(config: &PokeConfig) {
    config_handler::write_config_file(
        CONFIG_FILE_PATH,
        &PokeConfigLoader {
            missing_pokemon_ball: Some(config.missing_pokemon_ball.clone()),
            default_pokemon_ball: Some(config.default_pokemon_ball.clone()),
            pokeball_buy_amount: Some(config.pokeball_buy_amount),
            only_catch_missing: Some(config.only_catch_missing),
            should_buy_pokeball: Some(config.should_buy_pokeball),
            skip_catching_pokemon: Some(config.skip_catching_pokemon),
        },
    )
    .expect("Failed to write auth config")
}
