use chrono::{DateTime, Duration, Local};
use native_db::{native_db, Database, Key, ToKey};
use native_model::{native_model, Model};
use serde::{Deserialize, Serialize};

use crate::types::Id;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ExerciseType {
    Walk,
    Hike,
    Gym,
}

/// TODO: make another primary key
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[native_model(id = 1, version = 1)]
#[native_db]
pub struct Exercise {
    #[primary_key]
    pub id: Id,
    pub exercise_type: ExerciseType,
    pub age: i32,
    pub datetime: DateTime<Local>,
    pub duration: Duration,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExerciseBuilder {
    pub exercise_type: ExerciseType,
    pub age: i32,
    pub datetime: DateTime<Local>,
    pub duration: Duration,
}

impl ExerciseBuilder {
    pub fn build(self) -> Exercise {
        Exercise {
            id: Id::from(uuid::Uuid::new_v4()),
            exercise_type: self.exercise_type,
            age: self.age,
            datetime: self.datetime,
            duration: self.duration,
        }
    }
}

impl ToKey for ExerciseType {
    fn to_key(&self) -> Key {
        match self {
            ExerciseType::Walk => Key::new(vec![0u8]),
            ExerciseType::Hike => Key::new(vec![1u8]),
            ExerciseType::Gym => Key::new(vec![2u8]),
        }
    }

    fn key_names() -> Vec<String> {
        vec!["exercise_type".into()]
    }
}

fn _add_exercise(exercise: Exercise, db: &Database) -> Result<(), String> {
    log::info!("Adding exercise: {exercise:?}");
    let rw = db
        .rw_transaction()
        .expect("Failed to create a rw transaction");
    rw.insert(exercise).expect("Failed to save exercise");
    rw.commit().expect("failed to commit");
    Ok(())
}

/// TODO: Finish this function that will give you all exercises
fn _read_exercise(db: &Database) -> Vec<Exercise> {
    log::info!("Reading exercises");
    let r = db
        .r_transaction()
        .expect("Failed to create a r transaction");
    let exercises: Vec<Exercise> = r
        .scan()
        .primary()
        .expect("Failed to scan exercises")
        .all()
        .expect("Failed to get all exercises")
        .map(|result| result.expect("Failed to get exercise"))
        .collect();
    exercises
}

#[tauri::command]
pub fn add_exercise(exercise: ExerciseBuilder, db: tauri::State<Database>) -> Result<(), String> {
    log::info!("Adding exercise: {exercise:?}");
    _add_exercise(exercise.build(), db.inner())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_simple_insertions() {
        let db = native_db::Builder::new()
            .create_in_memory(&crate::data::DATABASE_MODELS)
            .unwrap();

        super::_add_exercise(
            super::Exercise {
                id: uuid::Uuid::new_v4().into(),
                exercise_type: super::ExerciseType::Walk,
                age: 30,
                datetime: chrono::Local::now(),
                duration: chrono::Duration::seconds(3600),
            },
            &db,
        )
        .unwrap();
    }

    #[test]
    fn test_simple_read() {
        let db = native_db::Builder::new()
            .create_in_memory(&crate::data::DATABASE_MODELS)
            .unwrap();

        let exercise = super::Exercise {
            id: uuid::Uuid::new_v4().into(),
            exercise_type: super::ExerciseType::Walk,
            age: 30,
            datetime: chrono::Local::now(),
            duration: chrono::Duration::seconds(3600),
        };

        super::_add_exercise(exercise.clone(), &db).unwrap();
        super::_read_exercise(&db);
        assert_eq!(super::_read_exercise(&db).len(), 1);
        assert_eq!(super::_read_exercise(&db)[0], exercise);
    }
}
