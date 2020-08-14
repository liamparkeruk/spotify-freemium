tell application "System Events"
    if application process "Spotify" exists then
        return true
    else
        return false
    end if
end tell
