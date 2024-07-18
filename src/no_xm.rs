use teloxide_core::{prelude::*, types::*};

pub async fn process_update(update: Update, bot: Bot) {
    match update.kind {
        UpdateKind::Message(message) => {
            if message.text().unwrap_or_default() == "羡慕" {
                let _ = bot.delete_message(message.chat.id, message.id).await;
            }
        }
        _ => {}
    }
}
