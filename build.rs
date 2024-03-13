use std::env;
use std::io::Result;

fn main() -> Result<()> {
    let mut prost_build = prost_build::Config::new();
    let zmk_path = env::var("ZMK_SOURCE_DIR").unwrap_or("../".to_string());
    prost_build.include_file("_include.rs");
    prost_build.compile_protos(&[format!("{zmk_path}/zmk/app/proto/zmk/studio.proto")], &[format!("{zmk_path}/zmk/app/proto/zmk/")])?;
    Ok(())
}