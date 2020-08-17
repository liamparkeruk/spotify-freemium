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

# Build debug version
cargo build || exit $?

# Create package bundle directories
printf "    ${GREEN}Creating${CLEAR} package bundle directories\n"
mkdir -p target/debug/bundle/macos/Spotify\ Freemium.app/Contents/MacOS
mkdir -p target/debug/bundle/macos/Spotify\ Freemium.app/Contents/Resources

# Copy Info.plist to package bundle
printf "     ${GREEN}Copying${CLEAR} Info.plist\n"
cp Info.plist target/debug/bundle/macos/Spotify\ Freemium.app/Contents

# Copy executable to package bundle
printf "     ${GREEN}Copying${CLEAR} spotify-freemium debug executable\n"
cp target/debug/spotify-freemium target/debug/bundle/macos/Spotify\ Freemium.app/Contents/MacOS

# Copy AppleScripts to package bundle
printf "     ${GREEN}Copying${CLEAR} AppleScripts\n"
cp -r resources/ target/debug/bundle/macos/Spotify\ Freemium.app/Contents/Resources/

# Exit messages
printf "    ${GREEN}Finished${CLEAR} generating Spotify Freemium package bundle\n"
printf "    ${GREEN}--------${CLEAR} ---------------------------------------------------------------------------------\n"
printf "     ${GREEN}Running${CLEAR} ./target/debug/bundle/macos/Spotify Freemium.app/Contents/MacOS/spotify-freemium\n"
printf "    ${GREEN}--------${CLEAR} ---------------------------------------------------------------------------------\n"

# Run application with working directory /
DIR="$PWD/target/debug/bundle/macos/Spotify Freemium.app/Contents/MacOS/spotify-freemium"
cd /
"$DIR"
