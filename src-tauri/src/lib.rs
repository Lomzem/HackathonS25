use native_db::{Database, Models};
use once_cell::sync::Lazy;
use tauri_plugin_fs::FsExt;

pub(crate) mod api {
    use native_db::{native_db, ToKey};
    use native_model::{native_model, Model};
    use serde::{Deserialize, Serialize};

    pub type Exercise = v1::Exercise;

    pub mod v1 {
        use chrono::{DateTime, Duration, Local};

        use super::*;

        #[derive(Serialize, Deserialize, Debug)]
        pub enum ExerciseType {
            Walk,
            Hike,
            Gym,
        }

        impl ToKey for ExerciseType {
            fn to_key(&self) -> native_db::Key {
                match self {
                    ExerciseType::Walk => native_db::Key::new("walk".into()),
                    ExerciseType::Hike => native_db::Key::new("hike".into()),
                    ExerciseType::Gym => native_db::Key::new("gym".into()),
                }
            }

            fn key_names() -> Vec<String> {
                vec!["exercise_type".into()]
            }
        }

        #[derive(Serialize, Deserialize, Debug)]
        #[native_model(id = 1, version = 1)]
        #[native_db]
        pub struct Exercise {
            #[primary_key]
            pub exercise_type: ExerciseType,
            pub age: i32,
            pub datetime: DateTime<Local>,
            pub duration: Duration,
        }
    }
}

static DATABASE_MODELS: Lazy<Models> = Lazy::new(|| {
    let mut models = Models::new();
    models.define::<api::v1::Exercise>().unwrap();
    models
});

#[tauri::command]
fn add_exercise(exercise: api::v1::Exercise, db: tauri::State<Database>) {
    println!("saving exercise: {:?}", exercise);
    let rw = db
        .rw_transaction()
        .expect("failed to create rw transaction");
    rw.insert(exercise).expect("failed to save exercise");
    rw.commit().expect("failed to commit");
    println!("saved exercise successfully");
}

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
