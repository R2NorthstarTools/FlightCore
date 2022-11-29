# Development

FlightCore uses [Tauri](https://tauri.app/) as its UI framework. This means it is split into a **backend** written in [Rust](https://www.rust-lang.org/) and a frontend written in [Vue](https://vuejs.org/) and [TypeScript](https://www.typescriptlang.org/).

## Design goals

In general FlightCore should _just work™_ for the majority of people using it. All errors should be caught and handled where possible. Thanks to CI and auto-updating, releases are easy to deploy and should be made whenever new features are available.

Pro-user and developer oriented features should be hidden by default to avoid users activating them by accident but still easy enough to access such that it doesn't become a hassle using them.

As for splitting logic between _frontend_ and _backend_, state and UI related logic should be done in frontend while all the remaining logic is done in backend. The backend should not hold state to avoid any concurrency issues in regards to asynchronous and multithreaded calls into the backend.

## Setup

Make sure you have the necessary dependencies for Tauri installed as described in this link: https://tauri.app/v1/guides/getting-started/prerequisites


Then, install `npm` dependencies with

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

Automatic recompiling on save is enabled for both the Rust and the Typescript/Vue code.

## Tauri

An introduction to Tauri can be seen in this short YouTube video: https://youtu.be/-X8evddpu7M

A longer Tauri tutorial can be found here: https://youtu.be/kRoGYgAuZQE

## Tips

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

## Docs

### Persistent store

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

## Other

This repo uses [EditorConfig](https://editorconfig.org/) to define some basic formatting rules. Find a plugin for your IDE [here](https://editorconfig.org/#download).

## Additional info

Based on source code for the [Rust Tauri Introduction Video](https://www.youtube.com/watch?v=kRoGYgAuZQE&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q)
