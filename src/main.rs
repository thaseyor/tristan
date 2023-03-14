mod config;
mod service;

use teloxide::prelude::*;
extern crate pretty_env_logger;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    info!("Starting tristan bot");

    let token = config::get_token();
    let bot = Bot::new(token).auto_send();

    let res = service::set_default_rights(&bot).await;

    if let Err(e) = res {
        error!("{:?}", e);
    }

    teloxide::repl(bot, |m: Message, bot: AutoSend<Bot>| async move {
        let is_private_chat = m.chat.is_private();
        let chat_id = m.chat.id;

        if is_private_chat {
            let text = m.text().unwrap_or_else(|| "/start");
            service::send_response(bot, chat_id, text).await?;
            return respond(());
        }

        let chat_title = m.chat.title().unwrap_or_else(|| "your group");
        let message_id = m.id;

        // check if system message
        let is_system_message = match m.kind {
            teloxide::types::MessageKind::Dice(_) => true,
            teloxide::types::MessageKind::LeftChatMember(_) => true,
            teloxide::types::MessageKind::NewChatMembers(_) => true,
            teloxide::types::MessageKind::NewChatTitle(_) => true,
            _ => false,
        };

        let is_forbidden_bot = m.via_bot.is_some()
            && config::INLINE_BOTS_BLACKLIST.contains(
                &&m.via_bot
                    .clone()
                    .unwrap()
                    .username
                    .unwrap_or_else(|| "".to_string())[..],
            );

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
    })
    .await;
}
