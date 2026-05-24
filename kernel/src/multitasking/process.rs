use alloc::string::String;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ProcessState {
    Ready,
    Running,
    Sleeping,
    Blocked,
    Terminated,
}

#[derive(Clone, Copy)]
pub struct Registers {
    pub rip: u64,
    pub rsp: u64,

    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,

    pub rsi: u64,
    pub rdi: u64,

    pub rbp: u64,

    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
}

pub struct Process {
    pub pid: u32,
    pub name: String,

    pub state: ProcessState,

    pub regs: Registers,

    pub stack_start: u64,
    pub stack_size: usize,

    pub privilege_level: u8, // 0 = Ring 0, 3 = Ring 3
}
