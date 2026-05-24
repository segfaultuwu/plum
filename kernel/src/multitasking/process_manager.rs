use crate::multitasking::process::{Process, ProcessState, Registers};
use crate::multitasking::scheduler::Scheduler;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ProcessManager {
    pub processes: Vec<Process>,
    pub current_pid: u32,
    pub next_pid: u32,
    scheduler: Scheduler,
}

impl ProcessManager {
    pub fn new() -> Self {
        ProcessManager {
            processes: Vec::new(),
            current_pid: 0,
            next_pid: 1,
            scheduler: Scheduler::new(),
        }
    }

    pub fn create_process(
        &mut self,
        name: &str,
        entry_point: u64,
        stack_start: u64,
        stack_size: usize,
        privilege_level: u8,
    ) -> u32 {
        let pid = self.next_pid;
        self.next_pid += 1;

        let process = Process {
            pid,
            name: String::from(name),
            state: ProcessState::Ready,
            regs: Registers {
                rip: entry_point,
                rsp: stack_start + stack_size as u64,
                rax: 0,
                rbx: 0,
                rcx: 0,
                rdx: 0,
                rsi: 0,
                rdi: 0,
                rbp: 0,
                r8: 0,
                r9: 0,
                r10: 0,
                r11: 0,
                r12: 0,
                r13: 0,
                r14: 0,
                r15: 0,
            },
            stack_start,
            stack_size,
            privilege_level,
        };

        self.processes.push(process);
        self.scheduler.add_process((pid - 1) as usize);

        pid
    }

    pub fn schedule(&mut self) {
        if let Some(next_pid_index) = self.scheduler.next_pid() {
            if next_pid_index < self.processes.len() {
                // Mark current process as ready if it's running
                if self.current_pid > 0 && ((self.current_pid - 1) as usize) < self.processes.len()
                {
                    let current_idx = (self.current_pid - 1) as usize;
                    if self.processes[current_idx].state == ProcessState::Running {
                        self.processes[current_idx].state = ProcessState::Ready;
                    }
                }

                // Set the next process as running
                self.current_pid = self.processes[next_pid_index].pid;
                self.processes[next_pid_index].state = ProcessState::Running;
            }
        }
    }

    pub fn get_current_process(&self) -> Option<&Process> {
        self.processes.iter().find(|p| p.pid == self.current_pid)
    }

    pub fn get_current_process_mut(&mut self) -> Option<&mut Process> {
        self.processes
            .iter_mut()
            .find(|p| p.pid == self.current_pid)
    }

    pub fn terminate_process(&mut self, pid: u32) {
        if let Some(process) = self.processes.iter_mut().find(|p| p.pid == pid) {
            process.state = ProcessState::Terminated;
            self.scheduler.remove_process((pid - 1) as usize);
        }
    }
}
