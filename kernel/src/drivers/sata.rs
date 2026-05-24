use alloc::string::String;

use crate::drivers::disk::{register, BlockDevice};

/// Discover SATA/AHCI controllers and register found block devices.
///
/// Currently this is a minimal stub: it registers a single pseudo-device
/// named "sata0" for demonstration. Replace this with real PCI/AHCI
/// enumeration and driver initialization when a PCI layer exists.
pub fn discover() -> alloc::vec::Vec<BlockDevice> {
    let mut devices = alloc::vec::Vec::new();

    // Stub device for development/testing
    let dev = BlockDevice {
        name: String::from("sata0"),
        size: 4 * 1024 * 1024, // 4 MiB (placeholder)
    };

    // Register and return
    register(dev.clone());
    devices.push(dev);

    devices
}

/// For development: register an embedded image if `rootfs.img` exists in repo
pub fn register_embedded_rootfs() {
    // If we have compiled a raw rootfs image, include it. Change path as needed.
    // This is optional — callers can register images via `drivers::disk::register_image`.
    #[allow(unused_imports)]
    {
        // Example: include_bytes!("../../rootfs/rootfs.img");
        // If you create such an image, uncomment registration below.
        // let img: &'static [u8] = include_bytes!("../../rootfs/rootfs.img");
        // crate::drivers::disk::register_image("rootfs", img);
    }
}
