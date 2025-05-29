use crossbeam::channel::{Receiver, Sender, select_biased};
use rodio::{Decoder, OutputStream, Sink};
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::Arc;
use std::fs::File;
use tracing;

use crate::shared::{Command, CommandTypes, Response, Signal};
use crate::queue::Queue;


pub struct PlayerContext {
    queue: Queue,
    sink: Arc<Sink>,
    _stream: OutputStream,
    is_playing: bool,
    ready: Signal,
    receiver: Receiver<(Command, Sender<Response>)>,
}

impl PlayerContext {
    pub fn new(receiver: Receiver<(Command, Sender<Response>)>) -> Self {
        let ready = Signal::new();
        let (stream, handle) = OutputStream::try_default().unwrap();
        let sink = Arc::new(Sink::try_new(&handle).unwrap());

        Self {
            queue: Queue::new(),
            sink,
            _stream: stream,
            is_playing: false,
            ready,
            receiver,
        }
    }

    pub fn start_player(&mut self) {
        loop {
            select_biased!{
                recv(self.receiver) -> packet => {
                    self.interpret_packet(packet.unwrap());
                },
                recv(self.ready.rx) -> _ => {
                    if self.is_playing {
                        self.queue.move_next();
                    }

                    self.play_file();
                },
            }
        }
    }
    
    fn play_file(&mut self) {
        let next = match self.queue.peek() {
            Some(path) => path,
            None => {
                tracing::info!("played every song in the queue");
                if self.is_playing {
                    self.is_playing = false;
                }
                return;
            }
        };

        let file = match File::open(next.clone()) {
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
        let cloned_sink = self.sink.clone();
        let ready_tx_clone = self.ready.tx.clone();
        std::thread::spawn(move || {
            cloned_sink.sleep_until_end();
            let _ = ready_tx_clone.send(());
        });

        if !self.is_playing {
            self.is_playing = true;
        }
    }

    fn interpret_packet(&mut self, packet: (Command, Sender<Response>)) {
        let (command, tx) = packet;
        match command.command {
            CommandTypes::Skip => self.skip(tx),
            CommandTypes::Clear => self.clear(tx),
            CommandTypes::Pause => self.pause(tx),
            CommandTypes::Resume => self.resume(tx),
            CommandTypes::Shutdown => {}
            CommandTypes::AddQueue => self.add_queue(command, tx),
            CommandTypes::LoopSong => self.loop_song(tx),
            CommandTypes::LoopQueue => self.loop_queue(tx),
            CommandTypes::SetVolume => {},
            CommandTypes::AddPlaylist => {}
        }
    }

    fn skip(&mut self, tx: Sender<Response>) {
        if !self.is_playing {
            let _ = tx.send(Response::err("no song is currently playing"));
            return;
        }
        
        // this causes start_player to move to the next song on its own
        self.sink.stop();
        let _ = tx.send(Response::ok());
    }

    fn clear(&mut self, tx: Sender<Response>) {
        self.queue.clear();
        if self.is_playing {
            self.sink.stop();
            self.is_playing = false
        }

        let _ = tx.send(Response::ok());
    }

    fn pause(&self, tx: Sender<Response>) {
        if !self.is_playing {
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

    fn add_queue(&mut self, command: Command, tx: Sender<Response>) {
        if !command.validate_payload() {
            let _ = tx.send(Response::err("ADD_QUEUE is missing its payload"));
            return;
        }

        let path = command.payload.unwrap();
        self.queue.append(PathBuf::from(path));
        let _ = tx.send(Response::ok());
        if !self.is_playing {
            let _ = self.ready.tx.send(());
        }
    }

    fn loop_song(&mut self, tx: Sender<Response>) {
        if !self.is_playing {
            let _ = tx.send(Response::err("no song is currently playing"));
            return;
        }

        self.queue.loop_song();
        let _ = tx.send(Response::ok());
    }

    fn loop_queue(&mut self, tx: Sender<Response>) {
        if self.queue.is_empty() {
            let _ = tx.send(Response::err("the queue is currently empty, nothing to loop"));
            return;
        }

        self.queue.loop_queue();
        let _ = tx.send(Response::ok());
    }
}
