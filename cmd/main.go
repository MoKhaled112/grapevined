package main

import (
    "os"
    "log/slog"
    "path/filepath"


    "github.com/altkeys/grapevined/internal/server"
    "github.com/altkeys/grapevined/internal/player"
)

// It's a surprise tool that will help us later
type GVConfig struct {
    MusicDir    string    `json:"music_dir"`
    LoopPl      bool      `json:"loop_pl"`

    // if these are non-null then enable the service
    Discord     *string   `json:"discord,omitempty"`
    Lastfm      *string   `json:"lastfm,omitempty"`

}

func main() {
    dir, err := os.UserConfigDir()
    if err != nil {
        panic("failed to fetch user config directory")
    }

    logPath := filepath.Join(dir, "grapevined", "log.json")
    if err = os.MkdirAll(filepath.Dir(logPath), 0755); err != nil {
        panic("failed to create configuration directory for grapevined")
    }
    
    logFile, err := os.OpenFile(logPath, os.O_CREATE | os.O_APPEND | os.O_WRONLY | os.O_TRUNC, 0644)
    if err != nil {
        panic("could not open grapevined log file")
    }
    defer logFile.Close()

    // os.Stdout for testing, ~/.config/grapevined/log for prod
    logger := slog.New(slog.NewJSONHandler(logFile, nil))

    var (
        // allow communication with TCP and Music threads
        interop     = make(chan server.Command)
        // allow Music thread to send a response to TCP thread
        response    = make(chan server.Response)
    )

    err = server.Initialize(logger, interop, response)
    if err != nil {
        logger.Error("failed to initialize grapevined TCP server", "err", err)
        return
    }

    err = player.Initialize(logger, interop, response)
    if err != nil {
        logger.Error("failed to initialize grapevined music server", "err", err)
        return
    }

    // consider WaitGroup graceful shutdown for this one later
    go player.Listen()
    err = server.Start()
    if err != nil {
        logger.Error("failed to start grapevined TCP server", "err", err)
        return
    }

    // if we get here that means the service closed
    logger.Info("successfully shutdown grapevined service")
}
