# Development

FlightCore uses [Tauri](https://tauri.app/) as its UI framework. This means it is split into a **backend** written in [Rust](https://www.rust-lang.org/) and a frontend written in [Vue](https://vuejs.org/) and [TypeScript](https://www.typescriptlang.org/).

## Design goals

In general FlightCore should _just work™_ for the majority of people using it. All errors should be caught and handled where possible. Thanks to CI and auto-updating, releases are easy to deploy and should be made whenever new features are available.

Pro-user and developer oriented features should be hidden by default to avoid users activating them by accident but still easy enough to access such that it doesn't become a hassle using them.

As for splitting logic between _frontend_ and _backend_, state and UI related logic should be done in frontend while all the remaining logic is done in backend. The backend should not hold state to avoid any concurrency issues in regards to asynchronous and multithreaded calls into the backend.

## Setup

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

### Frontend styling

For Vue components FlightCore uses the [Element Plus](https://element-plus.org/) library. A list of available components can be found [here](https://element-plus.org/en-US/component/button.html).

### Interacting between frontend and backend

The main way the frontend calls the backend is via the `invoke()` function provided by Tauri.

So assuming you have a backend function

```Rust
fn my_func(some_string: String, some_int: u32) {}
```

You can call it from the frontend with:

```Typescript
await invoke("my_func", { someString: "Hello, World!", someInt: random_int })
```

Note the change between `snake_case` and `camelCase` in the function argument names. This is imposed by Tauri.

For returning values after the function call using the `Result<T, E>` type in Rust is recommended.

This means you'll have a function

```Rust
fn other_func() -> Result<u32, String> {}
```

which returns `Result<u32, String>`

Now in the frontend when calling it you can for example

```Typescript
await invoke("other_func")
  .then((message) => {
    // Success
    console.log(`Call returned: ${message}`)
  })
  .catch((error) => {
    // Error
    console.log(error)
  });
```

but also

```Typescript
// Store return in `result` on success
let result = await invoke("other_func")
  .catch((error) => {
    // Error
    console.log(error)
  });
```

For more info, see the Tauri docs: https://tauri.app/v1/guides/features/command/

For periodic calls between backend and frontend you can use events. See the Tauri docs here: https://tauri.app/v1/guides/features/events/

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
