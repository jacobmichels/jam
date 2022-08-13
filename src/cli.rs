use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "github.com/jacobmichels/jam")]
#[clap(author = "Jacob Michels <jacob.michels2025@gmail.com>")]
#[clap(version = "0.1.0")]
#[clap(about = "Convert Spotify playlists to explicit", long_about = None)]
#[clap(propagate_version = true)]
pub struct Args {
    pub playlist_name: String,
}