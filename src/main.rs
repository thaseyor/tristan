mod config;
mod service;

use config::Config;
use log::{error, info};
use std::sync::Arc;
use teloxide::{prelude::*, types::MessageKind};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    info!("Starting tristan bot");

    let config = Arc::new(Config::from_env());
    let bot = Bot::new(&config.token);

    let res = service::set_default_rights(&bot).await;

    if let Err(e) = res {
        error!("{:?}", e);
    }

    Dispatcher::builder(bot, Update::filter_message().endpoint(handle_message))
        .dependencies(dptree::deps![config])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn handle_message(bot: Bot, m: Message, config: Arc<Config>) -> ResponseResult<()> {
    let is_private_chat = m.chat.is_private();
    let chat_id = m.chat.id;

    if is_private_chat {
        let text = m.text().unwrap_or("/start");
        service::send_response(bot, chat_id, text).await?;
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

    // check if bot left the group
    let leaved = m.left_chat_member();
    if let Some(leaved) = leaved {
        let me = bot.get_me().await?;

        if leaved.id == me.id {
            return respond(());
        }
    }

    let res = service::delete_system_message(bot, chat_id, chat_title, message_id).await;

    if let Err(e) = res {
        error!("{:?}", e);
    }

    respond(())
}
