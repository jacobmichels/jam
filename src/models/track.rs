use anyhow::bail;
use rspotify::model::{PlayableItem, PlaylistItem};

#[derive(Clone)]
pub struct Track {
    pub id: String,
    pub title: String,
    pub artist: String,
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
                    artist: song
                        .artists
                        .first()
                        .expect("track contained no artist")
                        .name
                        .clone(),
                    id: song.id.clone().expect("track contained no id").to_string(),
                    title: song.name.clone(),
                });
            } else {
                bail!("Playable item is a podcast episode");
            }
        } else {
            bail!("no playable item for playlist item");
        }
    }
}
