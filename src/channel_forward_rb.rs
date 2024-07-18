use teloxide_core::{prelude::*, types::*};

const CHANNEL_ID: ChatId = ChatId(-1002104163498);

// 主群
const FORWARD_GROUPS: [ChatId; 1] = [ChatId(-1002125329673)];

pub async fn process_update(update: Update, bot: Bot) {
    match update.kind {
        UpdateKind::ChannelPost(post) => {
            if post.chat.id != CHANNEL_ID {
                return;
            }
            for it in FORWARD_GROUPS {
                let res = bot.forward_message(it, CHANNEL_ID, post.id).await;
                log::info!("forward post: {:#?}", res);
            }
        }
        _ => {}
    }
}
