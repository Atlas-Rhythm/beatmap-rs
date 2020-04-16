use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Deserialize, Serialize)]
pub struct Info {
    #[serde(rename = "_version")]
    version: String,

    #[serde(rename = "_songName")]
    song_name: String,
    #[serde(rename = "_songSubName")]
    song_sub_name: String,
    #[serde(rename = "_songAuthorName")]
    song_author_name: String,
    #[serde(rename = "_levelAuthorBane")]
    level_author_name: String,

    #[serde(rename = "_beatsPerMinute")]
    beats_per_minute: f64,
    #[serde(rename = "_songTimeOffset")]
    song_time_offset: f64,
    #[serde(rename = "_shuffle")]
    shuffle: f64,
    #[serde(rename = "_shufflePeriod")]
    shuffle_period: f64,

    #[serde(rename = "_previewStartTime")]
    preview_start_time: f64,
    #[serde(rename = "_previewDuration")]
    preview_duration: f64,

    #[serde(rename = "_songFilename")]
    song_filename: String,
    #[serde(rename = "_coverImageFilename")]
    cover_image_filename: String,

    #[serde(rename = "_environmentName")]
    environment_name: String,
    #[serde(
        rename = "_allDirectionsEncironmentName",
        skip_serializing_if = "Option::is_none"
    )]
    all_directions_environment_name: Option<String>,

    #[serde(rename = "_customData", skip_serializing_if = "Option::is_none")]
    custom_data: Option<InfoCustomData>,

    #[serde(rename = "_difficultyBeatmapSets")]
    difficulty_beatmap_sets: Vec<DifficultyBeatmapSet>,
}

#[derive(Deserialize, Serialize)]
pub struct InfoCustomData {
    #[serde(
        rename = "_difficultyBeatmapSets",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    contributors: Vec<Contributor>,

    #[serde(
        rename = "_difficultyBeatmapSets",
        skip_serializing_if = "Option::is_none"
    )]
    custom_environment: Option<String>,
    #[serde(
        rename = "_difficultyBeatmapSets",
        skip_serializing_if = "Option::is_none"
    )]
    custom_environment_hash: Option<String>,

    #[serde(flatten, default)]
    custom: Map<String, Value>,
}

#[derive(Deserialize, Serialize)]
pub struct Contributor {
    #[serde(rename = "_role")]
    role: String,
    #[serde(rename = "_name")]
    name: String,
    #[serde(rename = "_iconPath", skip_serializing_if = "Option::is_none")]
    icon_path: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct DifficultyBeatmapSet {
    #[serde(rename = "_beatmapCharacteristicName")]
    beatmap_characteristic_name: String,
    #[serde(rename = "_difficultyBeatmaps")]
    difficulty_beatmaps: Vec<DifficultyBeatmap>,
}

#[derive(Deserialize, Serialize)]
pub struct DifficultyBeatmap {
    #[serde(rename = "_difficulty")]
    difficulty: Difficulty,
    #[serde(rename = "_difficultyRank")]
    difficulty_rank: DifficultyRank,
    #[serde(rename = "_beatmapFilename")]
    beatmap_filename: String,

    #[serde(rename = "_noteJumpMovementSpeed")]
    note_jump_movement_speed: f64,
    #[serde(rename = "_noteJumpStartBeatOffset")]
    note_jump_start_beat_offset: f64,

    #[serde(rename = "_customData")]
    custom_data: Option<DifficultyBeatmapCustomData>,
}

#[derive(Deserialize, Serialize)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
    Expert,
    ExpertPlus,
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr)]
pub enum DifficultyRank {
    Easy = 1,
    Normal = 3,
    Hard = 5,
    Expert = 7,
    ExpertPlus = 9,
}

#[derive(Deserialize, Serialize)]
pub struct DifficultyBeatmapCustomData {
    #[serde(rename = "_difficultyLabel", skip_serializing_if = "Option::is_none")]
    difficulty_label: Option<String>,

    #[serde(rename = "_editorOffset", skip_serializing_if = "Option::is_none")]
    editor_offset: Option<i32>,
    #[serde(rename = "_editorOldOffset", skip_serializing_if = "Option::is_none")]
    editor_old_offset: Option<i32>,

    #[serde(rename = "_colorLeft", skip_serializing_if = "Option::is_none")]
    color_left: Option<Color>,
    #[serde(rename = "_colorRight", skip_serializing_if = "Option::is_none")]
    color_right: Option<Color>,

    #[serde(rename = "_warnings", default, skip_serializing_if = "Vec::is_empty")]
    warnings: Vec<String>,
    #[serde(
        rename = "_information",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    information: Vec<String>,
    #[serde(
        rename = "_suggestions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    suggestions: Vec<String>,
    #[serde(
        rename = "_requirements",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    requirements: Vec<String>,

    #[serde(flatten, default)]
    custom: Map<String, Value>,
}

#[derive(Deserialize, Serialize)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}
