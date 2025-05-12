package server

import (
    "encoding/json"
    "log/slog"
    "errors"
    "net"
    "fmt"
)

type Command struct {
    Command     string  `json:"command"`
    Payload     *string `json:"payload,omitempty"`
}

type TcpListener struct {
    log         *slog.Logger
    active      bool
    stop        chan struct{}          // stop the TCP Server via a channel
    listener    net.Listener
}

var tcpserver *TcpListener

func Initialize(logger *slog.Logger) error {
    if tcpserver != nil {
        logger.Error("TcpListener is already initialized.")
        return errors.New("tcp server already initialized")
    }

    tcpserver = &TcpListener{
        log:        logger,
        active:     false,
        stop:       make(chan struct{}),
        listener:   nil,
    }

    return nil
}

func Start() error {
    if tcpserver.active {
        tcpserver.log.Error("TcpListener is already running.")
        return errors.New("tcp server is already running")
    }

    var err error
    for port := 6990; port <= 7000; port++ {
        address := fmt.Sprintf("127.0.0.1:%d", port)
        tcpserver.log.Info("attempting to create TCP server", "address", address)

        tcpserver.listener, err = net.Listen("tcp", address)
        if err == nil {
            tcpserver.log.Info("established TCP server", "address", address)
            break
        }
    }

    if tcpserver.listener == nil {
        tcpserver.log.Error("failed to create TCP server")
        return errors.New("no suitable TCP port found")
    }
   
    tcpserver.listen()
    return nil
}

func (tl *TcpListener) listen() {
    tl.active = true
    defer func() {
        tl.listener.Close()
        tl.active = false
    }()

    for {
        connection, err := tl.listener.Accept()
        if err != nil {
            select {
            case <-tl.stop:
                tl.log.Info("stopped the TCP server.")
                return
            default:
                tl.log.Error("an outside source closed the TCP server.")
                return
            }
        }

        go tl.handler(connection)
    }
}

func (tl *TcpListener) handler(conn net.Conn) {
    defer conn.Close()

    var (
        decoder = json.NewDecoder(conn)
        cmd     Command
    )

    err := decoder.Decode(&cmd)
    if err != nil {
        tl.log.Error("could not decode payload", "err", err)
        return
    }

    tl.log.Info("received command", "cmd", cmd.Command)
    switch cmd.Command {
    case "PLAY_FILE":
        // TODO: call play() function
    case "PLAY_PLAYLIST":
        // TODO: call playlist() function
    case "PAUSE":
        // TODO: call pause() function -- reuse for pause/resume
    case "SHUTDOWN":
        close(tl.stop)
        tl.listener.Close()
    default:
        tl.log.Warn("received unknown command", "cmd", cmd.Command)
    }
}
