use dotenv::dotenv;
use std::env;
use teloxide::{types::ChatAdministratorRights, utils::command::BotCommands};

pub fn get_token() -> String {
    dotenv().ok();
    let token = env::var("BOT_TOKEN").expect("BOT_TOKEN not set");
    return token;
}

pub const BOT_DESCRIPTION: &str = "Tristan here!\nAdd me as an administrator in your group!";

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "List of supported commands:")]
pub enum Command {
    #[command(description = "")]
    Start,
    #[command(description = "Info")]
    Help,
}

pub const BOT_RIGHTS: ChatAdministratorRights = ChatAdministratorRights {
    can_delete_messages: true,
    can_invite_users: false,
    can_pin_messages: Some(false),
    can_promote_members: false,
    can_restrict_members: false,
    can_change_info: false,
    can_manage_chat: false,
    can_edit_messages: Some(false),
    can_post_messages: Some(false),
    can_manage_video_chats: false,
    is_anonymous: false,
};
