#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod drivers;
mod utils;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut vga = drivers::graphics::vga::VGA::new(
        utils::colors::Colors::Text.to_vga(),
        utils::colors::Colors::Surface0.to_vga(),
    );
    vga.clear();
    vga.write_line("Hello, World!");
    vga.write_line("Hello, World!");
    vga.write_line("Hello, World!");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let mut vga = drivers::graphics::vga::VGA::new(
        utils::colors::Colors::Red.to_vga(),
        utils::colors::Colors::Surface0.to_vga(),
    );
    vga.write_string("Kernel panic!");
    loop {}
}
