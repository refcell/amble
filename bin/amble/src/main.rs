fn main() {
    if let Err(err) = amble_bin::cli::run() {
        eprintln!("Error: {err:?}");
        std::process::exit(1);
    }
}
