package player

import (
    "github.com/altkeys/grapevined/internal/server"
    "github.com/gopxl/beep/v2/speaker"
)

func clearPlayer() server.Response {
    if player == nil {
        return server.Response{
            Status: "ERR",
            ErrMsg: "The grapevined service is not active or initialized",
        }
    }

    player.queue = nil
    player.size = 0
    
    if player.active {
        // better to let asyncPlay clear the speaker and set
        // active to false
        player.skip <- struct{}{}
    }
    
    return server.Response{
        Status: "OK",
        ErrMsg: "",
    }
}

func addQueue(songPath string) server.Response {
    if player == nil {
        return server.Response{
            Status: "ERR",
            ErrMsg: "The grapevined service is not active or initialized",
        }
    }
    // TODO: probably mutex lock this huh
    player.queue = append(player.queue, songPath)
    player.size++

    return server.Response{Status: "OK", ErrMsg: "",}
}

func pauseSong() server.Response {
    if player == nil {
        return server.Response{
            Status: "ERR",
            ErrMsg: "The grapevined service is not active or initialized",
        }
    }

    if !player.active {
        return server.Response{
            Status: "ERR",
            ErrMsg: "There is no song currently playing",
        }
    }

    speaker.Lock()
    player.current.Paused = !player.current.Paused
    speaker.Unlock()

    return server.Response{
        Status: "OK",
        ErrMsg: "",
    }
}

func loopSong() server.Response {
    if player == nil {
        return server.Response {
            Status: "ERR",
            ErrMsg: "The grapevined service is not active or initialized",
        }
    }

    if !player.active {
        return server.Response {
            Status: "ERR",
            ErrMsg: "there is no song currently playing",
        }
    }

    player.loopSong = !player.loopSong
    return server.Response {
        Status: "OK",
        ErrMsg: "",
    }
}


// TODO: addPlaylist, setVolume, Looping
