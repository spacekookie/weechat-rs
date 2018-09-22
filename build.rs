extern crate cmake;
extern crate bindgen;

fn main() {
    let dst = cmake::build("weechat");
    println!("cargo:rustc-link-search=native={}/build/src/plugins", dst.display());
    println!("cargo:rustc-link-lib=static=weechat_plugins");

    let bind = binder::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
}
