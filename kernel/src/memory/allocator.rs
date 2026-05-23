use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use spin::Mutex;

pub struct Locked<A> {
    inner: Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Self {
            inner: Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
    allocations: usize,
    used: usize,
}

impl BumpAllocator {
    pub const fn new() -> Self {
        Self {
            heap_start: 0,
            heap_end: 0,
            next: 0,
            allocations: 0,
            used: 0,
        }
    }

    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start.saturating_add(heap_size);
        self.next = heap_start;
        self.allocations = 0;
        self.used = 0;
    }

    pub fn used(&self) -> usize {
        self.used
    }

    pub fn free(&self) -> usize {
        self.heap_end.saturating_sub(self.next)
    }

    pub fn allocations(&self) -> usize {
        self.allocations
    }

    pub fn reset(&mut self) {
        self.next = self.heap_start;
        self.allocations = 0;
        self.used = 0;
    }

    fn alloc_inner(&mut self, layout: Layout) -> *mut u8 {
        if self.heap_start == 0 || self.heap_end == 0 {
            return null_mut();
        }

        if layout.size() == 0 {
            return layout.align() as *mut u8;
        }

        let Some(alloc_start) = align_up_checked(self.next, layout.align()) else {
            return null_mut();
        };

        let Some(alloc_end) = alloc_start.checked_add(layout.size()) else {
            return null_mut();
        };

        if alloc_end > self.heap_end {
            return null_mut();
        }

        self.next = alloc_end;
        self.allocations += 1;
        self.used = self.next - self.heap_start;

        alloc_start as *mut u8
    }
}

unsafe impl GlobalAlloc for Locked<BumpAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.inner.lock().alloc_inner(layout)
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // bump allocator does not free individual allocations
    }
}

fn align_up_checked(addr: usize, align: usize) -> Option<usize> {
    let mask = align.checked_sub(1)?;

    if align == 0 || !align.is_power_of_two() {
        return None;
    }

    addr.checked_add(mask).map(|addr| addr & !mask)
}
