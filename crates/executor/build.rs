use std::{env, path::PathBuf};

fn main() {
    let crate_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let config = cbindgen::Config::from_file(crate_dir.join("cbindgen.toml")).unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .generate()
        .unwrap()
        .write_to_file("cyproto.h");
}
