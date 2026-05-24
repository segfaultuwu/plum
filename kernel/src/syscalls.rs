use x86_64::structures::idt::InterruptStackFrame;

#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyscallNumber {
    Write = 0,
    ReadLine = 1,
    ClearScreen = 2,
    ProcessList = 3,
    Yield = 4,
    Exit = 5,
    ProcessCount = 6,
    CurrentPid = 7,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SyscallArgs {
    pub arg0: u64,
    pub arg1: u64,
    pub arg2: u64,
    pub arg3: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyscallError {
    Unsupported,
    InvalidArgument,
}

pub fn dispatch(number: SyscallNumber, args: SyscallArgs) -> Result<u64, SyscallError> {
    match number {
        SyscallNumber::Yield => {
            crate::multitasking::schedule();
            Ok(0)
        }
        SyscallNumber::Exit => {
            let pid = args.arg0 as u32;
            let mut scheduler = crate::multitasking::SCHEDULER.lock();
            if let Some(manager) = scheduler.as_mut() {
                manager.terminate_process(pid);
                Ok(0)
            } else {
                Err(SyscallError::Unsupported)
            }
        }
        SyscallNumber::ProcessCount => {
            let scheduler = crate::multitasking::SCHEDULER.lock();
            let count = scheduler
                .as_ref()
                .map(|manager| manager.processes.len() as u64)
                .unwrap_or(0);
            Ok(count)
        }
        SyscallNumber::CurrentPid => {
            let scheduler = crate::multitasking::SCHEDULER.lock();
            let pid = scheduler
                .as_ref()
                .map(|manager| manager.current_pid as u64)
                .unwrap_or(0);
            Ok(pid)
        }
        SyscallNumber::Write
        | SyscallNumber::ReadLine
        | SyscallNumber::ClearScreen
        | SyscallNumber::ProcessList => {
            let _ = args;
            Err(SyscallError::Unsupported)
        }
    }
}

pub fn log_interrupt(stack_frame: &InterruptStackFrame) {
    crate::println!("SYSCALL\n{:#?}", stack_frame);
}
