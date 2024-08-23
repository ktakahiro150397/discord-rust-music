use crate::{Context, Error};
// use async_std::task;
use poise::{serenity_prelude as serenity, CreateReply};
use rusty_ytdl::{Video, VideoOptions, VideoSearchOptions};
// use std::time::Duration;

use super::super::playlist::playlist;
use super::super::playlist::track;

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

/// 指定のURLからYoutubeデータをダウンロードします。
#[poise::command(slash_command, category = "Test")]
pub(crate) async fn download(
    ctx: Context<'_>,
    #[description = "ダウンロードするURL"] url: String,
) -> Result<(), Error> {
    // 応答を遅らせる
    ctx.defer().await?;

    let message = ctx
        .say(format!("Downloading from {}...", url))
        .await?
        .clone();

    let video_options = VideoOptions {
        filter: VideoSearchOptions::Audio,
        ..Default::default()
    };

    let video = match Video::new_with_options(url, video_options) {
        Ok(v) => v,
        Err(_) => {
            let content = "指定されたURLの動画は見つかりませんでした...";
            let reply = CreateReply::default().content(content);
            message.edit(ctx, reply).await?;
            return Ok(());
        }
    };

    let details = video.get_info().await.unwrap().video_details;
    println!("{:?}", details);

    let folder = std::path::Path::new("temp");
    if !folder.exists() {
        std::fs::create_dir(folder).unwrap();
    }

    let file_name = format!("temp/{}.mp3", details.video_id);
    let path = std::path::Path::new(&file_name);
    video.download(path).await.unwrap();

    let reply =
        CreateReply::default().content(format!("Video downloaded {:?}", path.to_str().unwrap()));
    message.edit(ctx, reply).await?;

    Ok(())
}

/// プレイリストインスタンステスト
#[poise::command(slash_command, category = "Test")]
pub(crate) async fn playlist(ctx: Context<'_>) -> Result<(), Error> {
    // プレイリストを作成
    let mut playlist = playlist::PlayList::new();

    // 追加するトラック
    let track = track::Track::new("Test", "https://test.video.com", "".into());

    // トラック追加
    playlist.add(track);

    // 一時ファイルパス
    let temp_path = std::path::PathBuf::from("temp");
    let track = match track::Track::from_youtube_url(
        &temp_path,
        "https://www.youtube.com/watch?v=AsnMofieWkQ",
    )
    .await
    {
        Ok(t) => t,
        Err(e) => {
            println!("Error: {}", e);
            ctx.say(format!("Error: {}", e)).await?;
            return Ok(());
        }
    };
    println!("{:?}", track);

    // トラック追加
    playlist.add(track);

    println!("{:?}", playlist);

    ctx.say(format!("Title is {}", playlist.songs[1].title))
        .await?;

    Ok(())
}
