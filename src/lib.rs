mod anti_b23;
mod channel_forward;
mod channel_forward_rb;
mod have_or_not;
mod hug;
mod is_or;
mod o;
mod is_or_not;
mod good;
mod no_xm;

use log::Level;
use teloxide_core::{prelude::*, types::*};
use worker::{Request, *};

#[event(fetch)]
async fn main(mut req: Request, env: Env, _ctx: Context) -> Result<Response> {
    worker_logger::init_with_level(&Level::Debug);

    assert_eq!(
        env.secret("BOT_SECRET").unwrap().to_string(),
        req.headers()
            .get("X-Telegram-Bot-Api-Secret-Token")
            .unwrap()
            .unwrap_or_default()
    ); // let it panic

    let req_body = &req.text().await.expect("无法获取请求体");
    let update = serde_json::from_str::<Update>(req_body).expect("无法解析 Update");
    log::debug!("update: {:#?}", update);
    let bot = Bot::new(env.secret("BOT_TOKEN").unwrap().to_string());

    channel_forward::process_update(update.clone(), bot.clone()).await;
    channel_forward_rb::process_update(update.clone(), bot.clone()).await;
    no_xm::process_update(update.clone(), bot.clone()).await;

    match update.kind {
        UpdateKind::Message(message) => {
            let text = message.text().unwrap_or_default();
            hug::process_message(text, message.clone(), bot.clone()).await;
            anti_b23::process_message(text, message.clone(), bot.clone()).await;
            let message_to_send: Option<&str> =
                vec![o::o, have_or_not::process_message, is_or_not::process_message, good::process_message]
                    .iter()
                    .map(|f| f(text))
                    .find(Option::is_some)
                    .flatten();

            if let Some(message_to_send) = message_to_send {
                let res = bot
                    .send_message(message.chat.id, message_to_send)
                    .reply_to_message_id(message.id)
                    .allow_sending_without_reply(true)
                    .await;
                log::info!(
                    "Send message: {} {} {:#?}",
                    message.chat.id,
                    message_to_send,
                    res
                )
            }
        }
        _ => {}
    };

    Response::ok("ok")
}
