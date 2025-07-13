mod utils;

use std::borrow::Cow;
use std::sync::Arc;
use std::sync::atomic::Ordering;

use anyhow::Result;
use tmi::Privmsg;

use self::utils::{
    check_for_pokemon, get_pokeball, has_no_pokeball, is_pokemon_missing, purchase_successful,
    starts_with_mention,
};
use crate::context::{BotContext, SendMessageDto};

pub async fn handle_pokemon_message(message: Privmsg<'_>, context: Arc<BotContext>) -> Result<()> {
    // Ignore when the message is not from the poke bot
    if message.sender().id() != "519435394" {
        return Ok(());
    }

    // When a message contains !pokecatch, the bot has spawned a new pokemon
    if message.text().contains("!pokecatch") {
        tokio::spawn(check_for_pokemon(
            message.channel().to_string(),
            context.clone(),
        ));

        return Ok(());
    }

    if !starts_with_mention(message.text(), context.user_login.as_str()) {
        return Ok(());
    }

    if is_pokemon_missing(message.text()) {
        context.pokemon_missing.store(true, Ordering::SeqCst);

        if context.poke_config.skip_catching_pokemon {
            tracing::debug!("Skip catching pokemon is enabled, skipping");

            return Ok(());
        }

        context.sent_pokecatch.store(true, Ordering::SeqCst);

        context
            .sender_tx
            .send(SendMessageDto {
                channel: message.channel().to_string(),
                message: Cow::Owned(format!(
                    "!pokecatch {}",
                    context.poke_config.missing_pokemon_ball
                )),
            })
            .await?;
    }

    if context.poke_config.should_buy_pokeball {
        if has_no_pokeball(message.text()) {
            let pokeball = get_pokeball(&context);

            context
                .sender_tx
                .send(SendMessageDto {
                    channel: message.channel().to_string(),
                    message: Cow::Owned(format!(
                        "!pokeshop {} {}",
                        pokeball, context.poke_config.pokeball_buy_amount,
                    )),
                })
                .await?;

            return Ok(());
        }

        if purchase_successful(message.text()) {
            context.sent_pokecatch.store(true, Ordering::SeqCst);

            let pokeball = get_pokeball(&context);

            context
                .sender_tx
                .send(SendMessageDto {
                    channel: message.channel().to_string(),
                    message: Cow::Owned(format!("!pokecatch {pokeball}")),
                })
                .await?;
        }
    }

    Ok(())
}
