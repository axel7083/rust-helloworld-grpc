use std::{env, path::PathBuf};

use std::io::Result;

fn main() -> Result<()> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("helloworld_descriptor.bin"))
        .compile(&["proto/helloworld.proto"], &["proto"])
        .unwrap();
    Ok(())
}
