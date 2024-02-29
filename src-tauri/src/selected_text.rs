use std::{thread, time::Duration};

use tauri::AppHandle;
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
use tauri_plugin_shell::ShellExt;

async fn show_dialog(app: &AppHandle, message: &str, title: &str, kind: MessageDialogKind) {
    let _ = app.dialog()
        .message(message)
        .kind(kind)
        .title(title)
        .show(|result| match result {
            true => {},
            false => {},
        });
}


pub async fn get_selected_text(app: &AppHandle) -> Result<String, String> {
    const COPY_APPLE_SCRIPT: &str = r#"tell application "System Events" to keystroke "c" using command down"#;

  