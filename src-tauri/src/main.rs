#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{Manager, State};
use tokio::time::sleep;

#[derive(Default)]
struct Counter(Arc<Mutex<i32>>);

fn main() {
	tauri::Builder::default()
		.setup(|app| {
			let app_handle = app.app_handle();
			tauri::async_runtime::spawn(async move {
				loop {
					sleep(Duration::from_millis(2000)).await;
					println!("sending backend-ping");
					app_handle.emit_all("backend-ping", "ping").unwrap();
				}
			});

			Ok(())
		})
		.manage(Counter::default())
		.invoke_handler(tauri::generate_handler![hello_world, add_count])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}

#[tauri::command]
fn hello_world() -> String {
	"Hello World!!!!".to_string()
}

#[tauri::command]
fn add_count(num: i32, counter: State<'_, Counter>) -> String {
	let mut val = counter.0.lock().unwrap();
	*val += num;

	format!("{val}")
}
