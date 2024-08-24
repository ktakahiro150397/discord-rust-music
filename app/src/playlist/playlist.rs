use super::track::Track;
use tracing::{error, info, info_span};

#[derive(Debug)]
pub(crate) struct PlayList {
    pub songs: Vec<Track>,
}

impl PlayList {
    pub fn new() -> Self {
        Self { songs: Vec::new() }
    }

    pub fn add(&mut self, track: Track) {
        info!("Add Track: {:?}", &track);
        self.songs.push(track);
    }
}
