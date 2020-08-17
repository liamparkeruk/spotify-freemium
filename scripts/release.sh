#!/bin/bash

# Work from the project root
cd "${0%/*}"
cd ..

# Define console colors per https://stackoverflow.com/a/5947802/7300063
GREEN='\033[1;32m'
CLEAR='\033[0m'

# Shut down any running instances
printf "     ${GREEN}Quiting${CLEAR} other Spotify Freemium instance(s)\n"
killall -9 spotify-freemium &> /dev/null
killall -9 Spotify\ Freemium &> /dev/null

# Build release version
cargo build --release || exit $?

# Create package bundle directories
printf "    ${GREEN}Creating${CLEAR} package bundle directories\n"
mkdir -p target/release/bundle/macos/Spotify\ Freemium.app/Contents/MacOS
mkdir -p target/release/bundle/macos/Spotify\ Freemium.app/Contents/Resources/osascripts

# Copy Info.plist to package bundle
printf "     ${GREEN}Copying${CLEAR} Info.plist\n"
cp Info.plist target/release/bundle/macos/Spotify\ Freemium.app/Contents

# Copy executable to package bundle
printf "     ${GREEN}Copying${CLEAR} spotify-freemium release executable\n"
cp target/release/spotify-freemium target/release/bundle/macos/Spotify\ Freemium.app/Contents/MacOS

# Copy AppleScripts to package bundle
printf "     ${GREEN}Copying${CLEAR} AppleScripts\n"
cp -r resources/ target/release/bundle/macos/Spotify\ Freemium.app/Contents/Resources/

# Compress package bundle
printf "    ${GREEN}Creating${CLEAR} zip archive\n"
cd target/release/bundle/macos
zip spotify-freemium-0.1.0.zip Spotify\ Freemium.app -r -X > /dev/null

# Exit messages
printf "    ${GREEN}Finished${CLEAR} generating Spotify Freemium package bundle\n"
printf "    ${GREEN}--------${CLEAR} ------------------------------------------------------------------------------\n"
printf "        ${GREEN}Open${CLEAR} ./target/release/bundle/macos/Spotify Freemium.app to run Spotify Freemium\n"
printf "        ${GREEN}Copy${CLEAR} ./target/release/bundle/macos/spotify-freemium-0.1.0.zip to distribute\n"
printf "    ${GREEN}--------${CLEAR} ------------------------------------------------------------------------------\n"
