use opgen;

fn main() {
    //let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let _ = opgen::generator::new().generate("opgen/encoding/sm83.yml", "foo.rs");
}
