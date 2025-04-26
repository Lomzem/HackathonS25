use crate::types::Id;
use native_db::{native_db, Database, ToKey};
use native_model::{native_model, Model};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Styles {
    BodyScan,
    BodyAwareness,
    Breathwork,
    Mindfulness,
    Visualization,
    SelfCompassion,
    EmotionalProcessing,
    Acceptance,
    Relaxation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[native_model(id = 2, version = 1)]
#[native_db]
pub struct Meditation {
    #[primary_key]
    pub id: Id,
    pub meditaion_style: Vec<Styles>,
    pub uri: String,
    pub name: String,
    pub is_favorite: bool,
    pub length: u32,
}

impl Meditation {
    pub fn defaults() -> Vec<Self> {
        vec![
            Self {
                id: Uuid::new_v4().into(),
                uri: "https://mind-quest.werdxz.info/meditations/5countbreath.mp3".into(),
                name: "5-Count Breath Meditation".into(),
                meditaion_style: vec![Styles::Breathwork, Styles::Mindfulness],
                is_favorite: false,
                length: 306,
            },
            Self {
                id: Uuid::new_v4().into(),
                uri: "https://mind-quest.werdxz.info/meditations/acceptanceofemotion.mp3".into(),
                name: "Acceptance of Emotion".into(),
                meditaion_style: vec![Styles::EmotionalProcessing, Styles::Acceptance],
                is_favorite: false,
                length: 727,
            },
            Self {
                id: Uuid::new_v4().into(),
                uri: "https://mind-quest.werdxz.info/meditations/allowemotionunhookfromthought.mp3".into(),
                name: "Unhook Emotion from Thought".into(),
                meditaion_style: vec![Styles::EmotionalProcessing, Styles::Mindfulness],
                is_favorite: false,
                length: 576,
            },
            Self {
                id: Uuid::new_v4().into(),
                uri: "https://mind-quest.werdxz.info/meditations/awareness.mp3".into(),
                name: "Awareness Practice".into(),
                meditaion_style: vec![Styles::Mindfulness],
                is_favorite: false,
                length: 1021,
            },
            Self {
                id: Uuid::new_v4().into(),
                uri: "https://mind-quest.werdxz.info/meditations/awarenesstps.mp3".into(),
                name: "Awareness Through Physical Sensation".into(),
                meditaion_style: vec![Styles::BodyAwareness, Styles::Mindfulness],
                is_favorite: false,
                length: 724,
            },
            Self {
                id: Uuid::new_v4().into(),
                uri: "https://mind-quest.werdxz.info/meditations/bodyscanallowmeditationkw.mp3".into(),
                name: "Body Scan: Allow and Observe".into(),
                meditaion_style: vec![Styles::BodyAwareness, Styles::Acceptance],
                is_favorite: false,
                length: 777,
            },
            Self {
                id: Uuid::new_v4().into(),
                uri: "https://mind-quest.werdxz.info/meditations/breathmeditationkw.mp3".into(),
                name: "Breath Meditation".into(),
                meditaion_style: vec![Styles::Breathwork, Styles::Mindfulness],
                is_favorite: false,
                length: 600,
            },
            Self {
                id: Uuid::new_v4().into(),
                uri: "https://mind-quest.werdxz.info/meditations/comehometoyourselftalkkw.mp3".into(),
                name: "Come Home to Yourself".into(),
                meditaion_style: vec![Styles::SelfCompassion, Styles::Mindfulness],
                is_favorite: false,
                length: 1207,
            },
            Self {
                id: Uuid::new_v4().into(),
                uri: "https://mind-quest.werdxz.info/meditations/compassion.mp3".into(),
                name: "Cultivating Compassion".into(),
                meditaion_style: vec![Styles::SelfCompassion, Styles::EmotionalProcessing],
                is_favorite: false,
                length: 1801,
            },
            Self {
                id: Uuid::new_v4().into(),
                uri: "https://mind-quest.werdxz.info/meditations/Affectionate-Breathing-April2024.mp3".into(),
                name: "Affectionate Breathing".into(),
                meditaion_style: vec![Styles::Breathwork, Styles::SelfCompassion],
                is_favorite: false,
                length: 1115,
            },
            Self {
                id: Uuid::new_v4().into(),
                uri: "https://mind-quest.werdxz.info/meditations/Balancing-Yin-and-Yang-April2024.mp3".into(),
                name: "Balancing Yin and Yang".into(),
                meditaion_style: vec![Styles::Mindfulness, Styles::EmotionalProcessing],
                is_favorite: false,
                length: 803,
            },
            Self {
                id: Uuid::new_v4().into(),
                uri: "https://mind-quest.werdxz.info/meditations/Compassionate-Body-Scan-April2024.mp3".into(),
                name: "Compassionate Body Scan".into(),
                meditaion_style: vec![Styles::BodyAwareness, Styles::SelfCompassion],
                is_favorite: false,
                length: 1361,
            },
            Self {
                id: Uuid::new_v4().into(),
                uri: "https://mind-quest.werdxz.info/meditations/Compassionate-Friend-March2024.mp3".into(),
                name: "Compassionate Friend Practice".into(),
                meditaion_style: vec![Styles::SelfCompassion, Styles::EmotionalProcessing],
                is_favorite: false,
                length: 1072,
            },
            Self {
                id: Uuid::new_v4().into(),
                uri: "https://mind-quest.werdxz.info/meditations/Fierce-Friend-April2024.mp3".into(),
                name: "Fierce Friend Practice".into(),
                meditaion_style: vec![Styles::SelfCompassion, Styles::EmotionalProcessing],
                is_favorite: false,
                length: 916,
            },
            Self {
                id: Uuid::new_v4().into(),
                uri: "https://mind-quest.werdxz.info/meditations/General-Self-Compassion-Break-March2024.mp3".into(),
                name: "Self-Compassion Break".into(),
                meditaion_style: vec![Styles::SelfCompassion, Styles::Mindfulness],
                is_favorite: false,
                length: 313,
            },
            Self {
                id: Uuid::new_v4().into(),
                uri: "https://mind-quest.werdxz.info/meditations/Giving-and-Receiving-Compassion-April2024.mp3".into(),
                name: "Giving and Receiving Compassion".into(),
                meditaion_style: vec![Styles::SelfCompassion, Styles::EmotionalProcessing],
                is_favorite: false,
                length: 1230,
            },
            Self {
                id: Uuid::new_v4().into(),
                uri: "https://mind-quest.werdxz.info/meditations/Loving-Kindness-Meditation-April2024.mp3".into(),
                name: "Loving-Kindness Meditation".into(),
                meditaion_style: vec![Styles::SelfCompassion, Styles::Mindfulness],
                is_favorite: false,
                length: 1203,
            },
            Self {
                id: Uuid::new_v4().into(),
                uri: "https://mind-quest.werdxz.info/meditations/Noting-Practice-March2024.mp3".into(),
                name: "Noting Practice".into(),
                meditaion_style: vec![Styles::Mindfulness],
                is_favorite: false,
                length: 1123,
            },
            Self {
                id: Uuid::new_v4().into(),
                uri: "https://mind-quest.werdxz.info/meditations/Providing-Self-Compassion-Break-March2024-1.mp3".into(),
                name: "Providing a Self-Compassion Break".into(),
                meditaion_style: vec![Styles::SelfCompassion, Styles::Mindfulness],
                is_favorite: false,
                length: 406,
            },
            Self {
                id: Uuid::new_v4().into(),
                uri: "https://mind-quest.werdxz.info/meditations/01_Breathing_Meditation.mp3".into(),
                name: "Breathing Meditation".into(),
                meditaion_style: vec![Styles::Breathwork, Styles::Mindfulness],
                is_favorite: false,
                length: 331,
            },
            Self {
                id: Uuid::new_v4().into(),
                uri: "https://mind-quest.werdxz.info/meditations/02_Breath_Sound_Body_Meditation.mp3".into(),
                name: "Breath, Sound, and Body Meditation".into(),
                meditaion_style: vec![Styles::Breathwork, Styles::BodyAwareness, Styles::Mindfulness],
                is_favorite: false,
                length: 720,
            },
            Self {
                id: Uuid::new_v4().into(),
                uri: "https://mind-quest.werdxz.info/meditations/03_Complete_Meditation_Instructions.mp3".into(),
                name: "Complete Meditation Instructions".into(),
                meditaion_style: vec![Styles::Mindfulness],
                is_favorite: false,
                length: 1140,
            },
            Self {
                id: Uuid::new_v4().into(),
                uri: "https://mind-quest.werdxz.info/meditations/04_Meditation_for_Working_with_Difficulties.mp3".into(),
                name: "Meditation for Working with Difficulties".into(),
                meditaion_style: vec![Styles::EmotionalProcessing, Styles::Acceptance],
                is_favorite: false,
                length: 414,
            },
            Self {
                id: Uuid::new_v4().into(),
                uri: "https://mind-quest.werdxz.info/meditations/05_Loving_Kindness_Meditation.mp3".into(),
                name: "Loving-Kindness Meditation".into(),
                meditaion_style: vec![Styles::SelfCompassion, Styles::Mindfulness],
                is_favorite: false,
                length: 571,
            },
            Self {
                id: Uuid::new_v4().into(),
                uri: "https://mind-quest.werdxz.info/meditations/Body-Scan-for-Sleep.mp3".into(),
                name: "Body Scan for Sleep".into(),
                meditaion_style: vec![Styles::BodyAwareness, Styles::Mindfulness, Styles::Relaxation],
                is_favorite: false,
                length: 829,
            },
            Self {
                id: Uuid::new_v4().into(),
                uri: "https://mind-quest.werdxz.info/meditations/Body-Scan-Meditation.mp3".into(),
                name: "Body Scan Meditation".into(),
                meditaion_style: vec![Styles::BodyAwareness, Styles::Mindfulness],
                is_favorite: false,
                length: 164,
            },
            Self {
                id: Uuid::new_v4().into(),
                uri: "https://mind-quest.werdxz.info/meditations/Body-Sound-Meditation.mp3".into(),
                name: "Body and Sound Meditation".into(),
                meditaion_style: vec![Styles::BodyAwareness, Styles::Mindfulness],
                is_favorite: false,
                length: 186,
            },
        ]
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
