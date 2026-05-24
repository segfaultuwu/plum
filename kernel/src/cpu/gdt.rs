use spin::Once;
use x86_64::instructions::segmentation::{Segment, CS, DS};
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

const DOUBLE_FAULT_IST_INDEX: u16 = 0;

static TSS_ONCE: Once<TaskStateSegment> = Once::new();
static GDT_ONCE: Once<(GlobalDescriptorTable, Selectors)> = Once::new();

fn tss_init() -> TaskStateSegment {
    let mut tss = TaskStateSegment::new();
    tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
        const STACK_SIZE: usize = 4096 * 5;
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

        let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
        let stack_end = stack_start + (STACK_SIZE as u64);
        stack_end
    };
    tss
}

fn gdt_init() -> (GlobalDescriptorTable, Selectors) {
    let tss = TSS_ONCE.call_once(tss_init);

    let mut gdt = GlobalDescriptorTable::new();
    let kernel_code = gdt.append(Descriptor::kernel_code_segment());
    let kernel_data = gdt.append(Descriptor::kernel_data_segment());
    let user_code = gdt.append(Descriptor::user_code_segment());
    let user_data = gdt.append(Descriptor::user_data_segment());
    let tss_selector = gdt.append(Descriptor::tss_segment(tss));

    (
        gdt,
        Selectors {
            kernel_code,
            kernel_data,
            user_code,
            user_data,
            tss: tss_selector,
        },
    )
}

pub struct Selectors {
    pub kernel_code: SegmentSelector,
    pub kernel_data: SegmentSelector,
    pub user_code: SegmentSelector,
    pub user_data: SegmentSelector,
    pub tss: SegmentSelector,
}

pub fn init_gdt() {
    use x86_64::instructions::tables::load_tss;
    let gdt = GDT_ONCE.call_once(gdt_init);
    gdt.0.load();
    unsafe {
        CS::set_reg(gdt.1.kernel_code);
        DS::set_reg(gdt.1.kernel_data);
        load_tss(gdt.1.tss);
    }
}

pub fn get_selectors() -> &'static Selectors {
    &GDT_ONCE.get().expect("GDT not initialized").1
}
