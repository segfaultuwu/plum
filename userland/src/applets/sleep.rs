use std::{thread, time::Duration};

pub fn run(args: &[String]) -> i32 {
    let seconds = match args.first() {
        Some(value) => match value.parse::<f64>() {
            Ok(parsed) if parsed >= 0.0 => parsed,
            _ => {
                eprintln!("sleep: invalid duration: {value}");
                return 1;
            }
        },
        None => {
            eprintln!("sleep: missing operand");
            return 1;
        }
    };

    thread::sleep(Duration::from_secs_f64(seconds));
    0
}
