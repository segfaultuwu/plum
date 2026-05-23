use std::{env, path::PathBuf};

fn main() {
    let kernel = PathBuf::from(
        env::var_os("CARGO_BIN_FILE_KERNEL_kernel").expect("missing kernel artifact"),
    );

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let bios_img = out_dir.join("plum-bios.img");

    bootloader::BiosBoot::new(&kernel)
        .create_disk_image(&bios_img)
        .unwrap();

    println!("cargo:rustc-env=PLUM_BIOS_IMG={}", bios_img.display());
}
