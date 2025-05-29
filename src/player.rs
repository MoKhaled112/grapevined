use crossbeam::channel::{select_biased, tick, Receiver, Sender};
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tracing;

use crate::queue::Queue;
use crate::shared::{Command, CommandTypes, Response, Signal};

static IS_PLAYING: AtomicBool = AtomicBool::new(false);

pub struct PlayerContext {
    queue: Queue,
    receiver: Receiver<(Command, Sender<Response>)>,
    sink: Arc<Sink>,
    stop: Signal,
    ticker: Receiver<Instant>,
    _stream: OutputStream,
}

impl PlayerContext {
    pub fn new(receiver: Receiver<(Command, Sender<Response>)>) -> Self {
        let stop = Signal::new();
        let queue = Queue::new();
        let (stream, handle) = OutputStream::try_default().unwrap();
        let sink = Arc::new(Sink::try_new(&handle).unwrap());
        let ticker = tick(std::time::Duration::from_millis(250));

        Self {
            queue,
            receiver,
            sink,
            stop,
            ticker,
            _stream: stream,
        }
    }

    pub fn start_player(&mut self) {
        loop {
            select_biased! {
                recv(self.receiver) -> packet => self.read_packet(packet.unwrap()),
                recv(self.stop.rx) -> _ => break,
                recv(self.ticker) -> _ => {
                    if !IS_PLAYING.load(Ordering::SeqCst) && !self.queue.is_empty() {
                        self.play_file();
                        self.queue.move_next();
                    }
                }
            }
        }
    }

    fn play_file(&mut self) {
        let next = match self.queue.peek() {
            Some(path) => path,
            None => return,
        };

        let file = match File::open(PathBuf::from(next.clone())) {
            Ok(f) => f,
            Err(_) => {
                tracing::warn!("failed to open {}, removing from queue", next.display());
                self.queue.remove_curr();
                return;
            }
        };

        let decoder = match Decoder::new(BufReader::new(file)) {
            Ok(d) => d,
            Err(_) => {
                tracing::warn!("failed to decode {}, removing from queue", next.display());
                self.queue.remove_curr();
                return;
            }
        };

        self.sink.append(decoder);
        IS_PLAYING.store(true, Ordering::SeqCst);

        let cloned = self.sink.clone();
        std::thread::spawn(move || {
            cloned.sleep_until_end();
            IS_PLAYING.store(false, Ordering::SeqCst);
        });
    }

    fn read_packet(&mut self, packet: (Command, Sender<Response>)) {
        let (command, tx) = packet;
        match command.command {
            CommandTypes::Skip => self.skip(tx),
            CommandTypes::Clear => self.clear(tx),
            CommandTypes::Pause => self.pause(tx),
            CommandTypes::Resume => self.resume(tx),
            CommandTypes::Shutdown => self.shutdown(tx),
            CommandTypes::AddQueue => self.add_queue(command, tx),
            CommandTypes::LoopSong => self.loop_song(tx),
            CommandTypes::LoopQueue => self.loop_queue(tx),
            CommandTypes::AddPlaylist => self.add_playlist(command, tx),
        }
    }

    fn skip(&self, tx: Sender<Response>) {
        if !IS_PLAYING.load(Ordering::SeqCst) {
            let _ = tx.send(Response::err("no song is currently playing"));
            return;
        }

        self.sink.stop();
        let _ = tx.send(Response::ok());
    }

    fn clear(&mut self, tx: Sender<Response>) {
        self.queue.clear();
        if IS_PLAYING.load(Ordering::SeqCst) {
            self.sink.stop();
        }

        let _ = tx.send(Response::ok());
    }

    fn pause(&self, tx: Sender<Response>) {
        if !IS_PLAYING.load(Ordering::SeqCst) {
            let _ = tx.send(Response::err("no song is currently playing"));
            return;
        }

        if self.sink.is_paused() {
            let _ = tx.send(Response::err("the current song is already paused"));
            return;
        }

        self.sink.pause();
        let _ = tx.send(Response::ok());
    }

    fn resume(&self, tx: Sender<Response>) {
        // according to the rodio docs this does nothing if the sink is not
        // paused so... it'll do for now I guess

        self.sink.play();
        let _ = tx.send(Response::ok());
    }

    fn shutdown(&mut self, tx: Sender<Response>) {
        self.queue.clear();
        self.sink.stop();
        let _ = self.stop.tx.send(());
        let _ = tx.send(Response::ok());
    }

    fn add_queue(&mut self, command: Command, tx: Sender<Response>) {
        if !command.validate_payload() {
            let _ = tx.send(Response::err("ADD_QUEUE packet is missing its payload"));
            return;
        }

        let path = command.payload.unwrap();
        self.queue.append(PathBuf::from(path));
        let _ = tx.send(Response::ok());
    }

    fn loop_song(&mut self, tx: Sender<Response>) {
        if !IS_PLAYING.load(Ordering::SeqCst) {
            let _ = tx.send(Response::err("no song is currently playing"));
            return;
        }

        self.queue.loop_song();
        let _ = tx.send(Response::ok());
    }

    fn loop_queue(&mut self, tx: Sender<Response>) {
        if self.queue.is_empty() {
            let _ = tx.send(Response::err(
                "the queue is currently empty, nothing to loop",
            ));
            return;
        }

        self.queue.loop_queue();
        let _ = tx.send(Response::ok());
    }

    fn add_playlist(&mut self, command: Command, tx: Sender<Response>) {
        if !command.validate_payload() {
            let _ = tx.send(Response::err("ADD_PLAYLIST packet is missing its payload"));
            return;
        }

        self.queue.clear();
        if IS_PLAYING.load(Ordering::SeqCst) {
            self.sink.stop();
        }

        let path = command.payload.unwrap();
        let size = match self.queue.load_m3u(PathBuf::from(path.clone())) {
            Some(s) => s,
            None => {
                let _ = tx.send(Response::err(
                    "failed to process playlist file, check your logs for more information",
                ));
                return;
            }
        };

        tracing::info!("added {} items to the queue from {}", size, path);
        let _ = tx.send(Response::ok());
    }
}
