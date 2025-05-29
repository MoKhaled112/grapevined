use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub struct Queue {
    deque: VecDeque<PathBuf>,
    index: usize,
    loop_curr: bool,
    loop_all: bool,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            deque: VecDeque::new(),
            index: 0,
            loop_curr: false,
            loop_all: false,
        }
    }

    pub fn peek(&self) -> Option<PathBuf> {
        if let Some(next) = self.deque.get(self.index) {
            return Some(next.clone());
        }

        None
    }

    pub fn remove_curr(&mut self) {
        let _removed = self.deque.remove(self.index);
    }

    pub fn is_empty(&self) -> bool {
        self.deque.is_empty()
    }

    pub fn clear(&mut self) {
        self.deque.clear();
        self.index = 0;
        self.loop_curr = false;
        self.loop_all = false;
    }

    pub fn append(&mut self, item: PathBuf) {
        self.deque.push_back(item);
    }

    pub fn loop_song(&mut self) {
        self.loop_curr = !self.loop_curr
    }

    pub fn loop_queue(&mut self) {
        self.loop_all = !self.loop_all
    }

    pub fn move_next(&mut self) {
        if self.loop_curr {
            return;
        }

        if self.loop_all {
            self.index += 1;
            if self.index >= self.deque.len() {
                self.index = 0
            }

            return;
        }

        let _next = self.deque.remove(self.index);
        if self.index >= self.deque.len() {
            self.index = 0;
        }
    }

    pub fn load_m3u(&mut self, path: PathBuf) -> Option<usize> {
        // check if it is a .m3u file
        if !path.extension().map(|s| s == "m3u").unwrap_or(false) {
            tracing::warn!("{} is not a .m3u file", path.display());
            return None;
        }

        let file = match File::open(path.clone()) {
            Ok(f) => f,
            Err(_) => {
                tracing::warn!("failed to open {}", path.display());
                return None;
            }
        };

        let reader = BufReader::new(file);
        let mut size = 0;

        for line in reader.lines() {
            let curr = line.unwrap().trim().to_string();
            if curr.is_empty() || curr.starts_with("#") {
                // ignore empty lines and #EXTM3U/#EXTINFO
                continue;
            }

            size += 1;
            self.deque.push_back(PathBuf::from(curr));
        }

        Some(size)
    }
}
