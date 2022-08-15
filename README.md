# jam
Converts Spotify playlists from clean -> explicit

![example output](/images/output.png)

## Motivation

Spotify likes to be inconsistent with their playlists, mixing clean and explicit songs within the same playlist. To me, this is annoying. I don't want to listen to censored music. This tool is a solution to this inconsistency.

## Usage

### Convert a playlist to explicit:
`jam "Power Gaming"`

This will create a new playlist in your account titled Power Gaming with clean songs swapped with their explicit version.

### Convert a playlist to explicit (dry run):
`jam --dry-run "Power Gaming"`

This will skip the playlist creation step.

### Convert a playlist to explicit while supplying an output name:
`jam "Power Gaming" --output "Cool Gaming Music 😎"`

This will create a new playlist in your account titled Cool Gaming Music 😎 with clean songs swapped with their explicit version.

## Installation

You'll need to compile from source. The following compiles jam and places it in ~/.cargo/bin
1. Get a stable rust toolchain with [rustup](https://rustup.rs/)
2. Run `cargo install --git https://github.com/jacobmichels/jam`
3. jam is now installed 🎉

## Authenticating with Spotify
jam uses oauth to authenticate with Spotify. Authenticating is a straightforward process handled at runtime. The access token and refresh token are cached at ~/.jam/credentials.json
