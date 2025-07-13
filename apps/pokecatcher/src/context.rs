use std::borrow::Cow;
use std::sync::atomic::AtomicBool;

use tokio::sync::mpsc::Sender;

#[derive(Debug)]
pub struct BotContext {
    pub sender_tx: Sender<SendMessageDto>,
    pub pokemon_missing: AtomicBool,
    pub sent_pokecatch: AtomicBool,
    pub user_login: String,
    pub poke_config: PokeConfig,
}

pub struct SendMessageDto {
    pub message: Cow<'static, str>,
    pub channel: String,
}

impl SendMessageDto {
    pub fn pokecheck(channel: String) -> Self {
        Self {
            message: Cow::Borrowed("!pokecheck"),
            channel,
        }
    }
}

#[derive(Debug)]
pub struct PokeConfig {
    pub missing_pokemon_ball: Cow<'static, str>,
    pub default_pokemon_ball: Cow<'static, str>,
    pub only_catch_missing: bool,
    pub skip_catching_pokemon: bool,
    pub should_buy_pokeball: bool,
    pub pokeball_buy_amount: u8,
    pub stop_on_no_money: bool,
}
