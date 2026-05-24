mod cat;
mod echo;
mod help;
mod mkdir;
mod ls;
mod pwd;
mod rm;
mod sleep;
mod touch;
mod truth;

use std::path::Path;

pub fn dispatch(args: &[String]) {
    let program_name = args
        .first()
        .and_then(|arg| Path::new(arg).file_stem())
        .and_then(|stem| stem.to_str())
        .unwrap_or("userland");

    let command = if is_known_applet(program_name) {
        program_name
    } else {
        args.get(1).map(String::as_str).unwrap_or("help")
    };

    let command_args = if command == program_name {
        &args[1..]
    } else if args.len() > 2 {
        &args[2..]
    } else {
        &[]
    };

    let exit_code = match command {
        "cat" => cat::run(command_args),
        "echo" => echo::run(command_args),
        "help" => help::run(command_args),
        "mkdir" => mkdir::run(command_args),
        "ls" => ls::run(command_args),
        "pwd" => pwd::run(command_args),
        "rm" => rm::run(command_args),
        "sleep" => sleep::run(command_args),
        "touch" => touch::run(command_args),
        "true" => truth::run_true(command_args),
        "false" => truth::run_false(command_args),
        other => {
            eprintln!("userland: unknown applet `{other}`");
            help::run(&[])
        }
    };

    if exit_code != 0 {
        std::process::exit(exit_code);
    }
}

fn is_known_applet(name: &str) -> bool {
    matches!(
        name,
        "cat"
            | "echo"
            | "help"
            | "ls"
            | "mkdir"
            | "pwd"
            | "rm"
            | "sleep"
            | "touch"
            | "true"
            | "false"
    )
}
