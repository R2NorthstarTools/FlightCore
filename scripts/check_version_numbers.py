# %%
""""Ensure that version numbers between `tauri.conf.json`, `cargo.toml`, and GitHub release are the same"""
import json
import toml

with open("src-tauri/tauri.conf.json", "rt") as f:
    tauri_conf_json = json.load(f)

with open("src-tauri/Cargo.toml", "rt") as f:
    Cargo_toml = toml.load(f)

tauri_conf_json["package"]["version"]
Cargo_toml["package"]["version"]

assert(tauri_conf_json["package"]["version"] == Cargo_toml["package"]["version"])
