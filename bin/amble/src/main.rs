fn main() {
    if let Err(err) = amble::cli::run() {
        eprintln!("Error: {err:?}");
        std::process::exit(1);
    }
}
