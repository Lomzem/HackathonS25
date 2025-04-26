#![feature(iterator_try_collect)]
mod data;
use data::affirmations::{
    get_random_affirmation, list_affirmations, list_affirmations_by_category,
    toggle_fav_affirmation,
};
use data::exercise::add_exercise;
use data::meditation::{list_meditations, toggle_fav_meditation};
use native_db::Database;
mod types;
use tauri::Manager;
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
            let path = app.path().resolve("mind-quest", tauri::path::BaseDirectory::LocalData).unwrap();
            let scope = app.fs_scope();
            scope
                .allow_directory(path, false)
                .expect("Cannot get scope");
            Ok(())
        })
        .manage(create_db())
        .invoke_handler(tauri::generate_handler![
            add_exercise,
            list_affirmations,
            list_affirmations_by_category,
            toggle_fav_affirmation,
            get_random_affirmation,
            toggle_fav_meditation,
            list_meditations
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn create_db() -> Database<'static> {
    let db_path = std::path::Path::new("mind-quest.db");

    let db = if db_path.exists() {
        log::info!("Opening existing database");
        native_db::Builder::new()
            .open(&data::DATABASE_MODELS, db_path)
            .expect("Failed to open database")
    } else {
        log::info!("Creating new database");
        let db = native_db::Builder::new()
            .create(&data::DATABASE_MODELS, db_path)
            .expect("Failed to create database");

        // Initialize with default affirmations
        let affirmations = data::affirmations::Affirmation::defaults();
        let meditations = data::meditation::Meditation::defaults();
        let rw = db
            .rw_transaction()
            .expect("Failed to create rw transaction");

        for affirmation in affirmations {
            rw.insert(affirmation)
                .expect("Failed to insert affirmation");
        }

        for meditation in meditations {
            rw.insert(meditation).expect("Failed to insert meditation");
        }

        rw.commit().expect("Failed to commit transaction");
        db
    };
    db
}
