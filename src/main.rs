fn main() {
    if let Err(err) = shell_stash::run() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}
