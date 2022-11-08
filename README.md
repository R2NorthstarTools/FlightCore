<p align="center">
	<img src="assets/Square310x310Logo.png" width="200px"><br>
</p>

# FlightCore

A [Northstar](https://northstar.tf/) installer, updater, and soon to be mod-manager

![FlightCore screenshot](assets/main-window-screenshot.png)

## Roadmap

--> See https://github.com/GeckoEidechse/FlightCore/issues/1

## Download

Head to over to [releases](https://github.com/GeckoEidechse/FlightCore/releases) and download
- **Windows:** `FlightCore_X.Y.Z_x64_en-US.msi`
or
- **Linux:** `flight-core_X.Y.Z_amd64.AppImage` .

## Development

Make sure you have the necessary dependencies installed: https://tauri.app/v1/guides/getting-started/prerequisites


Install `npm` dependencies with 

```sh
npm install
```

Install UI dependencies too

```sh
cd src-vue && npm install
```

Then for developing

```sh
npx tauri dev
```

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

### Docs

In regards to storing persistent data, FlightCore uses [`tauri-plugin-store`](https://github.com/tauri-apps/tauri-plugin-store). It's a key-value store accessed in frontend to load and store small amounts of data.

The goal is to store as little data in there as possible and instead get the necessary info on app launch.
For example the install path of Titanfall2 should be inferred everytime on launch using Steam library or Origin, so that if the player changes the install location, there's no need to sync it up with the persistent store again.
The exception to this is when Steam/Origin is unable to find the install location and the user manually selected a location instead. In this case, we don't want to re-prompt the user on every launch of FlightCore to enter the Titanfall2 install location.

**Usage example for `tauri-plugin-store`:**

```typescript
// Import the lib
import { Store } from 'tauri-plugin-store-api';
// Define a store based on filename to write to
const persistentStore = new Store('flight-core-settings.json');

// Save change in persistent store
await persistentStore.set('northstar-release-canal', { value: "NorthstarReleasecandidate" });

// Grab Northstar release canal value from store if exists
var persistent_northstar_release_canal = (await persistentStore.get('northstar-release-canal')) as any;
if(persistent_northstar_release_canal) { // For some reason, the plugin-store doesn't throw an eror but simply returns `null` when key not found
    // Put value from peristent store into current store
    state.northstar_release_canal = persistent_northstar_release_canal.value as string;
}
else {
    console.log("Value not found in store");
}

```

### Building

Release builds are generally done via CI. To build locally, make sure typescript is compiled (`./node_modules/.bin/rollup --config`), then run `npm run tauri build`.

### Other

This repo uses [EditorConfig](https://editorconfig.org/) to define some basic formatting rules. Find a plugin for your IDE [here](https://editorconfig.org/#download).

## Why yet another Northstar intaller/updater/mod-manager instead of contributing to an existing one?

The 3 main GUI tools for handling such tasks with Norhtstar are

- [r2modman](https://github.com/ebkr/r2modmanPlus)
- [Viper](https://github.com/0neGal/viper)
- [VTOL](https://github.com/BigSpice/VTOL)

while they get most of the work done, each of them has their own problem.

- **r2modman** has not too great UX and given that it also has to support other games there's not a(n easy) way to quickly add new features specific to Northstar
- **Viper** probably has the best UX but is missing features such as Origin process runtime detection (to avoid LSX errors) and lacks the ability to install Northstar from Thunderstore. Further there are still cases where JavaScript errors are not handled properly simply showing the stack trace and confusing users.
- **VTOL** has recently undergone a rewrite that removes a lot of older issues (such as requiring to be run as admin), however it is Windows exclusive and requires installing an additional library not shipped directly with the application, confusing some users. It also has a lot of edge case handling that while giving a smoother user experience blows up code base complexity.

With that said, FlightCore is not written from scratch. For handling Northstar specific logic, functions are re-used from the CLI-only Northstar installer called [papa](https://github.com/AnActualEmerald/papa) by making use of the underlying library [libthermite](https://crates.io/crates/libthermite).

The plan is to upstream any changes to `libthermite` so that it can be re-used by any other Rust-based Northstar installer.

## Additional info

Based on source code for the [Rust Tauri Introduction Video](https://www.youtube.com/watch?v=kRoGYgAuZQE&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q)
