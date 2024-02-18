use std::path::PathBuf;
use opgen;

fn main() {
    //let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let generator = opgen::generator::new().generate("opgen/encoding/lr35902.yml", "foo.rs");
}
