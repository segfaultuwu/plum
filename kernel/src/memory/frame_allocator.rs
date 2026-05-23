use bootloader_api::info::{MemoryRegionKind, MemoryRegions};
use x86_64::{
    structures::paging::{FrameAllocator, PhysFrame, Size4KiB},
    PhysAddr,
};

pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryRegions,
    next: usize,
}

impl BootInfoFrameAllocator {
    pub unsafe fn init(memory_map: &'static MemoryRegions) -> Self {
        Self {
            memory_map,
            next: 0,
        }
    }

    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> + '_ {
        self.memory_map
            .iter()
            .filter(|region| region.kind == MemoryRegionKind::Usable)
            .map(|region| region.start..region.end)
            .flat_map(|range| range.step_by(4096))
            .map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}
