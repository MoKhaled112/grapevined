use axum::{
    extract::State,
    http::Method,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;
use std::{env, net::SocketAddr};
use tower_http::cors::{Any, CorsLayer};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum CommandTypes {
    Skip,
    Clear,
    Pause,
    Resume,
    Shutdown,
    AddQueue,
    LoopSong,
    LoopQueue,
    AddPlaylist,
    Status, // optional; daemon may respond ERR if unsupported
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Command {
    command: CommandTypes,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Response {
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    errmsg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
}

#[derive(Clone)]
struct AppState {
    daemon_addr: String,
}

#[tokio::main]
async fn main() {
    // Discover daemon addr: env or scan 6990..7000
    let daemon_addr = env::var("GRAPEVINED_ADDR")
        .unwrap_or_else(|_| discover_daemon().unwrap_or_else(|| "127.0.0.1:6990".to_string()));

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/ping", get(ping))
        .route("/api/status", get(status))
        .route("/api/skip", post(skip))
        .route("/api/clear", post(clear))
        .route("/api/pause", post(pause))
        .route("/api/resume", post(resume))
        .route("/api/shutdown", post(shutdown))
        .route("/api/loop/song", post(loop_song))
        .route("/api/loop/queue", post(loop_queue))
        .route("/api/queue", post(add_queue))
        .route("/api/playlist", post(add_playlist))
        .with_state(AppState { daemon_addr })
        .layer(cors);

    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    println!("HTTP gateway on http://{addr}");
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

async fn ping(State(state): State<AppState>) -> Json<Response> {
    Json(Response {
        status: "OK".into(),
        errmsg: None,
        data: Some(serde_json::json!({ "daemon": state.daemon_addr })),
    })
}

async fn status(State(state): State<AppState>) -> Json<Response> {
    send_cmd(&state.daemon_addr, Command { command: CommandTypes::Status, payload: None }).await
}
async fn skip(State(state): State<AppState>) -> Json<Response> {
    send_cmd(&state.daemon_addr, Command { command: CommandTypes::Skip, payload: None }).await
}
async fn clear(State(state): State<AppState>) -> Json<Response> {
    send_cmd(&state.daemon_addr, Command { command: CommandTypes::Clear, payload: None }).await
}
async fn pause(State(state): State<AppState>) -> Json<Response> {
    send_cmd(&state.daemon_addr, Command { command: CommandTypes::Pause, payload: None }).await
}
async fn resume(State(state): State<AppState>) -> Json<Response> {
    send_cmd(&state.daemon_addr, Command { command: CommandTypes::Resume, payload: None }).await
}
async fn shutdown(State(state): State<AppState>) -> Json<Response> {
    send_cmd(&state.daemon_addr, Command { command: CommandTypes::Shutdown, payload: None }).await
}
async fn loop_song(State(state): State<AppState>) -> Json<Response> {
    send_cmd(&state.daemon_addr, Command { command: CommandTypes::LoopSong, payload: None }).await
}
async fn loop_queue(State(state): State<AppState>) -> Json<Response> {
    send_cmd(&state.daemon_addr, Command { command: CommandTypes::LoopQueue, payload: None }).await
}

#[derive(Deserialize)]
struct PathBody { path: String }

async fn add_queue(State(state): State<AppState>, Json(body): Json<PathBody>) -> Json<Response> {
    send_cmd(&state.daemon_addr, Command { command: CommandTypes::AddQueue, payload: Some(body.path) }).await
}
async fn add_playlist(State(state): State<AppState>, Json(body): Json<PathBody>) -> Json<Response> {
    send_cmd(&state.daemon_addr, Command { command: CommandTypes::AddPlaylist, payload: Some(body.path) }).await
}

async fn send_cmd(addr: &str, cmd: Command) -> Json<Response> {
    let payload = serde_json::to_vec(&cmd).unwrap();
    let resp = tokio::task::spawn_blocking({
        let addr = addr.to_string();
        move || {
            let mut stream = TcpStream::connect(addr)?;
            stream.set_read_timeout(Some(Duration::from_secs(3)))?;
            stream.write_all(&payload)?;
            let mut buf = Vec::with_capacity(1024);
            stream.read_to_end(&mut buf)?;
            let r: Response = serde_json::from_slice(&buf)?;
            Ok::<_, std::io::Error>(r)
        }
    })
    .await
    .ok()
    .and_then(Result::ok)
    .unwrap_or(Response { status: "ERR".into(), errmsg: Some("gateway failed to reach daemon".into()), data: None });

    Json(resp)
}

fn discover_daemon() -> Option<String> {
    for port in 6990..=7000 {
        let addr = format!("127.0.0.1:{port}");
        if TcpStream::connect(&addr).is_ok() {
            return Some(addr);
        }
    }
    None
}

