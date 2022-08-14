use anyhow::{bail, Result};
use clap::Parser;
use cli::Args;
use colour::{blue_ln, green_ln, red_ln};
use models::track::Track;
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

    green_ln!("Playlist query: {}", args.playlist_query);

    let playlists = spotify.search_playlists(&args.playlist_query).await?;

    if playlists.is_empty() {
        bail!("No playlists matched that query.")
    }

    let mut selected_playlist: Option<SlimPlaylist> = None;

    if playlists.len() == 1 {
        let playlist = playlists.first().unwrap().clone();
        blue_ln!("Found one match: {} by {}", playlist.name, playlist.owner);
        selected_playlist = Some(playlists.first().unwrap().clone());
    } else if playlists.first().unwrap().name == args.playlist_query.trim() {
        let playlist = playlists.first().unwrap().clone();
        blue_ln!("Found exact match: {} by {}", playlist.name, playlist.owner);
        selected_playlist = Some(playlist);
    } else {
        for playlist in playlists {
            blue_ln!("Found playlist: {} by {}", playlist.name, playlist.owner);
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

    let num_tracks = playlist.tracks.len();

    let mut new_tracks = Vec::with_capacity(num_tracks);
    let mut should_create_playlist = false;

    for track in playlist.tracks {
        if !track.explicit {
            let tracks = spotify.search_tracks(&track.title).await?;
            let new = find_explicit_version(&track, tracks);
            if let Some(track) = new {
                green_ln!("Replacing track: {}", track.title);
                should_create_playlist = true;
                new_tracks.push(track);
            } else {
                new_tracks.push(track);
            }
        } else {
            new_tracks.push(track);
        }
    }

    if !should_create_playlist {
        red_ln!("Found no replaceable tracks, new playlist will not be created")
    }

    Ok(())
}

fn find_explicit_version(base: &Track, tracks: Vec<Track>) -> Option<Track> {
    for track in tracks {
        if track.explicit
            && track.title == base.title
            && do_vecs_match(&base.artists, &track.artists)
        {
            return Some(track);
        }
    }

    None
}

// stolen from Paul Chernoch and Shepmaster
// https://stackoverflow.com/questions/29504514/whats-the-best-way-to-compare-2-vectors-or-strings-element-by-element
fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}
