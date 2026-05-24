use x86_64::structures::idt::InterruptStackFrame;

pub extern "x86-interrupt" fn syscall_handler(stack_frame: InterruptStackFrame) {
    crate::syscalls::log_interrupt(&stack_frame);
}
