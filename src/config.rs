extern crate dirs;

use std::env;
use std::path::PathBuf;

pub fn ensure_macos() {
    assert_eq!("macos", env::consts::OS, "Spotify Freemium only runs on MacOS.");
}

pub fn get_spotify_dir() -> PathBuf {
    let home_dir = dirs::home_dir();
    if let Some(mut home_dir) = home_dir {
        home_dir.push(["Library", "Application Support", "Spotify", "Users"].iter().collect::<PathBuf>());
        home_dir
    } else {
        panic!("Something went wrong locating the path of your Spotify installation...");
    }
}