use crate::{Context, Error};
use poise::serenity_prelude as serenity;

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
