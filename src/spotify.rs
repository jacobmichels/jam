use anyhow::{bail, Result};
use std::{collections::HashSet, fs, str::FromStr};

use async_trait::async_trait;
use dirs::home_dir;
use rspotify::{
    model::{PlaylistId, SearchResult, SearchType, TrackId, UserId},
    prelude::{BaseClient, OAuthClient, PlayableId},
    AuthCodePkceSpotify, Credentials, OAuth,
};

use crate::{
    models::{
        playlist::{Playlist, SlimPlaylist},
        track::Track,
    },
    traits::Spotify,
};

#[derive(Clone)]
pub struct SpotifyPKCEClient {
    client: AuthCodePkceSpotify,
}

impl SpotifyPKCEClient {
    pub fn new(client_id: &str, redirect_uri: &str, scopes: Vec<String>) -> SpotifyPKCEClient {
        let mut scopes_set = HashSet::with_capacity(scopes.len());
        for scope in scopes {
            scopes_set.insert(scope);
        }

        let credentials = Credentials::new_pkce(client_id);
        let oauth_config = OAuth {
            redirect_uri: redirect_uri.to_string(),
            scopes: scopes_set,
            ..Default::default()
        };

        let mut spotify = AuthCodePkceSpotify::new(credentials, oauth_config);

        let config_dir = home_dir().unwrap().join(".jam");
        let config_file = config_dir.join("credentials.json");

        fs::create_dir_all(config_dir).unwrap();
        spotify.config.cache_path = config_file;
        spotify.config.token_cached = true;
        spotify.config.token_refreshing = true;

        SpotifyPKCEClient { client: spotify }
    }
}

#[async_trait]
impl Spotify for SpotifyPKCEClient {
    async fn auth(&mut self) -> Result<()> {
        let url = self.client.get_authorize_url(None)?;
        self.client.prompt_for_token(&url).await?;

        Ok(())
    }

    async fn search_playlists(&self, playlist_name: &str) -> Result<Vec<SlimPlaylist>> {
        let search_result = self
            .client
            .search(
                playlist_name,
                &SearchType::Playlist,
                None,
                None,
                Some(50),
                None,
            )
            .await?;

        if let SearchResult::Playlists(playlists) = search_result {
            return Ok(playlists.items.iter().map(SlimPlaylist::from).collect());
        } else {
            bail!("Invalid search: result did not contain playlists")
        }
    }

    async fn get_full_playlist(&self, playlist_id: &str) -> Result<Playlist> {
        let playlist_id = PlaylistId::from_str(playlist_id)?;

        let playlist_response = self.client.playlist(&playlist_id, None, None).await?;

        return Ok(Playlist::from(playlist_response));
    }

    async fn search_tracks(&self, track_name: &str) -> Result<Vec<Track>> {
        let search_result = self
            .client
            .search(track_name, &SearchType::Track, None, None, Some(50), None)
            .await?;

        if let SearchResult::Tracks(tracks) = search_result {
            return Ok(tracks.items.iter().map(Track::from).collect());
        } else {
            bail!("Invalid search: result did not contain tracks")
        }
    }

    async fn get_user_id(&self) -> Result<String> {
        let response = self.client.me().await?;
        return Ok(response.id.to_string());
    }

    async fn add_tracks_to_playlist(&self, playlist_id: &str, tracks: Vec<Track>) -> Result<()> {
        let mut track_ids: Vec<Box<dyn PlayableId>> = Vec::with_capacity(tracks.len());
        for track in tracks.iter() {
            let id = TrackId::from_str(&track.id)?;
            track_ids.push(Box::new(id));
        }

        let mut track_id_refs: Vec<&dyn PlayableId> = Vec::with_capacity(tracks.len());
        for track_id in track_ids.iter() {
            track_id_refs.push(track_id.as_ref());
        }

        let playlist_id = &PlaylistId::from_str(playlist_id)?;

        self.client
            .playlist_add_items(playlist_id, track_id_refs, None)
            .await?;

        Ok(())
    }

    async fn create_playlist(&self, playlist_name: &str, user_id: &str) -> Result<String> {
        let response = self
            .client
            .user_playlist_create(
                &UserId::from_str(user_id)?,
                &format!("{} (explicit)", playlist_name),
                Some(false),
                Some(false),
                None,
            )
            .await?;
        return Ok(response.id.to_string());
    }
}
