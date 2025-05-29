use crossbeam::channel::{bounded, Sender};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use tracing;

use crate::shared::{Command, Response};

pub struct TcpContext {
    transmitter: Sender<(Command, Sender<Response>)>,
    listener: TcpListener,
}

impl TcpContext {
    pub fn new(transmitter: Sender<(Command, Sender<Response>)>) -> Option<Self> {
        let listener = match bind_in_range() {
            Some(l) => l,
            None => {
                tracing::error!("grapevined server failed to bind in port range 6990-7000");
                return None;
            }
        };

        Some(Self {
            transmitter,
            listener,
        })
    }

    pub fn start_listener(&self) {
        for connection in self.listener.incoming() {
            match connection {
                Ok(connection) => {
                    let cloned_transmitter = self.transmitter.clone();
                    std::thread::spawn(move || {
                        conn_helper(connection, cloned_transmitter);
                    });
                }

                Err(_) => continue,
            }
        }
    }
}

fn bind_in_range() -> Option<TcpListener> {
    for port in 6990..=7000 {
        let addr = format!("127.0.0.1:{}", port);
        if let Ok(listener) = TcpListener::bind(&addr) {
            tracing::info!("grapevined server bound to {}", addr);
            return Some(listener);
        }
    }

    None
}

fn conn_helper(mut conn: TcpStream, tx: Sender<(Command, Sender<Response>)>) {
    let mut buffer = [0; 1024];
    let size = match conn.read(&mut buffer) {
        Ok(s) => s,
        Err(_) => {
            tracing::error!("failed to read from incomming connection");
            return;
        }
    };

    let packet: Command = match serde_json::from_slice(&buffer[..size]) {
        Ok(p) => p,
        Err(_) => {
            tracing::error!("could not deserialze incoming packet");
            return;
        }
    };

    tracing::info!("received {:?} packet", &packet.command);

    // I know crossbeam is MPMC but I'd rather stay with MPSC for now
    let (resp_tx, resp_rx) = bounded::<Response>(1);
    if tx.send((packet, resp_tx)).is_err() {
        tracing::error!("failed to send command to the music thread");
        return;
    }

    let response = match resp_rx.recv() {
        Ok(r) => r,
        Err(_) => Response::err("music thread failed to return a response"),
    };

    if let Ok(payload) = serde_json::to_vec(&response) {
        let _ = conn.write_all(&payload);
    }
}
