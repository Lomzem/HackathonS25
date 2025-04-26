use std::sync::LazyLock;

use native_db::Models;

pub mod exercise;

static DATABASE_MODELS: LazyLock<Models> = LazyLock::new(|| {
    let mut models = Models::new();
    models.define::<exercise::Exercise>().unwrap();
    models
});
