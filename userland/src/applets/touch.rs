use std::{fs::OpenOptions, io::Write};

pub fn run(args: &[String]) -> i32 {
    if args.is_empty() {
        eprintln!("touch: missing operand");
        return 1;
    }

    let mut status = 0;

    for path in args {
        let result = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .and_then(|mut file| file.write_all(&[]));

        if let Err(err) = result {
            eprintln!("touch: {path}: {err}");
            status = 1;
        }
    }

    status
}
