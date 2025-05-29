use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::writer::BoxMakeWriter;
use crossbeam::channel::{Sender, unbounded};
use tracing;
use dirs;


mod shared;
mod server;

use crate::shared::{Command, Response};
use crate::server::TcpContext;

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
            return
        }
    };

    std::thread::spawn(move || {
        tcp_context.start_listener();
    });
    tracing::info!("started TCP server");
}
