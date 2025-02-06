use rustc_version::{version_meta, Channel};

fn main() {
    match version_meta().unwrap().channel {
        Channel::Stable => (),
        _ => println!("cargo:rustc-cfg=feature=\"nightly\"")
    }
}
