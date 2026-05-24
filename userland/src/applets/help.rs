pub fn run(_args: &[String]) -> i32 {
    println!("userland - BusyBox-style multi-call binary");
    println!();
    println!("Available applets:");
    println!("  cat   - print files or stdin");
    println!("  echo  - print arguments");
    println!("  help  - show this message");
    println!("  mkdir - create directories");
    println!("  ls    - list directory entries");
    println!("  pwd   - print the current directory");
    println!("  rm    - remove files or directories");
    println!("  sleep - pause for a number of seconds");
    println!("  touch - create empty files");
    println!("  true  - exit successfully");
    println!("  false - exit with failure");
    println!();
    println!("Usage:");
    println!("  userland <applet> [args...]");
    println!("  ln -s userland cat   # or any other applet name");
    0
}
