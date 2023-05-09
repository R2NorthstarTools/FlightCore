// Linux specific code

use regex::Regex;
use std::process::Command;

// I intend to add more linux related stuff to check here, so making a func
// for now tho it only checks `ldd --version`
// - salmon
#[cfg(target_os = "linux")]
pub fn linux_checks_librs() -> Result<(), String> {
    // Perform various checks in terms of Linux compatibility
    // Return early with error message if a check fails

    // check `ldd --version` to see if glibc is up to date for northstar proton
    let min_required_ldd_version = 2.33;
    let lddv = check_glibc_v();
    if lddv < min_required_ldd_version {
        return Err(format!(
            "GLIBC is not version {} or greater",
            min_required_ldd_version
        ));
    };

    // All checks passed
    Ok(())
}

pub fn check_glibc_v() -> f32 {
    let out = Command::new("/bin/ldd")
        .arg("--version")
        .output()
        .expect("failed to run 'ldd --version'");

    // parse the output down to just the first line
    let lddva = String::from_utf8_lossy(&out.stdout);
    let lddvl: Vec<&str> = lddva.split('\n').collect();
    let lddvlo = &lddvl[0];
    let reg = Regex::new(r"(2.\d{2}$)").unwrap();
    if let Some(caps) = reg.captures_iter(lddvlo).next() {
        return caps.get(1).unwrap().as_str().parse::<f32>().unwrap(); // theres prolly a better way ijdk how tho
    }
    0.0 // this shouldnt ever be reached but it has to be here
}

/*
Outputs of ldd --verssion from distros, all we care about is the first line so trimmed, also removed all duplicates
Thanks tony
Distros not included: AmazonLinux, Gentoo, Kali, Debian before 11, Oracle Linux, Scientific Linux, Slackware, Mageia, Neurodebian, RHEL 8 and 9 (Same as AlmaLinux), RockyLinux (Same as AlmaLinux), Ubuntu before 20.04

AlmaLinux 8
ldd (GNU libc) 2.35

Centos Stream 8
ldd (GNU libc) 2.28

Centos Stream 9
ldd (GNU libc) 2.34

Centos 7
ldd (GNU libc) 2.17

Debian 11
ldd (Debian GLIBC 2.31-13+deb11u4) 2.31

Debian Testing
ldd (Debian GLIBC 2.35-1) 2.35

Debian Unstable
ldd (Debian GLIBC 2.35-3) 2.35

Fedora 37
ldd (GNU libc) 2.36

Opensuse Leap
ldd (GNU libc) 2.31

Ubuntu 20.04
ldd (Ubuntu GLIBC 2.31-0ubuntu9.9) 2.31

Ubuntu 22.04
ldd (Ubuntu GLIBC 2.35-0ubuntu3.1) 2.35

Ubuntu 22.10
ldd (Ubuntu GLIBC 2.36-0ubuntu2) 2.36
*/
