use crate::{Context, Error};
use async_std::task;
use poise::{serenity_prelude as serenity, CreateReply};
use std::time::Duration;

/// ユーザーのアカウント作成日時を表示します。
#[poise::command(slash_command, prefix_command, category = "Test")]
pub(crate) async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;

    Ok(())
}

/// テスト用のコマンドです。
#[poise::command(slash_command, prefix_command, category = "Test")]
pub(crate) async fn test(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("This is a test command".to_string()).await?;
    Ok(())
}

#[poise::command(slash_command, category = "Test")]
pub(crate) async fn download(
    ctx: Context<'_>,
    #[description = "ダウンロードするURL"] url: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    let message = ctx
        .say(format!("Downloading from {}...", url))
        .await?
        .clone();

    // Simulate a download
    task::sleep(Duration::from_secs(5)).await;

    message
        .edit(
            ctx,
            CreateReply::default().content(format!("Downloaded {}!", url)),
        )
        .await?;

    Ok(())
}
