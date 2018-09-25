extern crate bindgen;
extern crate cmake;

use bindgen::Builder;
use cmake::Config;
use std::fs;

fn main() {
    let dst = Config::new("weechat")
        .define("ENABLE_SCRIPTS", "OFF")
        .build();

    println!(
        "cargo:rustc-link-search=native={}/build/src/plugins",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=weechat_plugins");

    let bindings = Builder::default()
        .header("weechat/src/plugins/plugin.h")
        .generate()
        .expect("Unable to generate bindings");

    #[allow(unused_result)]
    fs::remove_file("src/ffi/mod.rs");

    bindings
        .write_to_file("src/ffi/mod.rs")
        .expect("Couldn't write bindings!");
}
