#![feature(iterator_try_collect)]
mod data;
mod types;
use tauri_plugin_fs::FsExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            let scope = app.fs_scope();
            scope
                .allow_directory("/mind-quest", false)
                .expect("Cannot get scope");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
