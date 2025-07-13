use std::borrow::Cow;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PokeConfigLoader {
    pub missing_pokemon_ball: Option<Cow<'static, str>>,
    pub default_pokemon_ball: Option<Cow<'static, str>>,
    pub only_catch_missing: Option<bool>,
    pub skip_catching_pokemon: Option<bool>,
    pub should_buy_pokeball: Option<bool>,
    pub pokeball_buy_amount: Option<u8>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AuthConfig<'a> {
    pub access_token: Cow<'a, str>,
    pub refresh_token: Cow<'a, str>,
}
