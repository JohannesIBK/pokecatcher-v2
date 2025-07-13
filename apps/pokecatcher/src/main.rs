mod configuration;
mod context;
mod message_interface;
mod pokemon;
mod utils;

use std::process::exit;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use auth_handler::get_user_token;
use tokio::sync::mpsc::channel;
use tracing_subscriber::filter::LevelFilter;

use crate::configuration::CLIENT_ID;
use crate::context::{BotContext, SendMessageDto};
use crate::message_interface::handle_message;
use crate::utils::{load_auth_config, load_config, write_auth_config};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let user_token = match get_user_token(CLIENT_ID, load_auth_config()).await {
        Ok(token) => {
            write_auth_config(&token);

            token
        }
        Err(err) => {
            tracing::error!("Failed to get user token: {:?}", err);
            exit(1);
        }
    };

    let pokeconfig = load_config();
    let (message_tx, mut message_rx) = channel::<SendMessageDto>(100);
    let context = Arc::new(BotContext {
        user_login: user_token.login.to_string(),
        pokemon_missing: AtomicBool::new(false),
        sent_pokecatch: AtomicBool::new(false),
        sender_tx: message_tx,
        poke_config: pokeconfig,
    });

    let credentials = tmi::Credentials::new(
        user_token.login.to_string(),
        format!("oauth:{}", user_token.access_token.secret()),
    );

    let mut client = tmi::Client::builder()
        .credentials(credentials)
        .connect()
        .await
        .expect("Failed to connect");

    client
        .join_all(&["migisch"])
        .await
        .expect("Failed to join channels");

    loop {
        tokio::select! {
            msg = message_rx.recv() => {
                let Some(message) = msg else {
                    continue;
                };

                tracing::info!("Sending message {} to channel: {}", message.message, message.channel);
                if let Err(err) = client.privmsg(message.channel.as_str(), &message.message).send().await {
                    tracing::error!("Failed to send message: {:?}", err);
                }
            }
            msg = client.recv() => {
                let Ok(msg) = msg else {
                    tracing::error!("Failed to parse a message");

                    continue;
                };

                if let Err(err) = handle_message(
                    &mut client,
                    msg,
                    &context,
                ).await {
                    tracing::error!("Failed to handle message: {:?}", err);
                }
            }
        }
    }
}
