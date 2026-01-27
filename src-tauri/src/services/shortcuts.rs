use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

/// Helper function to create the input window programmatically.
/// Calling getCurrentWindow().close() from webview frontend results in destroying initial input window
// TODO: Change to use config using from_config fn
fn create_input_window(app_handle: &AppHandle) -> tauri::Result<()> {
    WebviewWindowBuilder::new(app_handle, "input", WebviewUrl::App("input.html".into()))
        .title("Input Note")
        .inner_size(600.0, 200.0)
        .resizable(false)
        .decorations(false)
        .center()
        .always_on_top(true)
        .skip_taskbar(true)
        .build()?;

    Ok(())
}

/// Registers the global shortcut to show the input window.
pub fn register_global_shortcut(app_handle: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // Combines CMD (Mac) / CTRL (Win/Linux) + SHIFT + N
    let modifier = if cfg!(target_os = "macos") {
        Modifiers::SUPER | Modifiers::SHIFT
    } else {
        Modifiers::CONTROL | Modifiers::SHIFT
    };

    let shortcut = Shortcut::new(Some(modifier), Code::KeyN);

    // Clone the handle for use inside the closure
    let handle = app_handle.clone();

    // Register the shortcut and attach the handler.
    app_handle
        .global_shortcut()
        .on_shortcut(shortcut, move |_app, _shortcut, _event| {
            // Check if window already exists in memory
            if let Some(window) = handle.get_webview_window("input") {
                // 1. If it exists, just show and focus it
                let _ = window.show();
                let _ = window.set_focus();
            } else {
                // 2. If it doesn't exist (was closed), recreate it
                match create_input_window(&handle) {
                    Ok(_) => println!("Input window created."),
                    Err(e) => eprintln!("Failed to create input window: {}", e),
                }
            }
        })?;

    println!("Global shortcut 'CmdOrControl+Shift+N' registered.");
    Ok(())
}
