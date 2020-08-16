//! Utility functions to initialize the application

extern crate dirs;

use std::{path::PathBuf, env};

/// Checks if the application is running on MacOS
///
/// # Panics
///
/// - If the operating system is not MacOS
pub fn ensure_macos() {
    assert_eq!("macos", env::consts::OS, "Spotify Freemium only runs on MacOS.");
}

/// Locates the Spotify installation directory
///
/// Finds the user home directory using the [`dirs`](https://docs.rs/crate/dirs/) crate and appends
/// the location of the Spotify installation
///
/// # Examples
///
/// If the user is named `dominicrutk`, the Spotify installation directory will be:
/// ```text
/// /Users/dominicrutk/Library/Application Support/Spotify/Users
/// ```
///
/// # Panics
///
/// - If the user home directory cannot be found
pub fn get_spotify_dir() -> PathBuf {
    let home_dir = dirs::home_dir();
    if let Some(mut home_dir) = home_dir {
        home_dir.push(["Library", "Application Support", "Spotify", "Users"].iter().collect::<PathBuf>());
        home_dir
    } else {
        panic!("Something went wrong locating the path of your Spotify installation...");
    }
}