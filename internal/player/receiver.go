package player

import (
    "time"
    "errors"

    "github.com/altkeys/grapevined/internal/server"
)


func Listen() error {
    if player == nil {
        return errors.New("player has not been initialized")
    }

    for {
        select {
        case cmd := <- player.interop:
            resp, err := interpretCommand(cmd)
            if err != nil {
                player.log.Error("could not execute command", "cmd", cmd.Command, "err", err)
            }

            player.response <- resp
        default:
            if !player.active && len(player.queue) > 0 {
                PlayFile(player.queue[player.index])
            }

            time.Sleep(250 * time.Millisecond)
        }
    }
}

func interpretCommand(cmd server.Command) (server.Response, error) {
    var resp    server.Response

    switch cmd.Command {
    case "ADD_QUEUE":
        if cmd.Payload == nil {
            return server.Response{Status: "ERR", ErrMsg: "missing payload"}, errors.New("missing song file")
        }

        resp = addQueue(*cmd.Payload)

    case "ADD_PLAYLIST":
        if cmd.Payload == nil {
            return server.Response{Status: "ERR", ErrMsg: "missing payload"}, errors.New("missing playlist file")
        }

        resp = addPlaylist(*cmd.Payload)

    case "PAUSE":
        resp = pauseSong()

    case "CLEAR":
        resp = clearPlayer()

    case "LOOP_SONG":
        resp = loopSong()

    case "LOOP_PLAYLIST":
        resp = loopPlaylist()

    case "SKIP":
        resp = skipSong()


    // TODO: support for ADD_PLAYLIST, looping, SET_VOLUME
    }

    return resp, nil
}

