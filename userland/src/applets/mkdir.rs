use std::fs;

pub fn run(args: &[String]) -> i32 {
    if args.is_empty() {
        eprintln!("mkdir: missing operand");
        return 1;
    }

    let mut status = 0;

    for path in args {
        if let Err(err) = fs::create_dir_all(path) {
            eprintln!("mkdir: {path}: {err}");
            status = 1;
        }
    }

    status
}
