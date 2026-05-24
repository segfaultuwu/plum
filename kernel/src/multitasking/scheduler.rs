use super::process::Process;
use alloc::vec::Vec;

pub struct Scheduler {
    ready_queue: Vec<usize>,
    current_index: usize,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            ready_queue: Vec::new(),
            current_index: 0,
        }
    }

    pub fn add_process(&mut self, pid: usize) {
        self.ready_queue.push(pid);
    }

    pub fn remove_process(&mut self, pid: usize) {
        self.ready_queue.retain(|&p| p != pid);
    }

    pub fn next_pid(&mut self) -> Option<usize> {
        if self.ready_queue.is_empty() {
            return None;
        }

        self.current_index = (self.current_index + 1) % self.ready_queue.len();
        Some(self.ready_queue[self.current_index])
    }
}
