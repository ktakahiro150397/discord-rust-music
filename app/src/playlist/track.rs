use rusty_ytdl::{Video, VideoOptions, VideoSearchOptions};
use std::fmt;
use std::path::PathBuf;

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

    pub async fn from_youtube_url(temp_path: &PathBuf, url: &str) -> Result<Self, TrackError> {
        // URLを確認
        let video = match Video::new_with_options(
            url,
            VideoOptions {
                filter: VideoSearchOptions::Audio,
                ..Default::default()
            },
        ) {
            Ok(v) => v,
            Err(_) => {
                // URLが見つからなかった
                return Err(TrackError::NotFound);
            }
        };

        // 動画情報を取得
        let video_detail = match video.get_info().await {
            Ok(v) => v.video_details,
            Err(_) => {
                // 動画情報が取得できなかった
                return Err(TrackError::FailedToRetrieveInfo);
            }
        };

        // ダウンロード
        match video.download(temp_path).await {
            Err(_) => {
                // ダウンロードに失敗
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
