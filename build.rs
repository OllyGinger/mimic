use std::path::PathBuf;
use std::env;
use opgen::generator;

fn main() {
    //let url = format!("vscode://vadimcn.vscode-lldb/launch/config?{{'request':'attach','pid':{}}}", std::process::id());
    //std::process::Command::new("C:\\Users\\olig1\\AppData\\Local\\Programs\\Microsoft VS Code\\Code.exe").arg("--open-url").arg(url).output().unwrap();
    //std::thread::sleep_ms(1000); // Wait for debugger to attach

    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let generator = generator::new().generate("opgen/encoding/lr35902.yml", "foo.rs");
}
