use anyhow::Result;
use clap::Parser;
use cli::Args;
use colour::green_ln;
use dirs::home_dir;
use rspotify::{
    model::{SearchResult, SearchType},
    prelude::{BaseClient, OAuthClient},
    scopes, AuthCodePkceSpotify, Credentials, OAuth,
};

mod cli;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    green_ln!("Playlist query: {}", args.playlist_name);

    let creds = Credentials::new_pkce("13034b6371a04f47bc53e5feb8435183");

    let oauth = OAuth {
        redirect_uri: "http://localhost:8888/callback".to_string(),
        scopes: scopes!("playlist-modify-private"),
        ..Default::default()
    };

    let mut spotify = AuthCodePkceSpotify::new(creds, oauth);
    let config_dir = home_dir().unwrap().join(".jam");
    let config_file = config_dir.join("credentials.json");

    std::fs::create_dir_all(config_dir).unwrap();
    spotify.config.cache_path = config_file;
    spotify.config.token_cached = true;
    spotify.config.token_refreshing = true;

    let url = spotify.get_authorize_url(None).unwrap();
    spotify.prompt_for_token(&url).await.unwrap();

    let search_result = spotify
        .search(
            &args.playlist_name,
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
