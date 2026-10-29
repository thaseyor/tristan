use dotenvy::dotenv;
use std::env;
use teloxide::{types::ChatAdministratorRights, utils::command::BotCommands};

pub struct Config {
    pub token: String,
    inline_bots_blacklist: Vec<String>,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        Self {
            token: env::var("BOT_TOKEN").expect("BOT_TOKEN not set"),
            inline_bots_blacklist: parse_usernames(
                &env::var("INLINE_BOTS_BLACKLIST").unwrap_or_default(),
            ),
        }
    }

    pub fn is_blacklisted_bot(&self, username: &str) -> bool {
        self.inline_bots_blacklist
            .iter()
            .any(|b| b.eq_ignore_ascii_case(username))
    }
}

fn parse_usernames(list: &str) -> Vec<String> {
    list.split(',')
        .map(|u| u.trim().trim_start_matches('@'))
        .filter(|u| !u.is_empty())
        .map(String::from)
        .collect()
}

pub const BOT_DESCRIPTION: &str = "Tristan here!\nAdd me as an administrator in your group!";

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "List of supported commands:")]
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
    can_post_stories: Some(false),
    can_edit_stories: Some(false),
    can_delete_stories: Some(false),
    can_manage_topics: Some(false),
    is_anonymous: false,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_comma_separated_usernames() {
        assert_eq!(parse_usernames(" SpamBot, @gif ,,"), ["SpamBot", "gif"]);
        assert!(parse_usernames("").is_empty());
    }
}
