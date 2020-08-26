use std::{ env };

fn main() {
    let target = env::var("TARGET").unwrap();
    if target != "wasm32-unknown-unknown" {
        panic!("Target must be wasm32-unknown-unknown, but it's {}", target);
    }
    let path = env::current_dir().unwrap();
    // println!("cargo:root={}", path.join("opus").join("lib"));
    println!("cargo:rustc-flags=-L native={} -l static=opus", path.join("opus").join("lib").to_str().unwrap());
}
