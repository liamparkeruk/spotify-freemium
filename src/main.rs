extern crate notify;

use notify::{Watcher, raw_watcher, RecursiveMode};
use std::{fs, sync::mpsc};

mod config;
mod freemium;
mod spotify;

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