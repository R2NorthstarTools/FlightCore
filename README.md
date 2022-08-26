
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

### Building

Release builds are generally done via CI. To build locally, make sure typescript is compiled (`./node_modules/.bin/rollup --config`), then run `npm run tauri build`.
