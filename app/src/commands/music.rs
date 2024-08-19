use crate::{Context, Error};
use poise::{serenity_prelude as serenity, CreateReply};
use rusty_ytdl::{Video, VideoOptions, VideoSearchOptions};

use super::super::playlist::playlist;
use super::super::playlist::track;

/// 指定したURLをプレイリストに追加します。(Youtubeのみ)
#[poise::command(slash_command, prefix_command, category = "Music")]
pub(crate) async fn add(ctx: Context<'_>, url: String) -> Result<(), Error> {
    ctx.say("This is a test command".to_string()).await?;
    Ok(())
}
