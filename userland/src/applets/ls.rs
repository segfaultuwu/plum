use std::{fs, path::Path};

pub fn run(args: &[String]) -> i32 {
    let target = args.first().map(String::as_str).unwrap_or(".");
    let path = Path::new(target);

    let read_dir = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("ls: {target}: {err}");
            return 1;
        }
    };

    let mut names = Vec::new();
    for entry in read_dir {
        match entry {
            Ok(entry) => names.push(entry.file_name().to_string_lossy().into_owned()),
            Err(err) => {
                eprintln!("ls: {target}: {err}");
                return 1;
            }
        }
    }

    names.sort_unstable();

    for name in names {
        println!("{name}");
    }

    0
}
