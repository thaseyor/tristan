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
        match m.kind {
            teloxide::types::MessageKind::Dice(_) => {}
            teloxide::types::MessageKind::LeftChatMember(_) => {}
            teloxide::types::MessageKind::NewChatMembers(_) => {}
            teloxide::types::MessageKind::NewChatTitle(_) => {}
            _ => return respond(()),
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
