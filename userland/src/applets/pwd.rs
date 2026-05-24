pub fn run(_args: &[String]) -> i32 {
    match std::env::current_dir() {
        Ok(path) => {
            println!("{}", path.display());
            0
        }
        Err(err) => {
            eprintln!("pwd: {err}");
            1
        }
    }
}
