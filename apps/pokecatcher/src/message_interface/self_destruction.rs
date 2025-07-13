use tmi::{Badge, Privmsg};

use crate::context::BotContext;

pub fn should_self_destruct(message: &Privmsg<'_>, context: &BotContext) {
    if message.sender().login() == context.user_login && message.text() == "!stopme" {
        tracing::warn!("Received stop command, self-destructing");
        std::process::exit(0);
    }

    if message.badges().any(|b| b == &Badge::Broadcaster)
        && message
            .text()
            .strip_prefix("!stopme @")
            .is_some_and(|user| user == context.user_login)
    {
        tracing::warn!("Broadcaster sent stop command, self-destructing");
        std::process::exit(0);
    }
}
