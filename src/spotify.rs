//! Functions to interface with Spotify
//!
//! These functions utilize AppleScripts stored in the `osascripts` directory to communicate with
//! Spotify easily and synchronously.

use std::process::Command;

/// Executes an AppleScript in the `osascripts` directory given the name of the script and arguments
///
/// - If the function succeeds, the result will be the value of `stdout` from the executed script.
/// - If the function fails, the result will be an error message.
fn execute_osascript(name: &str, args: Option<&[&str]>) -> Result<String, String> {
    match Command::new("osascript")
        .arg(format!("Resources/osascripts/{}.scpt", name))
        .args(if let Some(args) = args { args } else { &[] as &[&str] })
        .output() {
        Ok(output) => {
            if output.status.success() {
                match String::from_utf8(output.stdout) {
                    Ok(stdout) => Ok(stdout),
                    Err(error) => Err(format!("Something went wrong parsing UTF-8 bytes after successfully executing \"{}.scpt\": {:?}", name, error))
                }
            } else {
                match String::from_utf8(output.stderr) {
                    // Ok(stderr) => Err(stderr),
                    Ok(stderr) => Err(format!("Something went wrong executing \"{}.scpt\": {}", name, stderr)),
                    Err(error) => Err(format!("Something went wrong parsing UTF-8 bytes after unsuccessfully executing \"{}.scpt\": {:?}", name, error))
                }
            }
        },
        Err(error) => Err(format!("Something went wrong accessing \"{}.scpt\": {:?}", name, error))
    }
}

/// Opens the Spotify application
///
/// - If Spotify opens successfully, the result will be empty.
/// - If Spotify fails to open, the result will be an error message. In this application, the error
/// might be logged but will most likely be ignored.
pub fn open() -> Result<(), String> {
    if let Err(error) = execute_osascript("Open", None) {
        Err(error)
    } else {
        Ok(())
    }
}

/// Quits the Spotify application
///
/// - If Spotify quits successfully, the result will be empty.
/// - If Spotify fails to quit, the result will be an error message. In this application, the error
/// might be logged but will most likely be ignored.
pub fn quit() -> Result<(), String> {
    if let Err(error) = execute_osascript("Quit", None) {
        Err(error)
    } else {
        Ok(())
    }
}

/// Checks if the Spotify application is open
///
/// - If the check is successful, the result will be a boolean indicating whether the Spotify application is open.
/// - If the check is unsuccessful, the result will be an error message. In this application, the error
/// might be logged but will most likely be ignored.
pub fn is_open() -> Result<bool, String> {
    match execute_osascript("IsOpen", None) {
        Ok(stdout) => Ok(stdout.trim() == "true"),
        Err(error) => Err(error),
    }
}

/// Starts audio playback in Spotify
///
/// - If audio playback starts, the result will be empty.
/// - If audio playback fails to start, the result will be an error message. In this application, the error
/// might be logged but will most likely be ignored.
///
/// Note: if audio is already playing, this function will do nothing.
pub fn play() -> Result<(), String> {
    if let Err(error) = execute_osascript("Play", None) {
        Err(error)
    } else {
        Ok(())
    }
}

/// Pauses audio playback in Spotify
///
/// - If audio playback pauses, the result will be empty.
/// - If audio playback fails to pause, the result will be an error message. In this application, the error
/// might be logged but will most likely be ignored.
///
/// Note: if audio is already paused, this function will do nothing.
#[allow(dead_code)]
pub fn pause() -> Result<(), String> {
    if let Err(error) = execute_osascript("Pause", None) {
        Err(error)
    } else {
        Ok(())
    }
}

/// Toggles audio playback in Spotify
///
/// If audio is playing, it will pause. If it is paused, it will play.
///
/// - If audio playback toggles, the result will be empty.
/// - If audio playback fails to toggle, the result will be an error message. In this application, the error
/// might be logged but will most likely be ignored.
#[allow(dead_code)]
pub fn play_pause() -> Result<(), String> {
    if let Err(error) = execute_osascript("PlayPause", None) {
        Err(error)
    } else {
        Ok(())
    }
}

/// Sets the volume in Spotify to the given value
///
/// Valid volumes must be integers from 0 to 100, inclusive.
///
/// - If setting the volume is successful, the result will be empty.
/// - If setting the volume fails (including if the given volume is an invalid value), the result
/// will be an error message. In this application, the error might be logged but will most likely be ignored.
pub fn set_volume(volume: u8) -> Result<(), String> {
    if volume > 100 {
        return Err(String::from("Volume must be a whole number from 0 to 100"));
    }
    if let Err(error) = execute_osascript("SetVolume", Some(&vec![volume.to_string().as_str()])) {
        Err(error)
    } else {
        Ok(())
    }
}

/// Mutes Spotify
///
/// Convenience function for [`set_volume(0)`](./fn.set_volume.html)
pub fn mute() -> Result<(), String> {
    set_volume(0)
}

/// Gets the current volume
///
/// - If retrieving the volume is successful, the result will be an integer from 0 to 100, inclusive.
/// - If retrieving the volume is unsuccessful, the result will be an error message. In this application,
/// the error might be logged but will most likely be ignored.
pub fn get_volume() -> Result<u8, String> {
    match execute_osascript("GetVolume", None) {
        Ok(stdout) => {
            match stdout.trim().parse::<u8>() {
                Ok(volume) => Ok(volume),
                Err(error) => Err(format!("Something went wrong converting stdout \"{}\" to an unsigned, 8-bit integer: {:?}", stdout, error)),
            }
        },
        Err(error) => Err(error),
    }
}

/// Checks if Spotify is currently playing an ad
///
/// - If the check is successful, the result will be a boolean indicating whether an ad is playing (true)
/// or whether a song is playing (false).
/// - If the check is unsuccessful, the result will be an error message. In this application, the
/// error might be logged but will most likely be ignored.
pub fn is_playing_ad() -> Result<bool, String> {
    match execute_osascript("GetSongURL", None) {
        Ok(stdout) => Ok(stdout.trim().starts_with("spotify:ad")),
        Err(error) => Err(error),
    }
}

/// Gets the current song name
///
/// - If the function is successful, the result will be the song name.
/// - If the function is unsuccessful, the result will be an error message. In this application, the
/// error might be logged but will most likely be ignored.
///
/// Note: the song name is taken directly from `stdout`, so it should probably be trimmed before use.
/// This function does not trim it to avoid allocating memory for a new string when unnecessary.
#[allow(dead_code)]
pub fn get_song_name() -> Result<String, String> {
    match execute_osascript("GetSongName", None) {
        Ok(stdout) => Ok(stdout),
        Err(error) => Err(error),
    }
}