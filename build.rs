use std::process::Command;

fn main() {
    Command::new("make")
        .arg("-C")
        .arg("openlibm")
        .arg("libopenlibm.a")
        .env("CROSSCC", "gcc")
        .env("CFLAGS", "-fno-stack-protector")
        .spawn().unwrap().wait().unwrap();
    println!("cargo:rustc-link-search=native=openlibm");
}
