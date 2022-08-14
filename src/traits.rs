use anyhow::Result;
use async_trait::async_trait;

use crate::models::{
    playlist::{Playlist, SlimPlaylist},
    track::Track,
};

#[async_trait]
pub trait Spotify {
    async fn auth(&mut self) -> Result<()>;
    async fn search_playlists(&self, playlist_name: &str) -> Result<Vec<SlimPlaylist>>;
    async fn get_full_playlist(&self, playlist_id: &str) -> Result<Playlist>;
    async fn search_tracks(&self, track_name: &str) -> Result<Vec<Track>>;
}
