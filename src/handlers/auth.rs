/*
:project: telegram-onedrive
:author: L-ING
:copyright: (C) 2024 L-ING <hlf01@icloud.com>
:license: MIT, see LICENSE for more details.
*/

use crate::{auth_server, message::TelegramMessage, state::AppState};
use anyhow::{Context, Result};
use proc_macros::{check_in_group, check_senders};
use tokio::sync::mpsc::Receiver;

pub const PATTERN: &str = "/auth";

#[check_senders]
#[check_in_group]
pub async fn handler(message: TelegramMessage, state: AppState) -> Result<()> {
    let (tx_tg, tx_od, rx_tg, rx_od, _server_abort_handle) = auth_server::spawn().await?;

    {
        *state.auth_tx_tg.lock().await = Some(tx_tg);
        *state.auth_tx_od.lock().await = Some(tx_od);
    }

    let mut messages_to_delete = Vec::new();

    login_to_telegram(message.clone(), state.clone(), rx_tg, &mut messages_to_delete).await?;

    // Wait 10 seconds after Telegram login success, then delete telegram auth messages
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    for msg_id in messages_to_delete.drain(..) {
        state.telegram_bot.delete_messages(message.chat(), &[msg_id]).await.ok();
    }

    authorize_onedrive(message.clone(), state.clone(), false, rx_od, &mut messages_to_delete).await?;

    let onedrive = &state.onedrive;
    onedrive.set_current_user().await?;

    // Delete auth process messages immediately
    for msg_id in messages_to_delete.drain(..) {
        state.telegram_bot.delete_messages(message.chat(), &[msg_id]).await.ok();
    }

    // Success response
    let response = "OneDrive 授权成功！";
    let success_msg = message.respond(response).await.context(response)?;

    {
        *state.auth_tx_tg.lock().await = None;
        *state.auth_tx_od.lock().await = None;
    }

    // Wait 10 seconds before cleaning up the final success message and the command
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    
    state.telegram_bot.delete_messages(message.chat(), &[success_msg.id()]).await.ok();
    message.delete().await.ok();

    Ok(())
}

pub async fn login_to_telegram(
    message: TelegramMessage,
    state: AppState,
    rx: Receiver<String>,
    messages_to_delete: &mut Vec<i32>,
) -> Result<()> {
    let telegram_user = &state.telegram_user;

    telegram_user.login(message.clone(), rx, Some(messages_to_delete)).await?;

    let response = "登录 Telegram 成功！";
    let msg = message.respond(response).await.context(response)?;
    messages_to_delete.push(msg.id());

    Ok(())
}

pub async fn authorize_onedrive(
    message: TelegramMessage,
    state: AppState,
    should_add: bool,
    rx: Receiver<String>,
    messages_to_delete: &mut Vec<i32>,
) -> Result<()> {
    let onedrive = &state.onedrive;

    onedrive.login(message.clone(), should_add, rx, Some(messages_to_delete)).await?;

    Ok(())
}
