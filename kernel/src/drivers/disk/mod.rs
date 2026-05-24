use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;

#[derive(Clone, Debug)]
pub struct BlockDevice {
    pub name: String,
    pub size: u64,
}

static DEVICES: Mutex<Vec<BlockDevice>> = Mutex::new(Vec::new());

pub fn register(device: BlockDevice) {
    DEVICES.lock().push(device);
}

pub fn list() -> Vec<BlockDevice> {
    DEVICES.lock().clone()
}

// Simple registry for raw disk images embedded at compile time or registered by
// drivers. Each entry stores a name and a byte slice reference.
static IMAGES: Mutex<Vec<(&'static str, &'static [u8])>> = Mutex::new(Vec::new());

pub fn register_image(name: &'static str, image: &'static [u8]) {
    IMAGES.lock().push((name, image));
}

pub fn get_first_image() -> Option<&'static [u8]> {
    IMAGES.lock().first().map(|(_, img)| *img)
}
