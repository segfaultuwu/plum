use alloc::{string::String, vec::Vec};

pub struct Vfs {
    pub root: VfsNode,
}

pub struct VfsNode {
    pub name: String,
    pub is_dir: bool,
    pub children: Vec<VfsNode>,
}
