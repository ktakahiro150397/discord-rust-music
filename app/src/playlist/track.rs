use std::fmt;
use std::path::PathBuf;

/// キューに追加されるトラック情報
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

    pub async fn from_youtube_url(url: &str) -> Result<Self, TrackError> {
        Err(TrackError::NotFound)
        // std::unimplemented!("Not implemented yet");

        // if true {
        //     return Err(TrackError::NotFound);
        // }

        // Ok(Self {
        //     title: "test".to_string(),
        //     source_url: url.to_string(),
        //     file_path: PathBuf::from("test"),
        // })
    }
}

#[derive(Debug)]
pub enum TrackError {
    NotFound,
    Forbidden,
    Unexpected,
}

impl fmt::Display for TrackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrackError::NotFound => write!(f, "Track not found"),
            TrackError::Forbidden => write!(f, "Track is forbidden"),
            TrackError::Unexpected => write!(f, "Unexpected error"),
        }
    }
}

impl std::error::Error for TrackError {}
