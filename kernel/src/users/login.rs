use alloc::string::String;

use crate::{drivers::keyboard, flush, print, println};

pub fn login() {
    use crate::drivers::serial;
    use crate::users;

    serial::write("login: entered login()\n");

    for attempt in 1..=3 {
        println!("-- LOGIN --");
        serial::write("login: printed banner\n");

        print!("Username: ");
        flush!();
        let username = keyboard::read_line();

        print!("Password: ");
        flush!();

        // Read password with masking
        let mut password = String::new();
        loop {
            if let Some(c) = keyboard::read_key() {
                match c {
                    '\n' => {
                        print!("\n");
                        flush!();
                        break;
                    }
                    '\x08' => {
                        if password.pop().is_some() {
                            print!("\x08 ");
                            print!("\x08");
                            flush!();
                        }
                    }
                    _ => {
                        password.push(c);
                        print!("*");
                        flush!();
                    }
                }
            }
        }

        match users::get_user(&username) {
            Some(u) => {
                if u.verify_password(&password) {
                    println!("\nWelcome, {}!", username);
                    return;
                } else {
                    println!("\nInvalid credentials.");
                }
            }
            None => {
                println!("\nUser not found.");
            }
        }

        if attempt < 3 {
            println!("Try again.");
        }
    }

    println!("Too many failed login attempts. Halting..");
    loop {}
}
