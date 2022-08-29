# FlightCore

A [Northstar](https://northstar.tf/) installer, updater, and mod-manager

## Roadmap

- [ ] General
  - [ ] Icon
  - [ ] CI builds
  - [ ] Self updating
    - [x] ensure version sync with tauri.conf.json, cargo.toml
    - [x] CI release
    - [x] some scripts to create release JSON file
  - [ ] Self detect if outdated
  - [ ] Self update delivery
    - [ ] CI/CD
    - [ ] Flatpak (with CI to push flatpak update)
    - [ ] Chocolatey (maybe?)
    - [ ] Option to disable self-update (for Flatpak and Chocolatey)
  - [ ] Display current version in UI window
  - [x] Crash report uploading (sentry.io)
- [ ] Northstar install/launch support
  - [ ] 1-click to get to running Northstar
  - [ ] Detect game install path
    - [ ] Steam
    - [ ] Origin
    - [ ] EA
  - [ ] Support for multiple release channels (GitHub + Thunderstore)
  - [ ] Launch Northstar support via gamelauncher (Steam/Origin/EA)
  - [ ] Read-out current version from exe and mods
- [ ] Mod install support
  - [ ] check mod validity
  - [ ] mod update detection
- [ ] Extra
  - [ ] "dev mode"
    - [ ] PR install support
  - [ ] Support multiple profiles
  - [ ] get list of installed mods and export as TXT
  - [ ] support r2mm links
  - [ ] CloudFlare blocking debugger (in particular make sure to check IPv4)
  - [ ] Open common config files

## Development

Make sure you have the necessary dependencies installed: https://tauri.app/v1/guides/getting-started/prerequisites


Install `npm` dependencies with 

```sh
npm install
```

Then for developing

```sh
# terminal 1 (UI localhost for hot-reload)
npm run ui-dev

# terminal 2 (for the Rust/App hot-reload)
npm run tauri dev
```

> **Note**
> On Windows instead of `npm run ui-dev`, you may need to run 
> `./node_modules/.bin/rollup --config --watch`
> and
> `npm run localhost`
> in two separate console windows

### Tips

Note that you can adjust the behaviour of Tauri windows in `tauri.conf.json`, e.g.

```json
"windows": [
  {
    "fullscreen": false,
    "resizable": true,
    "alwaysOnTop": true,
    "x": 1200,
    "y": 0,
    "height": 500,
    "width": 300,
    "title": "FlightCore"
  }
]
```

### Building

Release builds are generally done via CI. To build locally, make sure typescript is compiled (`./node_modules/.bin/rollup --config`), then run `npm run tauri build`.

# Old README (to be removed)


Source code for the [Rust Tauri Introduction Video](https://www.youtube.com/watch?v=kRoGYgAuZQE&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q)

## Setup

```sh
npm install
```

## Run

```sh
# terminal 1 (UI localhost for hot-reload)
npm run ui-dev

# terminal 2 (for the Rust/App hot-reload)
npm run tauri dev
```

## Database Pool as state

Rather to have a simple Mutex for the state, database can be used. 

```
sqlx = { version = "0.6", features = [ "runtime-tokio-rustls", "postgres" ] }
```

```rs
let con_string = format!("postgres://postgres:postgres@localhost/postgres");
let db = PgPoolOptions::new()
	.max_connections(5)
	.connect(&con_string)
	.await
	.expect("Cannot create PgPool");

let arc_db = Arc::new(db);
```

Then

```rs
tauri::Builder::default()
	.manage(arc_db)
```
