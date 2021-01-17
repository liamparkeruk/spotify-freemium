//! Logic to handle ads by restarting Spotify

use crate::spotify;

/// Handles an event that might indicate an ad playing
///
/// After an event occurs, the following steps are taken:
///
/// 1. Ensure that Spotify is open and that an ad is playing
/// 2. Quit and reopen Spotify
/// 3. Start audio playback
pub fn handle_event() -> bool {
    let mut restarted = false;
    match spotify::is_open() {
        Ok(open) => {
            if open {
                match spotify::is_playing_ad() {
                    Ok(playing_ad) => {
                        if playing_ad {
                            let Err(error) => {
                                    eprintln!("{}", error);
                                    0
                                },
                            };
                            if let Err(error) = spotify::quit() {
                                eprintln!("{}", error);
                            }
                            if let Err(error) = spotify::open() {
                                eprintln!("{}", error);
                            }
                            if let Err(error) = spotify::play() {
                                eprintln!("{}", error);
                            }
                            restarted = true;
                        }
                    },
                    Err(error) => eprintln!("{}", error),
                }
            }
        },
        Err(error) => eprintln!("{}", error),
    }
    restarted
}
