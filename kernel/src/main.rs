#![no_std]
#![no_main]

use bootloader_api::{BootInfo, BootloaderConfig, entry_point};
use core::panic::PanicInfo;

mod drivers;
mod utils;

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.frame_buffer.minimum_framebuffer_width = Some(800);
    config.frame_buffer.minimum_framebuffer_height = Some(600);
    config
};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    drivers::serial::init();
    drivers::serial::write("Plum booted!\n");
    let fb = boot_info.framebuffer.as_mut().unwrap();
    let mut framebuffer = drivers::graphics::framebuffer::Framebuffer::from_bootloader(fb);
    framebuffer.clear(0x1E1E2E);
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
