use opgen::generator;
use std::env;
use std::path::PathBuf;

fn main() {
    //let url = format!("vscode://vadimcn.vscode-lldb/launch/config?{{'request':'attach','pid':{}}}", std::process::id());
    //std::process::Command::new("C:\\Users\\olig1\\AppData\\Local\\Programs\\Microsoft VS Code\\Code.exe").arg("--open-url").arg(url).output().unwrap();
    //std::thread::sleep_ms(1000); // Wait for debugger to attach
    const LR35902_DEFINITION_PATH: &str = "../opgen/encoding/sm83.yml";
    println!("cargo:rerun-if-changed={}", LR35902_DEFINITION_PATH);

    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let lr35902_path = out_dir.join("sm83.rs");
    let generator =
        generator::new().generate(LR35902_DEFINITION_PATH, lr35902_path.to_str().unwrap());
}
