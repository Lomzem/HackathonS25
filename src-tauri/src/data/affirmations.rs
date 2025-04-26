use crate::types::Id;
use native_db::{native_db, Database, ToKey};
use native_model::{native_model, Model};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Category {
    SelfLoveAndConfidence,
    SuccessAndAbundance,
    HealthAndWellness,
    PositivityAndGratitude,
    CourageAndResilience,
    RelationshipsAndLove,
    CreativityAndFocus,
    LettingGoAndTrustingTheProcess,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[native_model(id = 3, version = 1)]
#[native_db]
pub struct Affirmation {
    #[primary_key]
    pub id: Id,
    pub category: Category,
    pub text: String,
    pub is_favorite: bool,
}

impl Affirmation {
    pub fn defaults() -> Vec<Self> {
        vec![
            // Self-Love & Confidence
            Self {
                id: Uuid::new_v4().into(),
                category: Category::SelfLoveAndConfidence,
                text: "I am worthy of love, respect, and happiness.".into(),
                is_favorite: false,
            },
            Self {
                id: Uuid::new_v4().into(),
                category: Category::SelfLoveAndConfidence,
                text: "I embrace my flaws and celebrate my strengths.".into(),
                is_favorite: false,
            },
            Self {
                id: Uuid::new_v4().into(),
                category: Category::SelfLoveAndConfidence,
                text: "I am enough just as I am.".into(),
                is_favorite: false,
            },
            Self {
                id: Uuid::new_v4().into(),
                category: Category::SelfLoveAndConfidence,
                text: "I radiate confidence and self-assurance.".into(),
                is_favorite: false,
            },
            Self {
                id: Uuid::new_v4().into(),
                category: Category::SelfLoveAndConfidence,
                text: "My self-worth is not defined by others' opinions.".into(),
                is_favorite: false,
            },
            // Success & Abundance
            Self {
                id: Uuid::new_v4().into(),
                category: Category::SuccessAndAbundance,
                text: "I attract success and opportunities effortlessly.".into(),
                is_favorite: false,
            },
            Self {
                id: Uuid::new_v4().into(),
                category: Category::SuccessAndAbundance,
                text: "I am capable of achieving my biggest dreams.".into(),
                is_favorite: false,
            },
            Self {
                id: Uuid::new_v4().into(),
                category: Category::SuccessAndAbundance,
                text: "Money flows to me easily and abundantly.".into(),
                is_favorite: false,
            },
            Self {
                id: Uuid::new_v4().into(),
                category: Category::SuccessAndAbundance,
                text: "Every challenge is an opportunity for growth.".into(),
                is_favorite: false,
            },
            Self {
                id: Uuid::new_v4().into(),
                category: Category::SuccessAndAbundance,
                text: "I am in control of my destiny.".into(),
                is_favorite: false,
            },
            // Health & Wellness
            Self {
                id: Uuid::new_v4().into(),
                category: Category::HealthAndWellness,
                text: "My body is strong, healthy, and full of energy.".into(),
                is_favorite: false,
            },
            Self {
                id: Uuid::new_v4().into(),
                category: Category::HealthAndWellness,
                text: "I nourish my mind, body, and soul daily.".into(),
                is_favorite: false,
            },
            Self {
                id: Uuid::new_v4().into(),
                category: Category::HealthAndWellness,
                text: "Every cell in my body vibrates with health.".into(),
                is_favorite: false,
            },
            Self {
                id: Uuid::new_v4().into(),
                category: Category::HealthAndWellness,
                text: "I choose peace over stress.".into(),
                is_favorite: false,
            },
            Self {
                id: Uuid::new_v4().into(),
                category: Category::HealthAndWellness,
                text: "I am grateful for my vitality and well-being.".into(),
                is_favorite: false,
            },
            // Positivity & Gratitude
            Self {
                id: Uuid::new_v4().into(),
                category: Category::PositivityAndGratitude,
                text: "I choose joy in every moment.".into(),
                is_favorite: false,
            },
            Self {
                id: Uuid::new_v4().into(),
                category: Category::PositivityAndGratitude,
                text: "Gratitude fills my heart and attracts more blessings.".into(),
                is_favorite: false,
            },
            Self {
                id: Uuid::new_v4().into(),
                category: Category::PositivityAndGratitude,
                text: "I release negativity and embrace positivity.".into(),
                is_favorite: false,
            },
            Self {
                id: Uuid::new_v4().into(),
                category: Category::PositivityAndGratitude,
                text: "My life is filled with love and happiness.".into(),
                is_favorite: false,
            },
            Self {
                id: Uuid::new_v4().into(),
                category: Category::PositivityAndGratitude,
                text: "I am a magnet for good things.".into(),
                is_favorite: false,
            },
            // Courage & Resilience
            Self {
                id: Uuid::new_v4().into(),
                category: Category::CourageAndResilience,
                text: "I overcome obstacles with ease and grace.".into(),
                is_favorite: false,
            },
            Self {
                id: Uuid::new_v4().into(),
                category: Category::CourageAndResilience,
                text: "Fear does not control me; I am brave and bold.".into(),
                is_favorite: false,
            },
            Self {
                id: Uuid::new_v4().into(),
                category: Category::CourageAndResilience,
                text: "Every setback is a setup for a comeback.".into(),
                is_favorite: false,
            },
            Self {
                id: Uuid::new_v4().into(),
                category: Category::CourageAndResilience,
                text: "I trust in my ability to handle anything.".into(),
                is_favorite: false,
            },
            Self {
                id: Uuid::new_v4().into(),
                category: Category::CourageAndResilience,
                text: "I am unstoppable.".into(),
                is_favorite: false,
            },
            // Relationships & Love
            Self {
                id: Uuid::new_v4().into(),
                category: Category::RelationshipsAndLove,
                text: "I attract healthy, loving relationships.".into(),
                is_favorite: false,
            },
            Self {
                id: Uuid::new_v4().into(),
                category: Category::RelationshipsAndLove,
                text: "I communicate with kindness and confidence.".into(),
                is_favorite: false,
            },
            Self {
                id: Uuid::new_v4().into(),
                category: Category::RelationshipsAndLove,
                text: "I deserve love that is deep and unconditional.".into(),
                is_favorite: false,
            },
            Self {
                id: Uuid::new_v4().into(),
                category: Category::RelationshipsAndLove,
                text: "I set boundaries with love and respect.".into(),
                is_favorite: false,
            },
            Self {
                id: Uuid::new_v4().into(),
                category: Category::RelationshipsAndLove,
                text: "My heart is open to giving and receiving love.".into(),
                is_favorite: false,
            },
            // Creativity & Focus
            Self {
                id: Uuid::new_v4().into(),
                category: Category::CreativityAndFocus,
                text: "My mind is full of brilliant ideas.".into(),
                is_favorite: false,
            },
            Self {
                id: Uuid::new_v4().into(),
                category: Category::CreativityAndFocus,
                text: "Creativity flows through me effortlessly.".into(),
                is_favorite: false,
            },
            Self {
                id: Uuid::new_v4().into(),
                category: Category::CreativityAndFocus,
                text: "I am focused, disciplined, and productive.".into(),
                is_favorite: false,
            },
            Self {
                id: Uuid::new_v4().into(),
                category: Category::CreativityAndFocus,
                text: "I trust my intuition and inner wisdom.".into(),
                is_favorite: false,
            },
            Self {
                id: Uuid::new_v4().into(),
                category: Category::CreativityAndFocus,
                text: "My potential is limitless.".into(),
                is_favorite: false,
            },
            // Letting Go & Trusting the Process
            Self {
                id: Uuid::new_v4().into(),
                category: Category::LettingGoAndTrustingTheProcess,
                text: "I release what no longer serves me.".into(),
                is_favorite: false,
            },
            Self {
                id: Uuid::new_v4().into(),
                category: Category::LettingGoAndTrustingTheProcess,
                text: "I trust the journey of life.".into(),
                is_favorite: false,
            },
            Self {
                id: Uuid::new_v4().into(),
                category: Category::LettingGoAndTrustingTheProcess,
                text: "Everything happens for my highest good.".into(),
                is_favorite: false,
            },
            Self {
                id: Uuid::new_v4().into(),
                category: Category::LettingGoAndTrustingTheProcess,
                text: "I surrender control and embrace peace.".into(),
                is_favorite: false,
            },
            Self {
                id: Uuid::new_v4().into(),
                category: Category::LettingGoAndTrustingTheProcess,
                text: "The universe has my back.".into(),
                is_favorite: false,
            },
        ]
    }
}

fn _list_affirmations(db: &Database) -> Result<Vec<Affirmation>, String> {
    let r = db
        .r_transaction()
        .expect("Failed to create a read transaction");

    let affirmations: Vec<Affirmation> = r
        .scan()
        .primary()
        .expect("fail to scan primary")
        .all()
        .expect("failed to scan all")
        .try_collect()
        .expect("Failed");
    Ok(affirmations)
}

#[tauri::command]
pub fn list_affirmations(db: tauri::State<Database>) -> Result<Vec<Affirmation>, String> {
    log::info!("Listing affirmations");
    _list_affirmations(db.inner())
}

#[tauri::command]
pub fn list_affirmations_by_category(
    category: Category,
    db: tauri::State<Database>,
) -> Result<Vec<Affirmation>, String> {
    log::info!("Listing affirmations by category: {category:?}");

    let affirmations = _list_affirmations(db.inner())?;
    let filtered = affirmations
        .into_iter()
        .filter(|a| a.category == category)
        .collect();

    Ok(filtered)
}

fn _toggle_fav_affirmation(id: Id, db: &Database) -> Result<(), ()> {
    let rw = db
        .rw_transaction()
        .expect("Failed to create a rw transaction");

    let old: Affirmation = rw
        .get()
        .primary(id)
        .expect("Database Error")
        .expect("Not found");

    rw.update(
        old.clone(),
        Affirmation {
            is_favorite: !old.is_favorite,
            ..old
        },
    )
    .expect("Failed to update");

    rw.commit().unwrap();

    Ok(())
}

#[tauri::command]
pub fn toggle_fav_affirmation(id: Id, db: tauri::State<Database>) -> Result<(), String> {
    log::info!("Toggling fav affirmation");
    _toggle_fav_affirmation(id, db.inner()).map_err(|_| "Failed to toggle".to_string())
}

#[tauri::command]
pub fn get_random_affirmation(db: tauri::State<Database>) -> Result<Affirmation, String> {
    log::info!("Getting a random affirmation");
    let affirmations = _list_affirmations(db.inner())?;

    if affirmations.is_empty() {
        return Err("No affirmations found".to_string());
    }

    let mut rng = rand::rng();
    let index = rand::Rng::random_range(&mut rng, 0..affirmations.len());

    Ok(affirmations[index].clone())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_list_affirmations() {
        let db = native_db::Builder::new()
            .create_in_memory(&crate::data::DATABASE_MODELS)
            .unwrap();

        // Initialize with defaults
        let defaults = Affirmation::defaults();

        let rw = db.rw_transaction().unwrap();
        for affirmation in defaults.clone() {
            rw.insert(affirmation).unwrap();
        }
        rw.commit().unwrap();

        let result = _list_affirmations(&db).unwrap();
        assert_eq!(result.len(), defaults.len());
    }

    #[test]
    fn test_toggle_favorite() {
        let db = native_db::Builder::new()
            .create_in_memory(&crate::data::DATABASE_MODELS)
            .unwrap();

        // Insert a test affirmation
        let affirmation = Affirmation {
            id: Uuid::new_v4().into(),
            category: Category::SelfLoveAndConfidence,
            text: "Test affirmation".into(),
            is_favorite: false,
        };

        let rw = db.rw_transaction().unwrap();
        rw.insert(affirmation.clone()).unwrap();
        rw.commit().unwrap();

        // Toggle favorite
        _toggle_fav_affirmation(affirmation.id.clone(), &db).unwrap();

        // Check that it's toggled
        let r = db.r_transaction().unwrap();
        let updated: Affirmation = r.get().primary(affirmation.id).unwrap().unwrap();
        assert!(updated.is_favorite);
    }
}
