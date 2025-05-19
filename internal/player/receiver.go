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
            if !player.active && player.size > 0 {
                PlayFile(player.queue[0])
            }

            time.Sleep(250 * time.Millisecond)
        }
    }
}

func interpretCommand(cmd server.Command) (server.Response, error) {
    var (
        resp    server.Response
        err     error
    )

    switch cmd.Command {
    case "ADD_QUEUE":
        if cmd.Payload == nil {
            return server.Response{Status: "ERR", ErrMsg: "missing payload"}, err
        }

        resp = addQueue(*cmd.Payload)

    case "PAUSE":
        resp = pauseSong()

    case "CLEAR":
        resp = clearPlayer()

    case "LOOP_SONG":
        resp = loopSong()

    // TODO: support for ADD_PLAYLIST, looping, SET_VOLUME
    }

    return resp, nil
}

