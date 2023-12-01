use regex::Regex;
use teloxide_core::{prelude::*, types::*};

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
            MessageEntityKind::TextMention { user } => user,
            _ => unreachable!(),
        });
    let Some(to_user) = at_entity
        .or(data.reply_to_message().map_or(None, |it| it.from()))
        .or(data.from())
    else {
        return;
    };

    let mut tg_entities: Vec<MessageEntity> = vec![MessageEntity {
        kind: MessageEntityKind::TextMention {
            user: data.from().unwrap().clone(),
        },
        offset: 0,
        length: data.from().unwrap().first_name.chars().count(),
    }];
    let mut tg_text = format!(
        "{} {}了 ",
        data.from().unwrap().first_name,
        add_space_if_english(action.into())
    );

    tg_entities.push(MessageEntity {
        kind: MessageEntityKind::TextMention {
            user: to_user.clone(),
        },
        offset: tg_text.chars().count(),
        length: to_user.first_name.chars().count(),
    });
    tg_text = format!("{}{}", tg_text, to_user.first_name);

    if let Some(suffix) = exec.get(5) {
        tg_text = format!("{} {}", tg_text, suffix.as_str());
    };

    tg_text = format!("{}！", tg_text);

    let res = bot
        .send_message(data.chat.id, tg_text.clone())
        .reply_to_message_id(data.id)
        .entities(tg_entities.clone())
        .await;
    log::info!("Send message: {} {} {:#?} {:#?}", data.chat.id, tg_text, tg_entities, res)
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
