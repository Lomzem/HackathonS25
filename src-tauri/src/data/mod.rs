use std::sync::LazyLock;

use native_db::Models;

pub mod affirmations;
pub mod exercise;
pub mod meditation;

pub static DATABASE_MODELS: LazyLock<Models> = LazyLock::new(|| {
    let mut models = Models::new();
    models.define::<exercise::Exercise>().unwrap();
    models.define::<affirmations::Affirmation>().unwrap();
    models
});
