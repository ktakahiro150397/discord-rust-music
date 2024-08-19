use super::track::Track;

pub(crate) struct PlayList {
    pub songs: Vec<Track>,
}

impl PlayList {
    pub fn new() -> Self {
        Self { songs: Vec::new() }
    }

    pub fn add(&mut self, track: Track) {
        self.songs.push(track);
    }
}
