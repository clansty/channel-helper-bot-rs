use regex::Regex;
use reqwest::Url;
use teloxide_core::{prelude::*, types::*};

enum ChatOrUser {
    Chat(Chat),
    User(User),
}

pub async fn process_message(message: &str, data: Message, bot: Bot) {
    let regex = Regex::new(r"(^/([^a-zA-Z\s¥$]\S*)|^/[$¥](\S+))( (\S+))?").unwrap();
    let Some(exec) = regex.captures(message) else {
        return;
    };
    let Some(action) = exec.get(2).or(exec.get(3)) else {
        return;
    };

    let at_entity = data
        .entities()
        .map_or(None, |it| {
            it.iter()
                .find(|it| matches!(&it.kind, MessageEntityKind::TextMention { .. }))
        })
        .map(|it| match &it.kind {
            MessageEntityKind::TextMention { user } => ChatOrUser::User(user.clone()),
            _ => unreachable!(),
        });
    let Some(to_user) = at_entity
        .or(data
            .reply_to_message()
            .map_or(None, |it| match it.sender_chat() {
                Some(from) => Some(ChatOrUser::Chat(from.clone())),
                None => None,
            }))
        .or(data.reply_to_message().map_or(None, |it| match it.from() {
            Some(from) => Some(ChatOrUser::User(from.clone())),
            None => None,
        }))
        .or(data.sender_chat().map(|it| ChatOrUser::Chat(it.clone())))
        .or(data.from().map(|it| ChatOrUser::User(it.clone())))
    else {
        return;
    };
    let to_user_name = match to_user {
        ChatOrUser::Chat(ref chat) => chat.title().or(chat.first_name()).unwrap_or_default(),
        ChatOrUser::User(ref user) => &user.first_name,
    };
    let Some(from_user) = data
        .sender_chat()
        .map(|it| ChatOrUser::Chat(it.clone()))
        .or(data.from().map(|it| ChatOrUser::User(it.clone())))
    else {
        return;
    };
    let from_user_name = match from_user {
        ChatOrUser::Chat(ref chat) => chat.title().or(chat.first_name()).unwrap_or_default(),
        ChatOrUser::User(ref user) => &user.first_name,
    };

    let mut tg_entities: Vec<MessageEntity> = vec![MessageEntity {
        kind: match from_user {
            ChatOrUser::User(ref user) => MessageEntityKind::TextMention { user: user.clone() },
            ChatOrUser::Chat(ref chat) => MessageEntityKind::TextLink {
                url: Url::parse(&format!(
                    "https://t.me/{}",
                    chat.username().unwrap_or_default()
                ))
                .unwrap(),
            },
        },
        offset: 0,
        length: from_user_name.chars().count(),
    }];
    let mut tg_text = format!(
        "{} {}了 ",
        from_user_name,
        add_space_if_english(action.into())
    );

    tg_entities.push(MessageEntity {
        kind: match to_user {
            ChatOrUser::User(ref user) => MessageEntityKind::TextMention { user: user.clone() },
            ChatOrUser::Chat(ref chat) => MessageEntityKind::TextLink {
                url: Url::parse(&format!(
                    "https://t.me/{}",
                    chat.username().unwrap_or_default()
                ))
                .unwrap(),
            },
        },
        offset: tg_text.chars().count(),
        length: to_user_name.chars().count(),
    });
    tg_text = format!("{}{}", tg_text, to_user_name);

    if let Some(suffix) = exec.get(5) {
        tg_text = format!("{} {}", tg_text, suffix.as_str());
    };

    tg_text = format!("{}！", tg_text);

    let res = bot
        .send_message(data.chat.id, tg_text.clone())
        .reply_to_message_id(data.id)
        .entities(tg_entities.clone())
        .disable_web_page_preview(true)
        .await;
    log::info!(
        "Send message: {} {} {:#?} {:#?}",
        data.chat.id,
        tg_text,
        tg_entities,
        res
    )
}

fn is_chinese(c: char) -> bool {
    // 判断字符是否是中文字符
    c >= '\u{4E00}' && c <= '\u{9FFF}'
}

fn add_space_if_english(s: &str) -> String {
    if let Some(last_char) = s.chars().last() {
        if !is_chinese(last_char) {
            return format!("{} ", s);
        }
    }
    s.to_string()
}
