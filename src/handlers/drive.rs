/*
:project: telegram-onedrive
:author: L-ING
:copyright: (C) 2024 L-ING <hlf01@icloud.com>
:license: MIT, see LICENSE for more details.
*/

use super::{
    docs::{format_help, format_unknown_command_help},
    utils::text::cmd_parser,
};
use crate::{
    auth_server, client::OneDriveClient, handlers::auth::authorize_onedrive,
    message::TelegramMessage, state::AppState,
};
use anyhow::{anyhow, Context, Result};
use grammers_client::InputMessage;
use proc_macros::{check_in_group, check_senders};

pub const PATTERN: &str = "/drive";

#[check_senders]
#[check_in_group]
pub async fn handler(message: TelegramMessage, state: AppState) -> Result<()> {
    let onedrive = &state.onedrive;

    let cmd = cmd_parser(message.text());

    if cmd.len() == 1 {
        // /drive
        show_drive(onedrive, message).await?;
    } else if cmd.len() == 2 {
        if cmd[1] == "add" {
            // /drive add
            add_drive(message, state.clone()).await?;
        } else if cmd[1] == "logout" {
            // /drive logout
            logout_current_drive(onedrive, message).await?;
        } else if cmd[1] == "help" {
            // /drive help
            message
                .respond(InputMessage::html(format_help(PATTERN)))
                .await
                .context("help")?;
        } else {
            // /drive $index
            let index = cmd[1]
                .parse::<usize>()
                .context("账号索引应为整数")?
                - 1;

            set_drive(onedrive, message, index).await?;
        }
    } else if cmd.len() == 3 {
        if cmd[1] == "logout" {
            // /drive logout $index
            let index = cmd[2]
                .parse::<usize>()
                .context("账号索引应为整数")?
                - 1;

            logout_drive(onedrive, message, index).await?;
        } else {
            return Err(anyhow!("子命令错误")).context(format_unknown_command_help(PATTERN));
        }
    } else {
        return Err(anyhow!("命令错误")).context(format_unknown_command_help(PATTERN));
    }

    Ok(())
}

async fn show_drive(onedrive: &OneDriveClient, message: TelegramMessage) -> Result<()> {
    let usernames = onedrive.get_usernames().await?;
    if let Some(current_username) = onedrive.get_current_username().await? {
        if !usernames.is_empty() {
            let response = {
                let mut response = format!("当前账号是 {}", current_username);

                if usernames.len() > 1 {
                    response.insert(0, '\n');
                    for i in (1..=usernames.len()).rev() {
                        response.insert_str(0, &format!("{}. {}\n", i, usernames[i - 1]));
                    }
                }

                response
            };
            message.respond(response.as_str()).await.context(response)?;

            return Ok(());
        }
    }

    let response = "未找到账号。";
    message.respond(response).await.context(response)?;

    Ok(())
}

async fn add_drive(message: TelegramMessage, state: AppState) -> Result<()> {
    let (_, tx_od, _, rx_od, _server_abort_handle) = auth_server::spawn().await?;

    {
        *state.auth_tx_od.lock().await = Some(tx_od);
    }

    let mut messages_to_delete = Vec::new();
    authorize_onedrive(
        message.clone(),
        state.clone(),
        true,
        rx_od,
        &mut messages_to_delete,
    )
    .await?;

    {
        *state.auth_tx_od.lock().await = None;
    }

    for msg_id in messages_to_delete {
        state.telegram_bot.delete_messages(message.chat(), &[msg_id]).await.ok();
    }
    message.delete().await.ok();

    Ok(())
}

async fn logout_current_drive(onedrive: &OneDriveClient, message: TelegramMessage) -> Result<()> {
    let current_username = onedrive
        .get_current_username()
        .await?
        .ok_or_else(|| anyhow!("没有登录 OneDrive 账号"))?;

    onedrive.logout(None).await?;

    let response = {
        let mut response = format!(
            "OneDrive 账号 {} 已成功登出。",
            current_username
        );

        if let Some(current_username) = onedrive.get_current_username().await? {
            response.push_str(&format!("\n\n当前账号是 {}", current_username));
        }

        response
    };

    message.respond(response.as_str()).await.context(response)?;

    Ok(())
}

async fn set_drive(
    onedrive: &OneDriveClient,
    message: TelegramMessage,
    index: usize,
) -> Result<()> {
    let current_username = onedrive
        .get_current_username()
        .await?
        .ok_or_else(|| anyhow!("没有登录 OneDrive 账号"))?;

    let usernames = onedrive.get_usernames().await?;

    let selected_username = usernames
        .get(index)
        .ok_or_else(|| anyhow!("账号索引超出范围"))?;

    onedrive.change_account(selected_username).await?;

    if current_username == *selected_username {
        let response = "相同账号，无需更改。";
        message.respond(response).await.context(response)?;
    } else {
        let response = format!(
            "账号已从\n{}\n切换到\n{}",
            current_username, selected_username
        );
        message.respond(response.as_str()).await.context(response)?;
    }

    Ok(())
}

async fn logout_drive(
    onedrive: &OneDriveClient,
    message: TelegramMessage,
    index: usize,
) -> Result<()> {
    let usernames = onedrive.get_usernames().await?;

    let selected_username = usernames
        .get(index)
        .ok_or_else(|| anyhow!("account index out of range"))?;

    onedrive.logout(Some(selected_username.clone())).await?;

    let response = {
        let mut response = format!(
            "OneDrive 账号 {} 已成功登出。",
            selected_username
        );

        if let Some(current_username) = onedrive.get_current_username().await? {
            response.push_str(&format!("\n\n当前账号是 {}", current_username));
        }

        response
    };

    message.respond(response.as_str()).await.context(response)?;

    Ok(())
}
