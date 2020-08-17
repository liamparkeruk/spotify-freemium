# Spotify Freemium

No... this is not a horrible freemium application filled with microtransactions. Instead, it gives you the most important benefit of Spotify Premium for free.

Spotify Freemium is a MacOS application that automatically restarts Spotify when an ad plays. This behavior allows users who don't subscribe to Spotify Premium to enjoy an ad-free listening experience. It is perfect for cheap developers who want to listen to music while working.

This project was inspired by the now-archived [MuteSpotifyAds](https://github.com/simonmeusel/MuteSpotifyAds). It is mostly just for my own practice developing in Rust. As such, there is almost certainly unidiomatic code all over the place.

## Getting Started

First, set permissions on the build scripts:

```shell script
chmod 777 scripts/debug.sh scripts/release.sh
```

A debug build will pipe output to the current shell and compile Rust in `dev` mode. The MacOS application bundle will appear at `./target/debug/bundle/macos/Spotify Freemium.app`. To run a debug build from the command line:

```shell script
./scripts/debug.sh
```

A release build will not pipe output to the shell and will compile Rust in `release` mode. To create a release build:

```shell script
./scripts/release.sh
```

The generated application bundle will be located at `./target/release/bundle/macos/Spotify Freemium.app`, and a compressed version will appear at `./target/release/bundle/macos/spotify-freemium-[VERSION].zip`.

## Performance

Like the original [MuteSpotifyAds](https://github.com/simonmeusel/MuteSpotifyAds), this app is incredibly performant. On a 2017 MacBook Pro with an Intel Core i5-7267U @ 3.10 GHz, the app uses 0.0% CPU when idling and around 0.1% to 0.2% when the song changes or an ad plays. The app uses a negligible amount of RAM (i.e., around 1 MB).

## License

This project is licensed under the ISC License. You can read the full license [here](LICENSE.md).
