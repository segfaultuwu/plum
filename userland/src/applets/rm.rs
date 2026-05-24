use std::fs;

pub fn run(args: &[String]) -> i32 {
    if args.is_empty() {
        eprintln!("rm: missing operand");
        return 1;
    }

    let mut recursive = false;
    let mut targets = Vec::new();

    for arg in args {
        match arg.as_str() {
            "-r" | "-rf" | "-fr" => recursive = true,
            other => targets.push(other),
        }
    }

    if targets.is_empty() {
        eprintln!("rm: missing operand");
        return 1;
    }

    let mut status = 0;

    for target in targets {
        let result = if recursive {
            fs::remove_dir_all(target)
        } else {
            match fs::metadata(target) {
                Ok(metadata) if metadata.is_dir() => fs::remove_dir(target),
                Ok(_) => fs::remove_file(target),
                Err(err) => Err(err),
            }
        };

        if let Err(err) = result {
            eprintln!("rm: {target}: {err}");
            status = 1;
        }
    }

    status
}
