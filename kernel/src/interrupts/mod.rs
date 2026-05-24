use spin::Once;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use x86_64::PrivilegeLevel;

pub mod pic;
pub mod syscall;
pub mod timer;

static IDT_ONCE: Once<InterruptDescriptorTable> = Once::new();

fn idt_init() -> InterruptDescriptorTable {
    let mut idt = InterruptDescriptorTable::new();

    idt.breakpoint.set_handler_fn(breakpoint_handler);
    idt.page_fault.set_handler_fn(page_fault_handler);

    idt[0x20].set_handler_fn(timer::timer_interrupt_handler);

    idt[0x80]
        .set_handler_fn(syscall::syscall_handler)
        .set_privilege_level(PrivilegeLevel::Ring3);

    idt
}

pub fn init_idt() {
    pic::init_pic();
    let idt = IDT_ONCE.call_once(idt_init);
    idt.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    crate::println!("BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: x86_64::structures::idt::PageFaultErrorCode,
) {
    crate::println!("PAGE FAULT: {:?}\n{:#?}", error_code, stack_frame);
    loop {}
}
