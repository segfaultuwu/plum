pub fn init_pic() {
    unsafe {
        x86_64::instructions::port::Port::<u8>::new(0x20).write(0x11);
        x86_64::instructions::port::Port::<u8>::new(0xA0).write(0x11);

        x86_64::instructions::port::Port::<u8>::new(0x21).write(0x20);
        x86_64::instructions::port::Port::<u8>::new(0xA1).write(0x28);

        x86_64::instructions::port::Port::<u8>::new(0x21).write(0x04);
        x86_64::instructions::port::Port::<u8>::new(0xA1).write(0x02);

        x86_64::instructions::port::Port::<u8>::new(0x21).write(0x01);
        x86_64::instructions::port::Port::<u8>::new(0xA1).write(0x01);

        x86_64::instructions::port::Port::<u8>::new(0x21).write(0x00);
        x86_64::instructions::port::Port::<u8>::new(0xA1).write(0x00);
    }
}
