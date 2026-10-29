use std::{
    collections::HashMap,
    sync::Mutex,
    time::{Duration, Instant},
};
use teloxide::types::ChatId;

pub struct Reminders {
    interval: Duration,
    last_sent: Mutex<HashMap<ChatId, Instant>>,
}

impl Reminders {
    pub fn new(interval: Duration) -> Self {
        Self {
            interval,
            last_sent: Mutex::new(HashMap::new()),
        }
    }

    pub fn is_due(&self, chat_id: ChatId) -> bool {
        let now = Instant::now();
        let mut last_sent = self.last_sent.lock().unwrap();
        last_sent.retain(|_, sent| now.duration_since(*sent) < self.interval);

        if last_sent.contains_key(&chat_id) {
            return false;
        }

        last_sent.insert(chat_id, now);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_one_reminder_per_chat_per_interval() {
        let reminders = Reminders::new(Duration::from_secs(60));

        assert!(reminders.is_due(ChatId(1)));
        assert!(!reminders.is_due(ChatId(1)));
        assert!(reminders.is_due(ChatId(2)));
    }

    #[test]
    fn allows_reminder_after_interval() {
        let reminders = Reminders::new(Duration::ZERO);

        assert!(reminders.is_due(ChatId(1)));
        assert!(reminders.is_due(ChatId(1)));
    }
}
