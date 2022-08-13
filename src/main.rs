use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use cli::Args;
use rspotify::{
    model::{SearchResult, SearchType},
    prelude::{BaseClient, OAuthClient},
    scopes, AuthCodePkceSpotify, Credentials, OAuth,
};

mod cli;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!("{}", args.playlist_name);

    let creds = Credentials::new_pkce("13034b6371a04f47bc53e5feb8435183");

    let oauth = OAuth {
        redirect_uri: "http://localhost:8888/callback".to_string(),
        scopes: scopes!("playlist-modify-private"),
        ..Default::default()
    };

    let mut spotify = AuthCodePkceSpotify::new(creds, oauth);

    std::fs::create_dir_all("~/.jam/").unwrap();
    spotify.config.cache_path = PathBuf::from("~/.jam/credentials.json");
    spotify.config.token_cached = true;
    spotify.config.token_refreshing = true;

    let url = spotify.get_authorize_url(None).unwrap();
    spotify.prompt_for_token(&url).await.unwrap();
    // println!(
    //     "{:?}",
    //     spotify
    //         .token
    //         .lock()
    //         .await
    //         .unwrap()
    //         .as_ref()
    //         .unwrap()
    //         .access_token
    // );

    let search_result = spotify
        .search(
            "Top Songs - Canada",
            &SearchType::Playlist,
            None,
            None,
            Some(50),
            None,
        )
        .await
        .unwrap();

    match search_result {
        SearchResult::Playlists(page) => {
            for item in page.items {
                println!("{}", item.name);
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}
