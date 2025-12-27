fn main() {
    if let Err(err) = quick_cmd::run() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}
