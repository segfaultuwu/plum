use crate::{drivers, flush, print, println, FRAMEBUFFER, TERMINAL};
mod commands;

pub fn main() -> ! {
    println!("Welcome to PlumOS!");

    loop {
        flush!();
        print!("\x1b[92mroot\x1b[0m@\x1b[94mplum\x1b[0m:\x1b[96m/\x1b[0m $ ");
        flush!();
        let input = drivers::keyboard::read_line();
        if input == "clear" {
            unsafe {
                if let Some(framebuffer) = FRAMEBUFFER.as_mut() {
                    TERMINAL.lock().as_mut().unwrap().reset();
                    framebuffer.swap_buffers();
                }
            }
        } else if input == "lsblk" {
            commands::lsblk::lsblk();
        } else {
            println!(
                "{}: {}",
                crate::utils::colors::ansi_wrap(
                    crate::utils::colors::AnsiColor::Red,
                    "Not a valid command"
                ),
                input
            );
        }
    }
}
