use crossbeam::channel::{unbounded, Receiver, Sender};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CommandTypes {
    Skip,
    Clear,
    Pause,
    Resume,
    Shutdown,
    AddQueue,
    LoopSong,
    LoopQueue,
    SetVolume,
    AddPlaylist,
}

#[derive(Deserialize, Debug)]
pub struct Command {
    pub command: CommandTypes,
    pub payload: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct Response {
    #[serde(rename = "STATUS")]
    pub status: String,
    #[serde(rename = "ERRMSG")]
    pub errmsg: Option<String>,
}

pub struct Signal {
    pub tx: Sender<()>,
    pub rx: Receiver<()>,
}

impl Signal {
    pub fn new() -> Self {
        let (tx, rx) = unbounded::<()>();
        Self { tx, rx }
    }
}

impl Command {
    /// Validates the `payload` field of a Command object for ADD_QUEUE, ADD_PLAYLIST,
    /// and SET_VOLUME. If the command is not one of these 3, it returns true
    pub fn validate_payload(&self) -> bool {
        matches!(
            self.command,
            CommandTypes::AddPlaylist | CommandTypes::AddQueue | CommandTypes::SetVolume
        )
        .then(|| self.payload.is_some())
        .unwrap_or(true)
    }
}

impl Response {
    pub fn ok() -> Self {
        Self {
            status: String::from("OK"),
            errmsg: None,
        }
    }

    pub fn err(message: &str) -> Self {
        Self {
            status: String::from("ERR"),
            errmsg: Some(message.to_string()),
        }
    }
}
