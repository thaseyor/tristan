use crate::{
    config::{self, Command},
    reminders::Reminders,
};
use std::error::Error;
use teloxide::{
    prelude::*,
    types::{Me, MessageId},
    utils::command::BotCommands,
    ApiError, RequestError,
};

pub async fn set_default_rights(bot: &Bot) -> Result<(), RequestError> {
    bot.set_my_default_administrator_rights()
        .rights(config::BOT_RIGHTS)
        .await?;

    Ok(())
}

pub async fn delete_system_message(
    bot: &Bot,
    me: &Me,
    reminders: &Reminders,
    chat_id: ChatId,
    chat_title: &str,
    message_id: MessageId,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match bot.delete_message(chat_id, message_id).await {
        Ok(_) => Ok(()),
        Err(RequestError::Api(ApiError::MessageCantBeDeleted)) if reminders.is_due(chat_id) => {
            ask_owner_for_rights(bot, me, chat_id, chat_title).await
        }
        Err(RequestError::Api(ApiError::MessageCantBeDeleted)) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

async fn ask_owner_for_rights(
    bot: &Bot,
    me: &Me,
    chat_id: ChatId,
    chat_title: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let admins = bot.get_chat_administrators(chat_id).await?;
    let owner = admins
        .iter()
        .find(|a| a.is_owner())
        .ok_or("No owner in chat")?;

    let text = match admins.iter().find(|a| a.user.id == me.id) {
        Some(me_as_admin) if me_as_admin.can_delete_messages() => {
            return Err(
                format!("Can't delete message in {chat_title} despite having rights").into(),
            );
        }
        Some(_) => format!("Give me permission to delete messages pls in {chat_title}"),
        None => format!("Promote me to admin pls in {chat_title}"),
    };

    bot.send_message(owner.user.id, text).await?;

    Ok(())
}

pub async fn send_response(
    bot: &Bot,
    me: &Me,
    chat_id: ChatId,
    text: &str,
) -> Result<(), RequestError> {
    let response = get_command_response(text, me.username());
    bot.send_message(chat_id, response).await?;

    Ok(())
}

fn get_command_response(text: &str, username: &str) -> String {
    match Command::parse(text, username) {
        Ok(Command::Help) => Command::descriptions().to_string(),
        Ok(Command::Start) | Err(_) => config::BOT_DESCRIPTION.to_string(),
    }
}
