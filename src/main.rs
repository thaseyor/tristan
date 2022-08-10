mod config;

use config::Command;
use teloxide::{prelude::*, utils::command::BotCommands};
extern crate pretty_env_logger;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    info!("Starting tristan bot");

    let token = config::get_token();
    let bot = Bot::new(token).auto_send();

    let res = bot
        .set_my_default_administrator_rights()
        .rights(config::BOT_RIGHTS)
        .send()
        .await;

    if let Err(e) = res {
        error!("{:?}", e);
    }

    teloxide::repl(bot, |m: Message, bot: AutoSend<Bot>| async move {
        if m.chat.is_private() {
            let text = m.text();

            match text {
                None => {}
                Some(text) => {
                    let me = bot.get_me().await?;

                    let res = Command::parse(
                        text,
                        me.user
                            .username
                            .as_deref()
                            .expect("Bots must have usernames"),
                    );

                    let response = match res {
                        Ok(command) => get_command_response(command),
                        Err(_e) => config::BOT_DESCRIPTION.to_string(),
                    };

                    bot.send_message(m.chat.id, response).await?;
                }
            }
        } else {
            let leaved = m.left_chat_member();
            let joined = m.new_chat_members();

            if joined != None || leaved != None {
                let admins = bot.get_chat_administrators(m.chat.id).send().await?;

                let my_id = bot.get_me().await?.id;
                let owner_id = admins.iter().find(|a| a.kind.is_owner()).unwrap().user.id;
                let chat_title = m.chat.title().unwrap_or("your group");

                let me_as_admin = admins.iter().find(|a| a.user.id == my_id);

                match me_as_admin {
                    Some(me_as_admin) => {
                        if me_as_admin.can_delete_messages() {
                            bot.delete_message(m.chat.id, m.id).await?;
                        } else {
                            bot.send_message(
                                owner_id,
                                format!(
                                    "Give me permission to delete messages pls in {}",
                                    chat_title
                                ),
                            )
                            .await?;
                        }
                    }
                    None => {
                        bot.send_message(
                            owner_id,
                            format!("Promote me to admin pls in {}", chat_title),
                        )
                        .await?;
                    }
                }
            }
        }

        respond(())
    })
    .await;
}

fn get_command_response(command: Command) -> String {
    match command {
        Command::Start => config::BOT_DESCRIPTION.to_string(),
        Command::Help => Command::descriptions().to_string(),
    }
}
