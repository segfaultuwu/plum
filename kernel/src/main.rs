#![no_std]
#![no_main]

extern crate alloc;

use crate::memory::allocator::{BumpAllocator, Locked};
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use bootloader_api::{BootInfo, entry_point};
use core::panic::PanicInfo;

mod drivers;
mod memory;
mod utils;

entry_point!(kernel_main);

#[global_allocator]
static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    const HEAP_START: usize = 0x_4444_4444_0000;
    const HEAP_SIZE: usize = 100 * 1024;

    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }
    drivers::serial::write("Plum booted!\n");
    let fb = boot_info.framebuffer.as_mut().unwrap();
    let mut framebuffer = drivers::graphics::framebuffer::Framebuffer::from_bootloader(fb);
    framebuffer.clear(0x1E1E2E);
    let mut v = Vec::new();
    v.push(1);
    v.push(2);

    let s = String::from("PlumOS");

    let b = Box::new(42);
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut vga = drivers::graphics::vga::VGA::new(
        utils::colors::Colors::Red.to_vga(),
        utils::colors::Colors::Surface0.to_vga(),
    );
    let location = info.location().unwrap();
    vga.write_line("Kernel panic!");
    vga.write_string("Location: ");
    vga.write_string(location.file());
    vga.write_string(":");
    crate::utils::write_usize(&mut vga, location.line() as usize);

    loop {}
}
