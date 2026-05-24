use crate::{drivers, println};

pub fn lsblk() {
    // Attempt discovery (stub)
    let _discovered = drivers::sata::discover();

    let devices = drivers::disk::list();
    if devices.is_empty() {
        println!("No block devices found.");
        return;
    }

    println!("NAME\tSIZE (bytes)");
    for d in devices {
        println!("{}\t{}", d.name, d.size);
    }
}
