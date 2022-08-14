use anyhow::bail;
use rspotify::model::{FullTrack, PlayableItem, PlaylistItem};

#[derive(Clone, Debug)]
pub struct Track {
    pub id: String,
    pub title: String,
    pub artists: Vec<String>,
    pub explicit: bool,
}

impl TryFrom<&PlaylistItem> for Track {
    type Error = anyhow::Error;
    fn try_from(value: &PlaylistItem) -> Result<Self, Self::Error> {
        if let Some(track) = &value.track {
            if let PlayableItem::Track(song) = track {
                if song.is_local {
                    bail!("playable item is a local track")
                }

                return Ok(Track {
                    artists: song
                        .artists
                        .iter()
                        .map(|artist| artist.name.clone())
                        .collect(),
                    id: song.id.clone().expect("track contained no id").to_string(),
                    title: song.name.clone(),
                    explicit: song.explicit,
                });
            } else {
                bail!("Playable item is a podcast episode");
            }
        } else {
            bail!("no playable item for playlist item");
        }
    }
}

impl From<&FullTrack> for Track {
    fn from(value: &FullTrack) -> Self {
        Track {
            artists: value
                .artists
                .iter()
                .map(|artist| artist.name.clone())
                .collect(),
            explicit: value.explicit,
            id: value.id.clone().unwrap().to_string(),
            title: value.name.clone(),
        }
    }
}
