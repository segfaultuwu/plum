#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! flush {
    () => {
        unsafe {
            if let Some(framebuffer) = $crate::FRAMEBUFFER.as_mut() {
                framebuffer.swap_buffers();
            }
        }
    };
}

#[macro_export]
macro_rules! println {
    () => {{
        $crate::print!("\n");
        $crate::flush!();
    }};

    ($fmt:expr) => {{
        $crate::print!(concat!($fmt, "\n"));
        $crate::flush!();
    }};

    ($fmt:expr, $($arg:tt)*) => {{
        $crate::print!(
            concat!($fmt, "\n"),
            $($arg)*
        );
        $crate::flush!();
    }};
}
