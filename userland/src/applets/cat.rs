use std::{fs, io, path::Path};

pub fn run(args: &[String]) -> i32 {
    if args.is_empty() {
        let mut stdin = io::stdin().lock();
        let mut stdout = io::stdout().lock();
        if io::copy(&mut stdin, &mut stdout).is_err() {
            return 1;
        }
        return 0;
    }

    let mut had_error = false;

    for path in args {
        match fs::read_to_string(Path::new(path)) {
            Ok(contents) => print!("{contents}"),
            Err(err) => {
                had_error = true;
                eprintln!("cat: {path}: {err}");
            }
        }
    }

    if had_error {
        1
    } else {
        0
    }
}
