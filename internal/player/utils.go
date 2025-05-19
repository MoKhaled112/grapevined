package player

import (
    "os"
    "bufio"
    "strings"

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

func loopPlaylist() server.Response {
    if player == nil {
        return server.Response {
            Status: "ERR",
            ErrMsg: "The grapevined service is not active or initialized",
        }
    }

    if player.queue == nil || player.size == 0 {
        return server.Response {
            Status: "ERR",
            ErrMsg: "the queue is currently empty",
        }
    }

    player.loopPl = !player.loopPl
    return server.Response {
        Status: "OK",
        ErrMsg: "",
    }
}


func addPlaylist(playlistPath string) server.Response {
    if player == nil {
        return server.Response {
            Status: "ERR",
            ErrMsg: "The grapevined service is not active or initialized",
        }
    }
    
    // Fastest way to clear it, just ignore the SUCCESS response from this
    clearPlayer()
    size, err := parsePlaylist(playlistPath)
    if err != nil {
        return server.Response{Status: "ERR", ErrMsg: err.Error(),}
    }
    
    player.size = size
    return server.Response{Status: "OK", ErrMsg: "",}
}

func parsePlaylist(playlistPath string) (int, error) {
    file, err := os.Open(playlistPath)
    // ideally grapectl already verified this is a valid file but for 3rd party controllers
    if err != nil {
        return 0, err
    }

    defer file.Close()

    var size = 0

    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
        line := strings.TrimSpace(scanner.Text())
        if line != "" {
            size++
            player.queue = append(player.queue, line)
        }
    }

    if err = scanner.Err(); err != nil {
        return 0, err
    }

    return size, nil
}

func skipSong() server.Response {
    if player == nil {
        return server.Response{Status: "ERR", ErrMsg: "The grapevined service is not active or initialized",}
    }

    if !player.active {
        return server.Response{Status: "ERR", ErrMsg: "There is no song currently playing"}
    }

    player.skip <- struct{}{}

    return server.Response{Status: "OK", ErrMsg: ""}
}

// TODO: addPlaylist, setVolume, Looping
