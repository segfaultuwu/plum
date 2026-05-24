pub fn run(args: &[String]) -> i32 {
    let mut first = true;

    for arg in args {
        if !first {
            print!(" ");
        }
        first = false;
        print!("{arg}");
    }

    println!();
    0
}
