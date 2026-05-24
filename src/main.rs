use std::process::Command;

fn main() {
    let image = env!("PLUM_BIOS_IMG");
    let rootfs = env!("PLUM_ROOTFS_IMG");

    Command::new("qemu-system-x86_64")
        .args([
            "-machine",
            "pc",
            "-drive",
            &format!("index=0,format=raw,file={}", image),
            "-drive",
            &format!("index=1,format=raw,file={}", rootfs),
            "-serial",
            "stdio",
            "-no-reboot",
            "-no-shutdown",
        ])
        .status()
        .unwrap();
}
