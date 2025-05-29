use crossbeam::channel::{unbounded, Sender};
use dirs;
use tracing;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::writer::BoxMakeWriter;

mod player;
mod queue;
mod server;
mod shared;

use crate::player::PlayerContext;
use crate::server::TcpContext;
use crate::shared::{Command, Response};

fn create_log_appender() -> RollingFileAppender {
    let logs_dir = dirs::config_dir().unwrap().join("grapevined").join("logs");
    std::fs::create_dir_all(&logs_dir).expect("Could not create logging directory");
    RollingFileAppender::new(Rotation::DAILY, logs_dir, "log")
}

fn main() {
    let appender = create_log_appender();
    let (non_blocking, _guard) = tracing_appender::non_blocking(appender);
    tracing_subscriber::fmt()
        .with_writer(BoxMakeWriter::new(non_blocking))
        .with_ansi(false)
        .init();

    let (transmitter, receiver) = unbounded::<(Command, Sender<Response>)>();
    let tcp_context = match TcpContext::new(transmitter) {
        Some(context) => context,
        None => {
            tracing::error!("terminating grapevined due to server failing to bind");
            return;
        }
    };

    std::thread::spawn(move || {
        tcp_context.start_listener();
    });
    tracing::info!("started TCP server");

    let mut player_context = PlayerContext::new(receiver);
    tracing::info!("starting music thread");
    player_context.start_player();
    tracing::info!("grapevined has shutdown");
}
