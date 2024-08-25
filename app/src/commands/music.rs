use crate::{Context, Error};

/// 指定したURLをプレイリストに追加します。(Youtubeのみ)
#[poise::command(slash_command, prefix_command, category = "Music")]
pub(crate) async fn add(ctx: Context<'_>, _url: String) -> Result<(), Error> {
    ctx.say("This is a test command".to_string()).await?;
    Ok(())
}
