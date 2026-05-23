#![no_std]
#![no_main]
#![allow(static_mut_refs)]

extern crate alloc;

use crate::{
    memory::allocator::{BumpAllocator, Locked},
    terminal::framebuffer_terminal::Terminal,
};
use bootloader_api::info::MemoryRegions;
use bootloader_api::{entry_point, BootInfo};
use core::fmt::Write;
use core::{fmt, panic::PanicInfo};
use spin::Mutex;
use x86_64::{
    structures::paging::{FrameAllocator, Page, PageTableFlags},
    VirtAddr,
};

mod drivers;
mod memory;
mod terminal;
mod utils;

#[macro_use]
mod macros;

use bootloader_api::{config::Mapping, BootloaderConfig};

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

const HEAP_START: usize = 0x_4444_4444_0000;
const HEAP_SIZE: usize = 8 * 1024 * 1024;

static mut FRAMEBUFFER: Option<drivers::graphics::framebuffer::Framebuffer<'static>> = None;
static mut FONT: Option<drivers::graphics::psf::PSF> = None;

static FONT_DATA: &[u8] = include_bytes!("../../assets/default8x16.psf");

#[global_allocator]
static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());

pub static TERMINAL: Mutex<Option<Terminal>> = Mutex::new(None);

pub fn _print(args: fmt::Arguments) {
    let mut terminal = TERMINAL.lock();

    if let Some(term) = terminal.as_mut() {
        let _ = term.write_fmt(args);
    }
}

fn init_heap(physical_memory_offset: u64, memory_regions: &'static MemoryRegions) {
    let phys_mem_offset = VirtAddr::new(physical_memory_offset);

    let mut mapper = unsafe { memory::paging::init(phys_mem_offset) };

    let mut frame_allocator =
        unsafe { memory::frame_allocator::BootInfoFrameAllocator::init(memory_regions) };

    let heap_start = VirtAddr::new(HEAP_START as u64);
    let heap_end = heap_start + (HEAP_SIZE as u64 - 1);

    let start_page = Page::containing_address(heap_start);
    let end_page = Page::containing_address(heap_end);

    for page in Page::range_inclusive(start_page, end_page) {
        let frame = frame_allocator
            .allocate_frame()
            .expect("out of physical frames");

        memory::paging::map_page(
            &mut mapper,
            &mut frame_allocator,
            page,
            frame,
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
        )
        .expect("failed to map heap page");
    }

    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }
}

fn init_terminal(fb: &'static mut bootloader_api::info::FrameBuffer) {
    unsafe {
        FRAMEBUFFER = Some(drivers::graphics::framebuffer::Framebuffer::from_bootloader(fb));

        let framebuffer = FRAMEBUFFER.as_mut().unwrap();
        framebuffer.clear(0x1E1E2E);

        FONT = Some(drivers::graphics::psf::PSF::new(8, 16, &FONT_DATA[4..]));

        let font = FONT.as_ref().unwrap();

        framebuffer.swap_buffers();
        *TERMINAL.lock() = Some(Terminal::new(framebuffer, font, 0xCDD6F4));
    }
}

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    drivers::serial::write("Plum booted!\n");

    let physical_memory_offset = boot_info
        .physical_memory_offset
        .into_option()
        .expect("physical memory offset missing");

    let memory_regions = &boot_info.memory_regions;

    init_heap(physical_memory_offset, memory_regions);

    let fb = boot_info.framebuffer.as_mut().unwrap();
    init_terminal(fb);
    let logo_data = include_bytes!("../../assets/logo.qoi");

    if let Some(qoi) = drivers::graphics::qoi::Qoi::new(logo_data) {
        qoi.draw(unsafe { &mut FRAMEBUFFER.as_mut().unwrap() }, 340, 50);
        flush!();
    }
    println!(
        "
\x1b[35m _____ \x1b[34m _                 \x1b[36m  ___    ____
\x1b[35m|  __ \\\x1b[34m| |                \x1b[36m/ __ \\ / ____|
\x1b[35m| |__) |\x1b[34m |_   _ _ __ ___ \x1b[36m| |  | | (___
\x1b[35m|  ___/\x1b[34m| | | | | '_ ` _ \\\x1b[36m| |  | |\\___ \\
\x1b[35m| |\x1b[34m    | | |_| | | | | | |\x1b[36m| |__| |____) |
\x1b[35m|_|\x1b[34m    |_|\\__,_|_| |_| |_|\x1b[36m\\____/|_____/

\x1b[95m          PlumOS
\x1b[94m      by segfaultuwu
\x1b[0m"
    );
    println!("Framebuffer terminal works.");
    loop {
        flush!();
        print!("$ ");
        flush!();
        let input = drivers::keyboard::read_line();
        if input == "clear" {
            unsafe {
                if let Some(framebuffer) = FRAMEBUFFER.as_mut() {
                    TERMINAL.lock().as_mut().unwrap().reset();
                    framebuffer.swap_buffers();
                }
            }
        } else {
            println!("You typed: {}", input);
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    drivers::serial::write("Kernel panic!\n");

    if let Some(location) = info.location() {
        drivers::serial::write(location.file());
        drivers::serial::write(":");
        crate::utils::serial_write_usize(location.line() as usize);
        drivers::serial::write("\n");
    }

    loop {}
}
