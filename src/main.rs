use anyhow::{bail, Result};
use clap::Parser;
use cli::Args;
use colour::{blue_ln, green_ln};
use question::{Answer, Question};

use crate::{models::playlist::SlimPlaylist, spotify::SpotifyPKCEClient, traits::Spotify};

mod cli;
mod models;
mod spotify;
mod traits;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let spotify = SpotifyPKCEClient::new(
        "13034b6371a04f47bc53e5feb8435183",
        "http://localhost:8888/callback",
        vec!["playlist-modify-private".to_string()],
    );

    run(Box::new(spotify), args).await
}

async fn run(mut spotify: Box<dyn Spotify>, args: Args) -> Result<()> {
    spotify.auth().await?;

    green_ln!("Playlist query: {}", args.playlist_name);

    let playlists = spotify.search_playlists(&args.playlist_name).await?;

    if playlists.is_empty() {
        bail!("No playlists matched that query.")
    }

    let mut selected_playlist: Option<SlimPlaylist> = None;

    if playlists.len() == 1 {
        selected_playlist = Some(playlists.first().unwrap().clone());
    } else {
        for playlist in playlists {
            blue_ln!("Found playlist: {}", playlist.name);
            let ans = Question::new("Is this the playlist you want to convert? (Y/n)")
                .default(Answer::YES)
                .yes_no()
                .ask()
                .unwrap();
            if let Answer::YES = ans {
                selected_playlist = Some(playlist);
                break;
            }
        }
    }

    if selected_playlist.is_none() {
        bail!("No playlist selected");
    }

    let playlist = spotify
        .get_full_playlist(&selected_playlist.unwrap().id)
        .await?;

    for track in playlist.tracks {
        blue_ln!("{}", track.title);
    }

    Ok(())
}
