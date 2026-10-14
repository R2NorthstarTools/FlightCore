use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};

pub fn plugin_prompt(app: &tauri::AppHandle, plugin_paths: &[String]) -> bool {
    app.dialog()
        .message(plugin_paths.join("\n"))
        .title("Install native plugins?")
        .kind(MessageDialogKind::Warning)
        .buttons(MessageDialogButtons::OkCancelCustom(
            "Install".into(),
            "Cancel".into(),
        ))
        .blocking_show()
}
