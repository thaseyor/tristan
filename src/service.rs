use crate::config::{self, Command};
use std::error::Error;
use teloxide::{prelude::*, types::MessageId, utils::command::BotCommands, RequestError};

pub async fn set_default_rights(bot: &Bot) -> Result<(), RequestError> {
    bot.set_my_default_administrator_rights()
        .rights(config::BOT_RIGHTS)
        .send()
        .await?;

    Ok(())
}

pub async fn delete_system_message(
    bot: Bot,
    chat_id: ChatId,
    chat_title: &str,
    message_id: MessageId,
) -> Result<(), Box<dyn Error>> {
    let admins = bot.get_chat_administrators(chat_id).send().await?;

    if admins.is_empty() {
        return Err("No admins in chat".into());
    }

    let my_id = bot.get_me().await?.id;
    let owner = admins.iter().find(|a| a.is_owner());

    let me_as_admin = admins.iter().find(|a| a.user.id == my_id);

    match me_as_admin {
        Some(me_as_admin) if me_as_admin.can_delete_messages() => {
            bot.delete_message(chat_id, message_id).await?;
        }
        Some(_) => {
            if owner.is_none() {
                return Err("No owner in chat".into());
            }

            let owner_id = owner.unwrap().user.id;

            bot.send_message(
                owner_id,
                format!(
                    "Give me permission to delete messages pls in {}",
                    chat_title
                ),
            )
            .await?;
        }
        None => {
            if owner.is_none() {
                return Err("No owner in chat".into());
            }

            let owner_id = owner.unwrap().user.id;

            bot.send_message(
                owner_id,
                format!("Promote me to admin pls in {}", chat_title),
            )
            .await?;
        }
    }

    Ok(())
}

pub async fn send_response(bot: Bot, chat_id: ChatId, text: &str) -> Result<(), RequestError> {
    let me = bot.get_me().await?;
    let username = me.user.username.expect("Bots must have usernames");

    let response = get_command_response(text, &username);
    bot.send_message(chat_id, response).await?;

    Ok(())
}

fn get_command_response(text: &str, username: &str) -> String {
    match Command::parse(text, username) {
        Ok(Command::Help) => Command::descriptions().to_string(),
        Ok(Command::Start) | Err(_) => config::BOT_DESCRIPTION.to_string(),
    }
}
