// Linux specific code

use std::process::Command;

pub fn check_glibc_v() -> f32 {
    let out = Command::new("/bin/ldd")
        .arg("--version")
        .output()
        .expect("failed to run 'ldd --version'");
    
    let lddva = String::from_utf8_lossy(&out.stdout);
    let lddvl: Vec<&str> = lddva.split('\n').collect();
    let lddvw: Vec<&str> = lddvl[0].split(' ').collect();
    let lddv = &lddvw[3].to_string().parse::<f32>().unwrap();
    return *lddv;
}
