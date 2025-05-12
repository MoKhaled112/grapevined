package main

import (
    "log/slog"
    "os"


    "github.com/altkeys/grapevined/internal/server"
)

func main() {
    logger := slog.New(slog.NewJSONHandler(os.Stdout, nil))
    err := server.Initialize(logger);

    if err != nil {
        logger.Error("we are cooked")
        return
    }


    err = server.Start()
    if err != nil {
        logger.Error("WE ARE SO COOKED")
        return
    }

}
