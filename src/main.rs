use anyhow::{Context, Result as AnyResult};
use dotenv::dotenv;
use std::env;
use teloxide::prelude::*;
use teloxide::types::UpdateKind;

// Module structure
mod converter;
mod handlers;
mod telegram;

use handlers::process_webm;

#[tokio::main]
async fn main() -> AnyResult<()> {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting bot");

    let bot = Bot::from_env();

    if serverless_mode_enabled() {
        run_serverless_once(bot).await?;
        return Ok(());
    }

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        if let Err(e) = process_webm(&bot, &msg).await {
            log::error!("Error processing webm file: {:?}", e);
        }
        respond(())
    })
    .await;
    Ok(())
}

fn serverless_mode_enabled() -> bool {
    env::var("SERVERLESS_RUN")
        .map(|value| {
            matches!(
                value.to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false)
}

fn read_update_offset() -> Option<i64> {
    env::var("TELEGRAM_UPDATE_OFFSET")
        .ok()
        .and_then(|value| value.parse::<i64>().ok())
}

async fn run_serverless_once(bot: Bot) -> AnyResult<()> {
    let mut request = bot.get_updates();
    if let Some(offset) = read_update_offset() {
        let next_offset: i32 = (offset + 1)
            .try_into()
            .context("Telegram update offset exceeded i32 range")?;
        request = request.offset(next_offset);
    }

    let updates = request.send().await?;
    let mut last_processed_id: Option<i64> = None;

    for update in updates {
        let current_id = i64::from(update.id);
        last_processed_id =
            Some(last_processed_id.map_or(current_id, |existing| existing.max(current_id)));

        if let UpdateKind::Message(message) = update.kind
            && let Err(error) = process_webm(&bot, &message).await
        {
            log::error!("Error processing update {}: {:?}", current_id, error);
        }
    }

    if let Some(id) = last_processed_id {
        println!("LAST_UPDATE_ID={}", id);
    }

    Ok(())
}
