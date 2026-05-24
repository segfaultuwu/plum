use alloc::{format, string::String};

pub fn hash(input: &str) -> String {
    let mut hash: u64 = 0x9E37_79B9_7F4A_7C15;

    for byte in input.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100_0000_01B3);
        hash ^= hash >> 32;
        hash = hash.rotate_left(13);
    }

    format!("{:016X}", hash)
}
