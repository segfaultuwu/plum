use x86_64::structures::idt::InterruptStackFrame;

pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    unsafe {
        x86_64::instructions::port::Port::<u8>::new(0x20).write(0x20);
    }

    crate::multitasking::schedule();
}
