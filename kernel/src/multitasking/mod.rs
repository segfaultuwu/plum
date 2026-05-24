pub mod context;
pub mod process;
pub mod process_manager;
pub mod scheduler;

use process_manager::ProcessManager;
use spin::Mutex;

pub static SCHEDULER: Mutex<Option<ProcessManager>> = Mutex::new(None);

pub fn schedule() {
    let mut scheduler = SCHEDULER.lock();
    if let Some(manager) = scheduler.as_mut() {
        manager.schedule();
    }
}
