// Linux specific code
#[allow(dead_code)]
pub fn check_glibc_v() -> String {
    let shell = std::process::Command::new("ldd")
    .arg("--version")
    .stdout(std::process::Stdio::piped())
    .output()
    .unwrap();
    let stdout = String::from_utf8(shell.stdout).unwrap();

    // parse down to 1st line
    let lddv: Vec<&str> = stdout.split('\n').collect();
    let lddv: Vec<&str> = lddv[0].split(' ').collect();
    let lddv = &lddv[3];



    return lddv.to_string();
}
