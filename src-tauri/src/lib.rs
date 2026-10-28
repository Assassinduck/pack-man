// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod libs;

use libs::jsr_api::request_package;
use tauri_plugin_sentry::Sentry;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let sentry_plugin = Sentry::new();

    let options = sentry::ClientOptions::new()
        .add_integration(sentry_plugin.clone())
        .dsn("__YOUR_DSN__")
        .maybe_release(sentry::release_name!())
        .auto_session_tracking(true);

    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
    let options =
        options.add_integration(sentry::integrations::minidump::MinidumpIntegration::new());

    // Caution! Everything before here runs in both app and crash reporter processes
    let _guard = sentry::init(options);

    tauri::Builder::default()
        .plugin(sentry_plugin)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![request_package])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
