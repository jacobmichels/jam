use anyhow::Result;
use async_trait::async_trait;
use dyn_clone::DynClone;

use crate::models::{
    playlist::{Playlist, SlimPlaylist},
    track::Track,
};

dyn_clone::clone_trait_object!(Spotify);

#[async_trait]
pub trait Spotify: DynClone {
    async fn auth(&mut self) -> Result<()>;
    async fn search_playlists(&self, playlist_name: &str) -> Result<Vec<SlimPlaylist>>;
    async fn get_full_playlist(&self, playlist_id: &str) -> Result<Playlist>;
    async fn search_tracks(&self, track_name: &str) -> Result<Vec<Track>>;
    async fn get_user_id(&self) -> Result<String>;
    async fn create_playlist(&self, playlist_name: &str, user_id: &str) -> Result<String>;
    async fn add_tracks_to_playlist(&self, playlist_id: &str, tracks: Vec<Track>) -> Result<()>;
}
