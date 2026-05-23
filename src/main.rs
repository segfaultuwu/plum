use std::process::Command;

fn main() {
    let image = env!("PLUM_BIOS_IMG");

    Command::new("qemu-system-x86_64")
        .args([
            "-drive",
            &format!("format=raw,file={image}"),
            "-serial",
            "stdio",
            "-no-reboot",
            "-no-shutdown",
        ])
        .status()
        .unwrap();
}
