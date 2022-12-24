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

        if is_private_chat {
            let text = m.text();
            if let Some(text) = text {
                let chat_id = m.chat.id;
                let res = service::send_response(bot, chat_id, text).await;

                if let Err(e) = res {
                    error!("{:?}", e);
                }
            };
        } else {
            let chat_id = m.chat.id;
            let chat_title = m.chat.title().unwrap_or_else(|| "your group");
            let message_id = m.id;

            let game = m.dice();
            if let Some(_) = game {
                let res = bot.delete_message(chat_id, message_id).await;

                if let Err(e) = res {
                    error!("{:?}", e);
                }
                return respond(());
            }

            let leaved = m.left_chat_member();
            let joined = m.new_chat_members();

            // check if smbd joined or leaved
            if joined.is_some() || leaved.is_some() {
                // check if bot is the one who left the group
                if let Some(leaved) = leaved {
                    let me = bot.get_me().await;
                    match me {
                        Ok(me) => {
                            if leaved.id == me.id {
                                return respond(());
                            }
                        }
                        Err(e) => error!("{:?}", e),
                    }
                }

                let res =
                    service::delete_system_message(bot, chat_id, chat_title, message_id).await;

                if let Err(e) = res {
                    error!("{:?}", e);
                }
            }
        }

        respond(())
    })
    .await;
}
