use std::{fs, io};

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    fs::create_dir_all(format!("{}/templates/bin/", out_dir))
        .expect("unable to create templates bin directory");
    fs::create_dir_all(format!("{}/templates/lib/", out_dir))
        .expect("unable to create templates lib directory");

    copy_content(&out_dir, "templates/bin/Cargo.toml");
    copy_content(&out_dir, "templates/bin/main.rs");
    copy_content(&out_dir, "templates/lib/Cargo.toml");
    copy_content(&out_dir, "templates/lib/lib.rs");
    copy_content(&out_dir, "templates/Cargo.toml");
}

fn copy_content(out: &str, source: &str) {
    let out_path = format!("{}/{}", out, source);
    let mut out_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(out_path)
        .expect("unable to open/create data file");
    if let Ok(mut source_file) = fs::File::open(source) {
        io::copy(&mut source_file, &mut out_file).expect("failed to copy data after opening");
    }
}
