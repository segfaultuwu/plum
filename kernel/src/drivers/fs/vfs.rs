use alloc::{string::String, vec::Vec};
use spin::Mutex;

pub struct Vfs {
    pub root: VfsNode, // "/"
}

pub struct VfsNode {
    pub name: String,
    pub is_dir: bool,
    pub children: Vec<VfsNode>,
}

pub static VFS: Mutex<Vfs> = Mutex::new(Vfs {
    root: VfsNode {
        name: String::new(),
        is_dir: true,
        children: Vec::new(),
    },
});

pub fn mount_fat_image(img: &'static [u8]) -> Result<(), crate::drivers::fs::fat::FatError> {
    let vol = crate::drivers::fs::fat::FatVolume::new(img)?;
    let entries = vol.list_root_directory()?;
    let mut vfs = VFS.lock();
    vfs.root.name = String::from("/");
    vfs.root.children.clear();
    for entry in entries {
        vfs.root.children.push(VfsNode {
            name: entry.name,
            is_dir: entry.is_directory,
            children: Vec::new(),
        });
    }
    Ok(())
}

pub fn auto_mount() {
    if let Some(img) = crate::drivers::disk::get_first_image() {
        if mount_fat_image(img).is_ok() {
            crate::println!("Auto-mounted first disk to /");
        } else {
            crate::println!("Failed to auto-mount first disk (invalid format)");
        }
    } else {
        crate::println!("No disk to auto-mount");
    }
}
