mod config;
mod reminders;
mod service;

use config::Config;
use log::{error, info};
use reminders::Reminders;
use std::{sync::Arc, time::Duration};
use teloxide::{
    prelude::*,
    types::{Me, MessageKind},
};

const OWNER_REMINDER_INTERVAL: Duration = Duration::from_secs(24 * 60 * 60);

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    info!("Starting tristan bot");

    let config = Arc::new(Config::from_env());
    let bot = Bot::new(&config.token);

    if let Err(e) = service::set_default_rights(&bot).await {
        error!("{:?}", e);
    }

    let me = bot.get_me().await.expect("Failed to fetch bot info");
    let reminders = Arc::new(Reminders::new(OWNER_REMINDER_INTERVAL));

    Dispatcher::builder(bot, Update::filter_message().endpoint(handle_message))
        .dependencies(dptree::deps![config, me, reminders])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn handle_message(
    bot: Bot,
    m: Message,
    me: Me,
    config: Arc<Config>,
    reminders: Arc<Reminders>,
) -> ResponseResult<()> {
    let is_private_chat = m.chat.is_private();
    let chat_id = m.chat.id;

    if is_private_chat {
        let text = m.text().unwrap_or("/start");
        service::send_response(&bot, &me, chat_id, text).await?;
        return respond(());
    }

    let chat_title = m.chat.title().unwrap_or("your group");
    let message_id = m.id;

    // check if system message
    let is_system_message = matches!(
        m.kind,
        MessageKind::Dice(_)
            | MessageKind::LeftChatMember(_)
            | MessageKind::NewChatMembers(_)
            | MessageKind::NewChatTitle(_)
    );

    let is_forbidden_bot = m
        .via_bot
        .as_ref()
        .and_then(|b| b.username.as_deref())
        .is_some_and(|u| config.is_blacklisted_bot(u));

    if !(is_system_message || is_forbidden_bot) {
        return respond(());
    }

    if m.left_chat_member().is_some_and(|u| u.id == me.id) {
        return respond(());
    }

    if let Err(e) =
        service::delete_system_message(&bot, &me, &reminders, chat_id, chat_title, message_id).await
    {
        error!("{:?}", e);
    }

    respond(())
}
