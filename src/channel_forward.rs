use teloxide_core::{prelude::*, types::*};

const CHANNEL_ID: ChatId = ChatId(-1001311872089);
// 频道绑定群
const CHANNEL_GROUP: ChatId = ChatId(-1001570232723);
// 主群
const FORWARD_GROUPS: [ChatId; 1] = [ChatId(-1001531134777)];
const CHANNEL_BACKUP: ChatId = ChatId(-1001290369619);

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
        UpdateKind::Message(message) => {
            if message.chat.id != CHANNEL_GROUP {
                return;
            }
            let res = bot
                .forward_message(CHANNEL_BACKUP, CHANNEL_GROUP, message.id)
                .await;
            log::info!("forward backup: {:#?}", res);
        }
        _ => {}
    }
}
