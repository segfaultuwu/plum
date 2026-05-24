use alloc::string::String;
use alloc::vec::Vec;

pub mod login;

use crate::drivers::fs::fat::DirectoryEntry;
use spin::Mutex;

#[derive(Clone)]
pub struct User {
    pub username: String,
    pub password_hash: String,
}

pub enum UserError {
    UserAlreadyExists,
    UserNotFound,
    InvalidPassword,
}

pub struct UserManager {
    users: alloc::collections::BTreeMap<String, User>,
}

impl UserManager {
    pub fn new() -> Self {
        Self {
            users: alloc::collections::BTreeMap::new(),
        }
    }

    pub fn add_user(&mut self, user: User) -> Result<(), UserError> {
        if self.users.contains_key(&user.username) {
            return Err(UserError::UserAlreadyExists);
        }
        self.users.insert(user.username.clone(), user);
        Ok(())
    }

    pub fn get_user(&self, username: &str) -> Option<&User> {
        self.users.get(username)
    }

    pub fn list_users(&self) -> Vec<String> {
        self.users.keys().cloned().collect()
    }
}

static USER_MANAGER: Mutex<Option<UserManager>> = Mutex::new(None);

fn bytes_to_hex(b: &[u8]) -> String {
    let mut s = String::new();
    for byte in b {
        use core::fmt::Write;
        write!(s, "{:02x}", byte).ok();
    }
    s
}

impl User {
    pub fn new(username: String, password_hash: String) -> Self {
        Self {
            username,
            password_hash,
        }
    }

    pub fn verify_password(&self, password: &str) -> bool {
        let h = crate::crypto::sha256::hash(password.as_bytes());
        let hex = bytes_to_hex(&h);
        hex == self.password_hash
    }
}

pub fn init_from_passwd() {
    // Debug: notify entry
    crate::drivers::serial::write("init_from_passwd: start\n");

    // First try to read passwd from a registered disk image at runtime
    if let Some(img) = crate::drivers::disk::get_first_image() {
        crate::drivers::serial::write("init_from_passwd: found image\n");
        if let Ok(vol) = crate::drivers::fs::fat::FatVolume::new(img) {
            crate::drivers::serial::write("init_from_passwd: opened fat volume\n");
            // try to read /etc/passwd by walking the path
            if let Some(data) = read_path_from_volume(&vol, "etc/passwd") {
                crate::drivers::serial::write("init_from_passwd: read etc/passwd from image\n");
                if let Ok(s) = core::str::from_utf8(&data) {
                    populate_from_str(s);
                    crate::drivers::serial::write("init_from_passwd: populated from image\n");
                    return;
                }
            }
        }
    } else {
        crate::drivers::serial::write("init_from_passwd: no image registered\n");
    }

    // Fallback: include the rootfs /etc/passwd at compile time
    let content = include_str!("../../../rootfs/etc/passwd");
    crate::drivers::serial::write("init_from_passwd: using compile-time passwd\n");
    let mut lock = USER_MANAGER.lock();
    let mgr = UserManager::new();
    populate_from_str(content);
    *lock = Some(mgr);
    crate::drivers::serial::write("init_from_passwd: finished fallback population\n");
}

fn populate_from_str(content: &str) {
    crate::drivers::serial::write("populate_from_str: start\n");
    let mut lock = USER_MANAGER.lock();
    let mut mgr = UserManager::new();

    let mut count: usize = 0;
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((user, hash)) = line.split_once(':') {
            let u = User::new(String::from(user.trim()), String::from(hash.trim()));
            let _ = mgr.add_user(u);
            count += 1;
        }
    }

    *lock = Some(mgr);
    crate::drivers::serial::write("populate_from_str: done. users added: ");
    crate::utils::serial_write_usize(count);
    crate::drivers::serial::write("\n");
}

fn read_path_from_volume(vol: &crate::drivers::fs::fat::FatVolume, path: &str) -> Option<Vec<u8>> {
    let mut components: alloc::vec::Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if components.is_empty() {
        return None;
    }

    // Start at root
    let mut current_cluster: Option<u32> = None; // None means root directory

    while let Some(comp) = components.first().cloned() {
        let entries = if current_cluster.is_none() {
            vol.list_root_directory().ok()?
        } else {
            vol.list_directory_cluster(current_cluster.unwrap()).ok()?
        };

        // find matching entry (case-insensitive)
        let mut found: Option<DirectoryEntry> = None;
        for e in entries {
            if e.name.eq_ignore_ascii_case(comp) {
                found = Some(e);
                break;
            }
        }

        let entry = found?;
        components.remove(0);

        if components.is_empty() {
            // last component; return file contents
            if entry.is_directory {
                return None;
            }
            return vol
                .read_cluster_chain_bytes(entry.first_cluster, entry.size)
                .ok();
        } else {
            // descend into directory
            if !entry.is_directory {
                return None;
            }
            current_cluster = Some(entry.first_cluster);
        }
    }

    None
}

pub fn get_user(username: &str) -> Option<User> {
    let lock = USER_MANAGER.lock();
    if let Some(ref mgr) = *lock {
        mgr.get_user(username).cloned()
    } else {
        None
    }
}

pub fn list_users() -> Vec<String> {
    let lock = USER_MANAGER.lock();
    if let Some(ref mgr) = *lock {
        mgr.list_users()
    } else {
        Vec::new()
    }
}
