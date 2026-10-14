use super::{delete_thunderstore_mod, fc_sanity_check};
use crate::{GameInstall, InstallType};
use std::io::Write;

fn package_archive(paths: &[&str]) -> tempfile::NamedTempFile {
    let mut archive_file = tempfile::NamedTempFile::new().unwrap();
    {
        let mut archive = zip::ZipWriter::new(archive_file.as_file_mut());
        archive
            .start_file("manifest.json", zip::write::SimpleFileOptions::default())
            .unwrap();
        archive.write_all(b"{}").unwrap();
        for path in paths {
            archive
                .start_file(*path, zip::write::SimpleFileOptions::default())
                .unwrap();
            archive.write_all(b"{}").unwrap();
        }
        archive.finish().unwrap();
    }
    archive_file
}

#[test]
fn script_package_does_not_request_native_approval() {
    let archive = package_archive(&["mods/Test/mod.json"]);
    assert!(fc_sanity_check(&archive.as_file(), |_| panic!()).is_ok());
}

#[test]
fn invalid_package_does_not_request_native_approval() {
    let archive = package_archive(&["plugins/Test.dll"]);
    assert!(fc_sanity_check(&archive.as_file(), |_| panic!()).is_err());
}

#[test]
fn native_package_requests_one_approval_for_all_plugins() {
    let archive = package_archive(&[
        "mods/Test/mod.json",
        "plugins/Body.dll",
        "plugins/Shadow.dll",
    ]);
    let mut approvals = 0;
    assert!(fc_sanity_check(&archive.as_file(), |paths| {
        approvals += 1;
        assert_eq!(paths.len(), 2);
        assert!(paths.iter().any(|path| path.ends_with("Body.dll")));
        assert!(paths.iter().any(|path| path.ends_with("Shadow.dll")));
        true
    })
    .is_ok());
    assert_eq!(approvals, 1);
}

#[test]
fn denied_native_package_does_not_extract_files() {
    let archive = package_archive(&["mods/Test/mod.json", "plugins/Body.dll"]);
    let target = tempfile::tempdir().unwrap();
    let result = thermite::core::manage::install_with_sanity(
        "Publisher-NativePlugin-1.0.0",
        archive.as_file(),
        target.path(),
        |input| fc_sanity_check(input, |_| false),
    );
    assert!(result.is_err());
    assert!(std::fs::read_dir(target.path()).unwrap().next().is_none());
}

#[test]
fn approved_native_package_can_be_uninstalled_and_reinstalled() {
    let archive = package_archive(&["mods/Test/mod.json", "plugins/Body.dll"]);
    let target = tempfile::tempdir().unwrap();
    let profile = target.path().join("NativePluginProfile");
    let packages = profile.join("packages");
    let unrelated_plugin = profile.join("plugins/Unrelated.dll");
    std::fs::create_dir_all(unrelated_plugin.parent().unwrap()).unwrap();
    std::fs::write(&unrelated_plugin, b"MZ").unwrap();
    let installed = thermite::core::manage::install_with_sanity(
        "Publisher-NativePlugin-1.0.0",
        archive.as_file(),
        &packages,
        |input| fc_sanity_check(input, |_| true),
    )
    .unwrap();
    assert!(installed.join("plugins/Body.dll").is_file());
    assert!(installed.join("mods/Test/mod.json").is_file());
    assert!(installed
        .canonicalize()
        .unwrap()
        .starts_with(target.path().canonicalize().unwrap()));
    delete_thunderstore_mod(
        GameInstall {
            game_path: target.path().display().to_string(),
            profile: "NativePluginProfile".into(),
            install_type: InstallType::UNKNOWN,
        },
        "Publisher-NativePlugin-1.0.0".into(),
    )
    .unwrap();
    assert!(!installed.exists());
    assert!(unrelated_plugin.is_file());
    let reinstalled = thermite::core::manage::install_with_sanity(
        "Publisher-NativePlugin-1.0.0",
        archive.as_file(),
        &packages,
        |input| fc_sanity_check(input, |_| true),
    )
    .unwrap();
    assert!(reinstalled.join("plugins/Body.dll").is_file());
}
