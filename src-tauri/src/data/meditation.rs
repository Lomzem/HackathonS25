use crate::types::Id;
use native_db::{native_db, Database, ToKey};
use native_model::{native_model, Model};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Styles {
    BodyScan,
    Breath,
    Visualization,
    SelfCompassion,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[native_model(id = 1, version = 1)]
#[native_db]
pub struct Meditation {
    #[primary_key]
    pub id: Id,
    pub meditaion_style: Styles,
    pub uri: String,
    pub file_name: String,
    pub is_favorite: bool,
    pub length: u32,
}

impl Meditation {
    fn defaults() -> Vec<Self> {
        vec![todo!()]
    }
}

fn _list_meditations(db: &Database) -> Result<Vec<Meditation>, String> {
    let r = db
        .r_transaction()
        .expect("Failed to create a rw transaction");

    let meditations: Vec<Meditation> = r
        .scan()
        .primary()
        .expect("fail to scan primary")
        .all()
        .expect("failed to scan all")
        .try_collect()
        .expect("Failed");
    Ok(meditations)
}

#[tauri::command]
pub fn list_meditations(db: tauri::State<Database>) -> Result<Vec<Meditation>, String> {
    log::info!("Listing meditations");
    _list_meditations(db.inner())
}

fn _toggle_fav_meditation(id: Id, db: &Database) -> Result<(), ()> {
    let rw = db
        .rw_transaction()
        .expect("Failed to create a rw transaction");

    let old: Meditation = rw
        .get()
        .primary(id)
        .expect("Database Error")
        .expect("Not found");

    rw.update(
        old.clone(),
        Meditation {
            is_favorite: !old.is_favorite,
            ..old
        },
    )
    .expect("Failed to update");

    rw.commit().unwrap();

    Ok(())
}

#[tauri::command]
pub fn toggle_fav_meditation(id: Id, db: tauri::State<Database>) -> Result<(), String> {
    log::info!("Toggling fav meditation");
    _toggle_fav_meditation(id, db.inner()).map_err(|_| "Failed to toggle".to_string())
}
