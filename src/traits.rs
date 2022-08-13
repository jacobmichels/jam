use anyhow::Result;
use async_trait::async_trait;

use crate::models::playlist::{Playlist, SlimPlaylist};

#[async_trait]
pub trait Spotify {
    async fn auth(&mut self) -> Result<()>;
    async fn search_playlists(&self, playlist: &str) -> Result<Vec<SlimPlaylist>>;
    async fn get_full_playlist(&self, playlist_id: &str) -> Result<Playlist>;
}
