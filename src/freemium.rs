use crate::spotify;

pub fn handle_event() -> bool {
    if let Ok(playing_ad) = spotify::is_playing_ad() {
        if playing_ad {
            let volume = match spotify::get_volume() {
                Ok(volume) => volume,
                Err(error) => {
                    eprintln!("{}", error);
                    0
                },
            };
            if let Err(error) = spotify::mute() {
                eprintln!("{}", error);
            }
            if let Err(error) = spotify::quit() {
                eprintln!("{}", error);
            }
            if let Err(error) = spotify::open() {
                eprintln!("{}", error);
            }
            if let Err(error) = spotify::set_volume(volume) {
                eprintln!("{}", error);
            }
            if let Err(error) = spotify::play() {
                eprintln!("{}", error);
            }
            true
        } else {
            false
        }
    } else {
        false
    }
}