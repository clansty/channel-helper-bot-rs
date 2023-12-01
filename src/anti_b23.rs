use regex::Regex;
use teloxide_core::{prelude::*, types::*};

async fn replace_urls(text: &str) -> Option<String> {
    let re = Regex::new(r"https?://b23\.tv/([^\s/$.?#].[^\s]*)").unwrap();
    let mut new_text = text.to_owned();

    for mat in re.captures_iter(text) {
        log::info!("找到URL: {:?}", mat.get(0));
        if mat.get(1).unwrap().as_str().starts_with("BV") {
            continue;
        }
        let new_url = match b23_decode(mat.get(0).unwrap().into()).await {
            Ok(url) => url,
            Err(err) => {
                log::error!("{:#?}", err);
                continue;
            }
        };
        new_text = new_text.replace(mat.get(0).unwrap().as_str(), &new_url);
    }

    if new_text == text {
        None
    } else {
        Some(new_text)
    }
}

#[cfg(test)]
#[tokio::test]
async fn test_replace_urls() {
    let result = replace_urls("猫").await;
    assert_eq!(result, None);

    // 直接是 BV 号的不替换
    let result = replace_urls("猫 https://b23.tv/BV1oH4y1o77U").await;
    assert_eq!(result, None);

    // 不解析其他链接
    let result = replace_urls("猫 https://google.com").await;
    assert_eq!(result, None);

    // 多个链接全都替换
    let result = replace_urls("猫 https://b23.tv/YAoPMfK https://b23.tv/pfkF1WI").await;
    assert_eq!(result.unwrap(), "猫 https://www.bilibili.com/video/BV1me411R7Yx?p=1 https://www.bilibili.com/video/BV1S94y187PD?p=1");

    //https://t.me/SharkCat_Channel/5435
    let result = replace_urls(
        r#"【F1冠军车手基米·莱科宁 | 开启中国行-哔哩哔哩】 https://b23.tv/YAoPMfK

    来北京？！！这得想办法看看了"#,
    )
    .await;
    assert_eq!(
        result.unwrap(),
        r#"【F1冠军车手基米·莱科宁 | 开启中国行-哔哩哔哩】 https://www.bilibili.com/video/BV1me411R7Yx?p=1

    来北京？！！这得想办法看看了"#
    );

    //https://t.me/c/1731059168/21555
    let result = replace_urls(r#"【Tora：微软、清华联合发布最强开源数学大模型，在竞赛数学MATH上准确率首次突破50%，采用工具集成+大模型微调出数学推理语言模型-哔哩哔哩】 https://b23.tv/pfkF1WI"#).await;
    assert_eq!(
        result.unwrap(),
        r#"【Tora：微软、清华联合发布最强开源数学大模型，在竞赛数学MATH上准确率首次突破50%，采用工具集成+大模型微调出数学推理语言模型-哔哩哔哩】 https://www.bilibili.com/video/BV1S94y187PD?p=1"#
    );
}

async fn b23_decode(b23_url: &str) -> Result<String, reqwest::Error> {
    Ok(
        reqwest::get(format!("https://b23.wtf/api?full={}&status=200", b23_url))
            .await?
            .text()
            .await?
            .trim()
            .to_owned(),
    )
}

#[cfg(test)]
#[tokio::test]
async fn test_b23_decode() {
    let result = b23_decode("https://b23.tv/efRuQLF").await;
    assert_eq!(
        result.unwrap(),
        "https://www.bilibili.com/video/BV1Fy4y1N7Hi?p=1"
    );
}

pub async fn process_message(message: &str, data: Message, bot: Bot) {
    if let Some(result) = replace_urls(message).await {
        let res = bot.delete_message(data.chat.id, data.id).await;
        log::info!("Delete message: {} {} {:#?}", data.chat.id, data.id, res);
        let mut req = bot.send_message(
            data.chat.id,
            format!(
                "{}:\n{}\n\n请勿使用带跟踪的 b23.tv 短链接",
                data.from()
                    .map(|it| it.first_name.clone())
                    .unwrap_or("未知用户".to_owned()),
                result
            ),
        );
        req.reply_to_message_id = data.reply_to_message().map(|it| it.id);
        let res = req.await;
        log::info!("Send message: {} {:#?}", data.chat.id, res);
    }
}
