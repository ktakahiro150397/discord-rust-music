use rusty_ytdl::{Video, VideoOptions, VideoSearchOptions};
use serenity::model::error;
use std::env::current_dir;
use std::fmt;
use std::path::PathBuf;
use tracing::{error, info_span, trace, trace_span, Instrument};

/// キューに追加されるトラック情報
#[derive(Debug)]
pub struct Track {
    pub title: String,
    source_url: String,
    file_path: PathBuf,
}

impl Track {
    pub fn new(title: &str, source_url: &str, file_path: PathBuf) -> Self {
        Self {
            title: title.to_string(),
            source_url: source_url.to_string(),
            file_path: file_path,
        }
    }

    #[tracing::instrument]
    pub async fn from_youtube_url(temp_path: &PathBuf, url: &str) -> Result<Self, TrackError> {
        // URLを確認
        let video_options = VideoOptions {
            filter: VideoSearchOptions::Audio,
            ..Default::default()
        };
        let video = match Video::new_with_options(url, video_options) {
            Ok(v) => v,
            Err(_) => {
                // URLが見つからなかった
                return Err(TrackError::NotFound);
            }
        };

        // 動画情報を取得
        let video_detail = match video
            .get_info()
            .instrument(info_span!("Get Youtube video detail"))
            .await
        {
            Ok(v) => v.video_details,
            Err(e) => {
                // 動画情報が取得できなかった
                error!("{:?}", e);
                return Err(TrackError::FailedToRetrieveInfo);
            }
        };

        // ダウンロード
        let current = current_dir().expect("failed to get current dir");
        let temp_path = current.join(temp_path);

        if !temp_path.exists() {
            std::fs::create_dir(temp_path.clone()).unwrap();
        }

        let file_name = format!("{}.mp3", video_detail.video_id);
        let temp_path = temp_path.join(file_name);

        match video
            .download(&temp_path)
            .instrument(info_span!("Download Youtube video"))
            .await
        {
            Err(e) => {
                // ダウンロードに失敗
                error!("{:?}", e);
                return Err(TrackError::FailedToDownload);
            }
            _ => {}
        };

        // トラック情報を返す
        Ok(Self {
            title: video_detail.title,
            source_url: url.to_string(),
            file_path: temp_path.clone(),
        })
    }
}

#[derive(Debug)]
pub enum TrackError {
    NotFound,
    FailedToDownload,
    FailedToRetrieveInfo,
}

impl fmt::Display for TrackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrackError::NotFound => write!(f, "URLが見つかりませんでした。"),
            TrackError::FailedToDownload => write!(f, "ダウンロードに失敗しました。"),
            TrackError::FailedToRetrieveInfo => write!(f, "動画情報の取得に失敗しました。"),
        }
    }
}

impl std::error::Error for TrackError {}
