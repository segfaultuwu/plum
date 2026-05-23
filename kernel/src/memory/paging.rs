use x86_64::{
    registers::control::Cr3,
    structures::paging::{
        mapper::MapToError, Mapper, OffsetPageTable, Page, PageTable, PageTableFlags, PhysFrame,
        Size4KiB,
    },
    VirtAddr,
};

pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);

    unsafe { OffsetPageTable::new(level_4_table, physical_memory_offset) }
}

unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();

    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    unsafe { &mut *page_table_ptr }
}

pub fn map_page(
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl x86_64::structures::paging::FrameAllocator<Size4KiB>,
    page: Page,
    frame: PhysFrame,
    flags: PageTableFlags,
) -> Result<(), MapToError<Size4KiB>> {
    let map_to_result = unsafe { mapper.map_to(page, frame, flags, frame_allocator) };

    map_to_result?.flush();

    Ok(())
}
