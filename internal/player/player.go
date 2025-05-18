package player

import (
    "os"
    "time"
    "errors"
    "log/slog"
    "path/filepath"

    "github.com/altkeys/grapevined/internal/server"

    "github.com/gopxl/beep/v2"
    "github.com/gopxl/beep/v2/mp3"
    "github.com/gopxl/beep/v2/flac"
    "github.com/gopxl/beep/v2/speaker"
)

type PlayerContext struct {
    current     *beep.Ctrl
    queue       []string
    index       int
    size        int
    active      bool

    log         *slog.Logger
    skip        chan struct{}
    interop     <- chan server.Command
    finished    chan struct{}               // signal playback finished
    response    chan <- server.Response


}

var (
    // streamline speaker.Init() and beep.Resample to 44.1khz
    sample  = beep.SampleRate(44100)
    player    *PlayerContext
)

// --- Core Functions ---

// Initializes the player context with the required channels
func Initialize(logger *slog.Logger, cmd <- chan server.Command, resp chan <- server.Response) error {
    if player != nil {
        logger.Error("player is already initialized")
        return errors.New("player already initialized")
    }

    player = &PlayerContext{
        current:    nil,
        queue:      nil,
        index:      0,
        size:       0,
        active:     false,
        log:        logger,
        skip:       make(chan struct{}),
        finished:   make(chan struct{}),
        interop:    cmd,
        response:   resp,
    }

    // speaker.Init() returns its own error which is important too 
    // so might as well make it the return for this
    return speaker.Init(sample, sample.N(time.Second / 10))
}

func PlayFile(songPath string) error {
    if player == nil {
        return errors.New("music player has not been initialized")
    }

    file, err := os.Open(songPath)
    if err != nil {
        player.log.Warn("failed to open music file", "SongPath", songPath, "err", err)
        return err
    }

    var (
        streamer    beep.StreamSeekCloser
        format      beep.Format
        extension = filepath.Ext(songPath)
    )

    switch extension {
    case ".flac":
        streamer, format, err = flac.Decode(file)
    case ".mp3":
        streamer, format, err = mp3.Decode(file)
    default:
        err = errors.New("only .mp3 and .flac files are currently supported")
    }

    if err != nil {
        player.log.Error("failed to decode file", "SongPath", songPath, "err", err)
        return err
    }

    resample := beep.Resample(6, format.SampleRate, sample, streamer)
    player.current = &beep.Ctrl{Streamer: resample, Paused: false}

    // Needs beep.StreamSeekCloser to allow for streamer.Close()
    // player.current.Streamer is beep.Streamer which does not implement
    // the Close() function, and it also needs the file to maintain open
    // so we close it in the goroutine
    go asyncPlay(streamer, file)
    return nil
}


func asyncPlay(streamer beep.StreamSeekCloser, file *os.File) {
    // these need to be closed in here otherwise no audio is played
    defer streamer.Close()
    defer file.Close()

    seq := beep.Seq(player.current, beep.Callback(func(){
        player.finished <- struct{}{}
    }))
    
    speaker.Play(seq)
    player.active = true
    player.log.Info("starting playback", "SongPath", file.Name())
    select {
    case <- player.skip:
        speaker.Clear()
        player.log.Info("skipped song", "SongPath", file.Name())
    case <- player.finished:
        player.log.Info("finished playback", "SongPath", file.Name())
    }

    // gg go next
    player.active = false
    player.current = nil
    player.queue = player.queue[1:]
    player.size--

    // TODO: allow for the looping of a single song/playlist
    // use player.index to facilitate playlist looping and a boolean for
    // single song looping (beep.Loop is not toggleable afaik)
}
