use crate::ext::DefaultPartialEq;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Info {
    #[serde(rename = "_version")]
    pub version: String,

    #[serde(rename = "_songName")]
    pub song_name: String,
    #[serde(rename = "_songSubName")]
    pub song_sub_name: String,
    #[serde(rename = "_songAuthorName")]
    pub song_author_name: String,
    #[serde(rename = "_levelAuthorName")]
    pub level_author_name: String,

    #[serde(rename = "_beatsPerMinute")]
    pub beats_per_minute: f32,
    #[serde(rename = "_songTimeOffset")]
    pub song_time_offset: f32,
    #[serde(rename = "_shuffle")]
    pub shuffle: f32,
    #[serde(rename = "_shufflePeriod")]
    pub shuffle_period: f32,

    #[serde(rename = "_previewStartTime")]
    pub preview_start_time: f32,
    #[serde(rename = "_previewDuration")]
    pub preview_duration: f32,

    #[serde(rename = "_songFilename")]
    pub song_filename: String,
    #[serde(rename = "_coverImageFilename")]
    pub cover_image_filename: String,

    #[serde(rename = "_environmentName")]
    pub environment_name: String,
    #[serde(
        rename = "_allDirectionsEncironmentName",
        skip_serializing_if = "Option::is_none"
    )]
    pub all_directions_environment_name: Option<String>,

    #[serde(
        rename = "_customData",
        default,
        skip_serializing_if = "DefaultPartialEq::is_default"
    )]
    pub custom_data: InfoCustomData,

    #[serde(rename = "_difficultyBeatmapSets")]
    pub difficulty_beatmap_sets: Vec<DifficultyBeatmapSet>,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq)]
pub struct InfoCustomData {
    #[serde(
        rename = "_difficultyBeatmapSets",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub contributors: Vec<Contributor>,

    #[serde(
        rename = "_difficultyBeatmapSets",
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_environment: Option<String>,
    #[serde(
        rename = "_difficultyBeatmapSets",
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_environment_hash: Option<String>,

    #[serde(flatten, default)]
    pub custom: Map<String, Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Hash)]
pub struct Contributor {
    #[serde(rename = "_role")]
    pub role: String,
    #[serde(rename = "_name")]
    pub name: String,
    #[serde(rename = "_iconPath", skip_serializing_if = "Option::is_none")]
    pub icon_path: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DifficultyBeatmapSet {
    #[serde(rename = "_beatmapCharacteristicName")]
    pub beatmap_characteristic_name: String,
    #[serde(rename = "_difficultyBeatmaps")]
    pub difficulty_beatmaps: Vec<DifficultyBeatmap>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DifficultyBeatmap {
    #[serde(rename = "_difficulty")]
    pub difficulty: Difficulty,
    #[serde(rename = "_difficultyRank")]
    pub difficulty_rank: DifficultyRank,
    #[serde(rename = "_beatmapFilename")]
    pub beatmap_filename: String,

    #[serde(rename = "_noteJumpMovementSpeed")]
    pub note_jump_movement_speed: f32,
    #[serde(rename = "_noteJumpStartBeatOffset")]
    pub note_jump_start_beat_offset: f32,

    #[serde(
        rename = "_customData",
        default,
        skip_serializing_if = "DefaultPartialEq::is_default"
    )]
    pub custom_data: DifficultyBeatmapCustomData,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
    Expert,
    ExpertPlus,
}

#[repr(i32)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum DifficultyRank {
    Easy = 1,
    Normal = 3,
    Hard = 5,
    Expert = 7,
    ExpertPlus = 9,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
pub struct DifficultyBeatmapCustomData {
    #[serde(rename = "_difficultyLabel", skip_serializing_if = "Option::is_none")]
    pub difficulty_label: Option<String>,

    #[serde(rename = "_editorOffset", skip_serializing_if = "Option::is_none")]
    pub editor_offset: Option<i32>,
    #[serde(rename = "_editorOldOffset", skip_serializing_if = "Option::is_none")]
    pub editor_old_offset: Option<i32>,

    #[serde(rename = "_colorLeft", skip_serializing_if = "Option::is_none")]
    pub color_left: Option<Color>,
    #[serde(rename = "_colorRight", skip_serializing_if = "Option::is_none")]
    pub color_right: Option<Color>,

    #[serde(rename = "_warnings", default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
    #[serde(
        rename = "_information",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub information: Vec<String>,
    #[serde(
        rename = "_suggestions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub suggestions: Vec<String>,
    #[serde(
        rename = "_requirements",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub requirements: Vec<String>,

    #[serde(flatten, default)]
    pub custom: Map<String, Value>,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}
