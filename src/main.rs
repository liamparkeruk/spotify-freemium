//! Spotify Freemium is a MacOS application that automatically restarts Spotify when an ad plays.
//! This behavior allows users who don't subscribe to Spotify Premium to enjoy an ad-free listening experience.
//! It is perfect for cheap developers who want to listen to music while working.
//!
//! The source code and a more detailed README are available on [GitHub](https://github.com/dominicrutk/spotify-freemium).

extern crate notify;

use notify::{Watcher, raw_watcher, RecursiveMode};
use std::{fs, sync::mpsc};

mod config;
mod freemium;
mod spotify;

/// Configures the application, listens for events, and responds to events by restarting Spotify
///
/// Configuring he application and responding to events are outsourced to the [`config`](./config/index.html) and
/// [`freemium`](./freemium/index.html) modules respectively. The `main` function's primarily responsibility is to
/// initialize this function and create a file watcher on the Spotify installation directory.
/// This file watcher runs in another thread and returns events that might indicate an ad is playing.
fn main() {
    // Configure the application
    config::ensure_macos();
    let spotify_dir = config::get_spotify_dir();

    // Set up file watcher
    let (tx, rx) = mpsc::channel();
    let mut watcher = raw_watcher(tx).expect("Something went wrong creating a file watcher...");
    for child in fs::read_dir(spotify_dir).expect("Something went wrong reading the Spotify installation directory...") {
        if let Ok(child) = child {
            let mut path = child.path();
            if path.is_dir() {
                path.push("ad-state-storage.bnk");
                if path.exists() {
                    watcher.watch(path, RecursiveMode::NonRecursive).expect("Something went wrong adding the Spotify installation directory to the file watcher...");
                }
            }
        }
    }

    // Watch for file changes
    loop {
        match rx.recv() {
            Ok(_) => {
                freemium::handle_event();
            },
            Err(error) => {
                eprintln!("Error: {:?}", error);
            }
        }
    }
}