use std::process::Command;

fn execute_osascript(name: &str, args: Option<&[&str]>) -> Result<String, String> {
    match Command::new("osascript")
        .arg(format!("src/osascripts/{}.scpt", name))
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

pub fn open() -> Result<(), String> {
    if let Err(error) = execute_osascript("Open", None) {
        Err(error)
    } else {
        Ok(())
    }
}

pub fn quit() -> Result<(), String> {
    if let Err(error) = execute_osascript("Quit", None) {
        Err(error)
    } else {
        Ok(())
    }
}

pub fn play() -> Result<(), String> {
    if let Err(error) = execute_osascript("Play", None) {
        Err(error)
    } else {
        Ok(())
    }
}

pub fn pause() -> Result<(), String> {
    if let Err(error) = execute_osascript("Pause", None) {
        Err(error)
    } else {
        Ok(())
    }
}

pub fn play_pause() -> Result<(), String> {
    if let Err(error) = execute_osascript("PlayPause", None) {
        Err(error)
    } else {
        Ok(())
    }
}

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

pub fn mute() -> Result<(), String> {
    set_volume(0)
}

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

pub fn is_playing_ad() -> Result<bool, String> {
    match execute_osascript("GetSongURL", None) {
        Ok(stdout) => Ok(stdout.trim().starts_with("spotify:ad")),
        Err(error) => Err(error),
    }
}
