package server

import (
    "fmt"
    "net"
    "errors"
    "log/slog"
    "encoding/json"
)

type Command struct {
    Command     string  `json:"command"`
    Payload     *string `json:"payload,omitempty"`
}

type Response struct {
    Status      string `json:"STATUS"`
    ErrMsg      string `json:"ERRMSG,omitempty"`
}

type TcpServer struct {
    log         *slog.Logger
    listener    net.Listener
    // send `Command` packets to music thread
    interop     chan <- Command
    // receive `Response` packets from music thread
    response    <- chan Response
}

var (
    tcp *TcpServer
    active  = false
)

// --- Core Functions ---

// Initializes the tcp server with the required channels
func Initialize(logger *slog.Logger, cmd chan <- Command, resp <- chan Response) error {
    if tcp != nil {
        return errors.New("tcp server already initialized")
    }

    tcp = &TcpServer{
        log:        logger,
        interop:    cmd,
        response:   resp,
        listener:   nil,    // Initialize this in `Start`
    }

    return nil
}


// Main TCP thread. Attempt to create a TCP socket and keep it running
func Start() error {
    if tcp == nil {
        return errors.New("please initialize the tcp server first")
    }

    if active {
        tcp.log.Warn("tcp server received a Start request somehow")
        return errors.New("tcp server is already running")
    }

    var (
        err  error
        addr string
    )
    for port := 6990; port <= 7000; port++ {
        addr = fmt.Sprintf("127.0.0.1:%d", port)
        tcp.log.Info("attempting to create a connection", "addr", addr)
        tcp.listener, err = net.Listen("tcp", addr)
        if err == nil {
            tcp.log.Info("created TCP server", "addr", addr)
            break
        }
    }

    // Failed to create a TCP socket in the port range
    if tcp.listener == nil {
        tcp.log.Error("failed to create a TCP socket in the port range 6990 - 7000")
        return errors.New("no suitable port found in the range 6990-7000")
    }

    active = true
    listen()
    return nil
}

// Listens for incomming connections and passes them along to a handler
func listen() {
    defer func() {
        // tcp.listener.Close() was here but removed since "SHUTDOWN" already
        // handles tcp.listener.Close(), nvm might as well keep it
        tcp.listener.Close()

        // Honestly this does nothing as the app stops if the TCP server stops
        // but might as well for consistency
        active = false
    }()

    for {
        conn, err := tcp.listener.Accept()
        if err != nil {
            tcp.log.Info("the TCP server has been stopped")
            return
        }

        go helper(conn)
    }
}

// --- Helper Functions ---

// Validates that the received command is a valid grapevined command
func validate(cmd Command) bool {
    switch cmd.Command {
    case
        "SKIP",
        "CLEAR",
        "PAUSE",            // reuse for resuming as well
        "SHUTDOWN",
        "LOOP_SONG",        // only loop the currently playing song
        "ADD_QUEUE",
        "SET_VOLUME",
        "ADD_PLAYLIST",
        "LOOP_PLAYLIST":    // loop the entire playlist
        return true
    }
    return false
}

// Handles incoming connections and respond with a Response object
func helper(conn net.Conn) {
    defer conn.Close()

    var (
        decoder = json.NewDecoder(conn)
        cmd       Command
        resp      Response
    )

    err := decoder.Decode(&cmd)
    if err != nil {
        tcp.log.Warn("failed to decode packet", "err", err)
        return
    }

    if !validate(cmd) {
        tcp.log.Warn("received unknown command", "cmd", cmd.Command)
        return
    }

    // This one needs to be handled by the TCP (main) thread
    if cmd.Command == "SHUTDOWN" {
        tcp.listener.Close()
        return
    }

    tcp.log.Info("received command", "cmd", cmd.Command)
    tcp.interop <- cmd

    // await for response from music thread
    resp = <- tcp.response
    payload, err := json.Marshal(resp)
    if err != nil {
        tcp.log.Error("failed to marshall response, gvctl may timeout or hang", "err", err)
        return
    }

    payload = append(payload, '\n')
    _, err = conn.Write(payload)
    if err != nil {
        tcp.log.Error("failed to write entire packet to socket", "err", err)
        return
    }
}
