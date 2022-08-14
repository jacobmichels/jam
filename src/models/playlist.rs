use rspotify::model::{FullPlaylist, SimplifiedPlaylist};

use super::track::Track;

#[derive(Clone)]
pub struct Playlist {
    pub id: String,
    pub name: String,
    pub tracks: Vec<Track>,
}

impl From<FullPlaylist> for Playlist {
    fn from(fp: FullPlaylist) -> Self {
        let tracks: Vec<Track> = fp
            .tracks
            .items
            .iter()
            .map(Track::try_from)
            .filter(|item| item.is_ok())
            .map(|item| item.unwrap())
            .collect();

        Playlist {
            id: fp.id.to_string(),
            name: fp.name,
            tracks,
        }
    }
}

#[derive(Clone)]
pub struct SlimPlaylist {
    pub id: String,
    pub name: String,
    pub owner: String,
}

impl From<&SimplifiedPlaylist> for SlimPlaylist {
    fn from(sp: &SimplifiedPlaylist) -> Self {
        SlimPlaylist {
            id: sp.id.to_string(),
            name: sp.name.clone(),
            owner: sp
                .owner
                .clone()
                .display_name
                .expect("Owner has no display name"),
        }
    }
}
