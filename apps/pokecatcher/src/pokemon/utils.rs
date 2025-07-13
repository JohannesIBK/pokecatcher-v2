use std::borrow::Cow;
use std::sync::Arc;
use std::sync::atomic::Ordering;

use crate::context::{BotContext, SendMessageDto};

pub async fn check_for_pokemon(channel: String, context: Arc<BotContext>) -> anyhow::Result<()> {
    context.pokemon_missing.store(false, Ordering::SeqCst);
    context.sent_pokecatch.store(false, Ordering::SeqCst);

    // Send the !pokecheck message
    context
        .sender_tx
        .send(SendMessageDto::pokecheck(channel.clone()))
        .await?;

    if context.poke_config.skip_catching_pokemon {
        tracing::debug!("Skip catching pokemon is enabled, skipping");

        return Ok(());
    }

    // Wait 3 seconds if a missing message from the pokebot has been received
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    // Check whether the pokecatch message has already been sent
    if context.sent_pokecatch.load(Ordering::SeqCst) {
        tracing::debug!("Pokemon already caught, skipping");
        return Ok(());
    }

    if context.poke_config.only_catch_missing && context.pokemon_missing.load(Ordering::SeqCst) {
        tracing::debug!("Pokemon already in collection, skipping");

        return Ok(());
    };

    context.sent_pokecatch.store(true, Ordering::SeqCst);

    context
        .sender_tx
        .send(SendMessageDto {
            channel,
            message: Cow::Owned(format!("!pokecatch {}", get_pokeball(&context))),
        })
        .await?;

    Ok(())
}

pub fn is_pokemon_missing(msg: &str) -> bool {
    msg.ends_with('❌')
}

pub fn has_no_pokeball(msg: &str) -> bool {
    // Ignore the mention and check whether the user has no pokeballs
    msg.split_once(' ').is_some_and(|(_, message)| {
        message.starts_with("Du besitzt diesen Ball nicht.")
            || message.starts_with("You don’t own that ball.")
    })
}

pub fn starts_with_mention(message: &str, username: &str) -> bool {
    let expected_len = username.len() + 1; // +1 for '@'

    if message.len() < expected_len {
        return false;
    }

    message.as_bytes()[0] == b'@' && message[1..expected_len].to_lowercase() == username
}

pub fn get_pokeball(context: &BotContext) -> &str {
    if context.pokemon_missing.load(Ordering::SeqCst) {
        &context.poke_config.missing_pokemon_ball
    } else {
        &context.poke_config.default_pokemon_ball
    }
}

pub fn purchase_successful(msg: &str) -> bool {
    // Ignore user mention and check whether the purchase was successful
    msg.split_once(' ').is_some_and(|(_, message)| {
        message == "Purchase successful!" || message == "Kauf erfolgreich!"
    })
}
