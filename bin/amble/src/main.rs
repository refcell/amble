fn main() {
    if let Err(err) = amble::run() {
        eprintln!("Error: {err:?}");
        std::process::exit(1);
    }
}
