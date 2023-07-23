use tauri::api::dialog::blocking::MessageDialogBuilder;
use tauri::api::dialog::{MessageDialogButtons, MessageDialogKind};

/// Prompt on plugin
/// Returns:
/// - true: user accepted plugin install
/// - false: user denied plugin install
pub fn plugin_prompt() -> bool {
    let dialog = MessageDialogBuilder::new(
        "Plugin in package detected",
        "This mod contains a plugin. Plugins have unrestricted access to your computer!
        \nMake sure you trust the author!
        \n
        \nPress 'Ok' to continue or 'Cancel' to abort mod installation",
    )
    .kind(MessageDialogKind::Warning)
    .buttons(MessageDialogButtons::OkCancel);

    if dialog.show() {
        log::info!("Accepted plugin install");
        true
    } else {
        log::warn!("Plugin install cancelled");
        false
    }
}
