extern crate notify;

use notify::{Watcher, raw_watcher, RecursiveMode};
use std::fs;
use std::sync::mpsc;

mod config;
mod spotify;

fn main() {
    // Configure the application
    config::ensure_macos();
    let spotify_dir = config::get_spotify_dir();

    // Set up file watcher
    let (tx, rx) = mpsc::channel();
    let mut watcher = raw_watcher(tx).expect("Something went wrong creating a file watcher...");
    watcher.watch(spotify_dir, RecursiveMode::Recursive).expect("Something went wrong adding the Spotify installation directory to the file watcher...");

    // Watch for file changes
    loop {
        match rx.recv() {
            Ok(event) => {
                println!("Event: {:?}", event);
            },
            Err(error) => {
                println!("Error: {:?}", error);
            }
        }
    }
}