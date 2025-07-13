use std::sync::Arc;

use anyhow::{Context, Result};
use tmi::{IrcMessage, Message};

use crate::context::BotContext;
use crate::message_interface::self_destruction::should_self_destruct;
use crate::pokemon::handle_pokemon_message;

pub async fn handle_message(
    client: &mut tmi::Client,
    msg: IrcMessage,
    context: &Arc<BotContext>,
) -> Result<()> {
    match msg.as_typed().context("Failed to parse twitch message")? {
        Message::Join(message) if message.user() == context.user_login => {
            tracing::info!("Joined channel {}", message.channel());

            Ok(())
        }
        Message::Reconnect => {
            client
                .reconnect()
                .await
                .context("Failed to reconnect to twitch irc")?;
            client
                .join_all(&[context.poke_config.channel.as_str()])
                .await
                .context("Failed to rejoin channels after reconnect")
        }
        Message::Ping(ping) => client.pong(&ping).await.context("Failed to pong irc"),
        Message::Privmsg(privmsg) => {
            should_self_destruct(&privmsg, context);
            handle_pokemon_message(privmsg, context).await
        }
        _ => Ok(()),
    }
}
