use std::collections::VecDeque;
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
        let _removed =  self.deque.remove(self.index);
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
            return
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
}

