use crate::{Context, Error};
use poise::{serenity_prelude as serenity, CreateReply};
use rusty_ytdl::{Video, VideoOptions, VideoSearchOptions};
use songbird::tracks::Track;
use tracing::{info, info_span, trace, warn};
use tracing_futures::Instrument;

use serenity::async_trait;
// Event related imports to detect track creation failures.
use songbird::events::{Event, EventContext, EventHandler as VoiceEventHandler, TrackEvent};

use super::super::playlist::playlist;
use super::super::playlist::track;

/// ユーザーのアカウント作成日時を表示します。
#[poise::command(slash_command, prefix_command, category = "Test")]
#[tracing::instrument(name = "command_age", fields(category = "Test"), skip(ctx))]
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
#[tracing::instrument(name = "command_download", fields(category = "Test"), skip(ctx))]
pub(crate) async fn download(
    ctx: Context<'_>,
    #[description = "ダウンロードするURL"] url: String,
) -> Result<(), Error> {
    info!("Downloading URL {}...", url);

    // 応答を遅らせる
    ctx.defer().instrument(info_span!("defer")).await?;

    let message = ctx
        .say(format!("Downloading from {}...", url))
        .await?
        .clone();

    let video_options = VideoOptions {
        filter: VideoSearchOptions::Audio,
        ..Default::default()
    };

    let video = match Video::new_with_options(&url, video_options) {
        Ok(v) => v,
        Err(e) => {
            warn!("{}", e);

            let content = "指定されたURLの動画は見つかりませんでした...";
            let reply = CreateReply::default().content(content);
            message.edit(ctx, reply).await?;
            return Ok(());
        }
    };

    let details = match video.get_info().instrument(info_span!("get_info")).await {
        Ok(d) => d.video_details,
        Err(e) => {
            warn!("{}", e);

            let content = format!("指定されたURLの動画は見つかりませんでした...");
            let reply = CreateReply::default().content(content);
            message.edit(ctx, reply).await?;
            return Ok(());
        }
    };

    let folder = std::path::Path::new("temp");
    if !folder.exists() {
        std::fs::create_dir(folder).unwrap();
        info!("Create folder : {}", folder.to_str().unwrap());
    }

    let file_name = format!("temp/{}.mp3", details.video_id);
    let path = std::path::Path::new(&file_name);
    video
        .download(path)
        .instrument(info_span!("download_youtube"))
        .await
        .unwrap();
    info!("Downloaded {:?}", path.to_str().unwrap());

    let reply = CreateReply::default().content(format!(
        "Video downloaded! {} {:?}",
        url,
        path.to_str().unwrap()
    ));
    message.edit(ctx, reply).await?;

    Ok(())
}

/// プレイリストインスタンステスト
#[poise::command(slash_command, category = "Test")]
#[tracing::instrument(name = "command_playlist", fields(category = "Test"), skip(ctx))]
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
    trace!("{:?}", track);

    // トラック追加
    playlist.add(track);

    trace!("{:?}", playlist);

    ctx.say(format!("Title is {}", playlist.songs[1].title))
        .await?;

    Ok(())
}

/// あなたがいるボイスチャンネルに接続します。
#[poise::command(slash_command, category = "Test")]
#[tracing::instrument(name = "command_join", fields(category = "Test"), skip(ctx))]
pub(crate) async fn join(ctx: Context<'_>) -> Result<(), Error> {
    info!("join command called.");

    // 応答を遅らせる
    ctx.defer().instrument(info_span!("defer")).await?;

    let (guild_id, channel_id) = {
        let guild = ctx.guild().unwrap();
        let author_id = ctx.author().id;
        let guild_id = guild.id;

        let channel_id = guild
            .voice_states
            .get(&author_id)
            .and_then(|voice_state| voice_state.channel_id);

        (guild_id, channel_id)
    };

    trace!("guild_id: {:?}, channel_id: {:?}", guild_id, channel_id);

    // メッセージ送信者がボイスチャンネルに接続しているか確認
    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            warn!("Author is not in a voice channel");
            ctx.say("You are not in a voice channel").await?;
            return Ok(());
        }
    };

    info!("Connecting to : {:?}", connect_to);

    // songbirdコンテキストを取得
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let join = manager
        .join(guild_id, connect_to)
        .instrument(info_span!("join_voice_channel"))
        .await;

    match join {
        Ok(handler_lock) => {
            let mut handler = handler_lock.lock().await;
            handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier);
            info!("Connected to {:?}", channel_id);

            let message = ctx.say(format!("Left voice channel.")).await?.clone();
        }
        _ => (),
    }

    Ok(())
}

/// ボイスチャンネルから切断します。
#[poise::command(slash_command, category = "Test")]
#[tracing::instrument(name = "command_leave", fields(category = "Test"), skip(ctx))]
pub(crate) async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    // 応答を遅らせる
    ctx.defer().instrument(info_span!("defer")).await?;

    // songbirdコンテキストを取得
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        match manager
            .remove(guild_id)
            .instrument(info_span!("leave_voice_channel"))
            .await
        {
            Ok(_) => info!("Left voice channel."),
            Err(e) => warn!("Failed to leave voice channel: {:?}", e),
        };
    }

    let message = ctx.say(format!("Left voice channel.")).await?.clone();

    Ok(())
}

/// 曲を追加します。
#[poise::command(slash_command, category = "Test")]
#[tracing::instrument(name = "command_play", fields(category = "Test"), skip(ctx))]
pub(crate) async fn play(ctx: Context<'_>) -> Result<(), Error> {
    info!("play command called.");

    // 応答を遅らせる
    ctx.defer().instrument(info_span!("defer")).await?;

    let (guild_id, channel_id) = {
        let guild = ctx.guild().unwrap();
        let author_id = ctx.author().id;
        let guild_id = guild.id;

        let channel_id = guild
            .voice_states
            .get(&author_id)
            .and_then(|voice_state| voice_state.channel_id);

        (guild_id, channel_id)
    };

    trace!("guild_id: {:?}, channel_id: {:?}", guild_id, channel_id);

    // メッセージ送信者がボイスチャンネルに接続しているか確認
    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            warn!("Author is not in a voice channel");
            ctx.say("You are not in a voice channel").await?;
            return Ok(());
        }
    };

    info!("Connecting to : {:?}", connect_to);

    // songbirdコンテキストを取得
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let join = manager
        .join(guild_id, connect_to)
        .instrument(info_span!("join_voice_channel"))
        .await;

    match join {
        Ok(handler_lock) => {
            let mut handler = handler_lock.lock().await;
            handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier);
            info!("Connected to {:?}", channel_id);

            // 再生処理
            let mut driver = songbird::driver::Driver::default();

            // let source = songbird::ffmpeg("temp/CCWrCIfln3Q.mp3")
            //     .await
            //     .expect("Failed to create source.");
            let source = songbird::input::File::new("temp/CCWrCIfln3Q.mp3");

            info!("Playing {:?}", source);

            let track = Track::from(source);
            //let track_handle = driver.enqueue(track).await;
            //track_handle.play().expect("Failed to play track.");

            //let track_handle = handler.play(Track::from(source));
            let track_handle = handler.enqueue(track).await;
            info!("TrackHandle: {:?}", track_handle);

            let queue_len = handler.queue().len();

            let _ = ctx
                .say(format!("Queue length is {}", queue_len))
                .await?
                .clone();

            // let handle = driver.play_only(Track::from(source));

            // handle.play().expect("Failed to play track.");

            //handler.play_input("temp/AsnMofieWkQ.mp3");
        }
        _ => (),
    }

    Ok(())
}

/// 再生中の曲情報を取得します。
#[poise::command(slash_command, category = "Test")]
#[tracing::instrument(name = "command_info", fields(category = "Test"), skip(ctx))]
pub(crate) async fn info(ctx: Context<'_>) -> Result<(), Error> {
    // 応答を遅らせる
    ctx.defer().instrument(info_span!("defer")).await?;

    let guild_id = ctx.guild().unwrap().id;

    // songbirdコンテキストを取得
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;

        let queue_len = handler.queue().len();

        info!("Queue length: {}", queue_len);
        let _ = ctx
            .say(format!("Current queue length is {}!", queue_len))
            .await?
            .clone();

        for elem in handler.queue().current_queue() {
            info!("Track: {:?}", elem);
        }
    }

    Ok(())
}

struct TrackErrorNotifier;

#[async_trait]
impl VoiceEventHandler for TrackErrorNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            for (state, handle) in *track_list {
                println!(
                    "Track {:?} encountered an error: {:?}",
                    handle.uuid(),
                    state.playing
                );
            }
        }

        None
    }
}
