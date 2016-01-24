use std::env;

fn main() {
    let outdir = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", outdir);
}
