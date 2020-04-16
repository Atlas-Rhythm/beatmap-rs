use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Deserialize, Serialize)]
pub struct Difficulty {
    #[serde(rename = "_version")]
    version: String,

    #[serde(rename = "_events")]
    events: Vec<Event>,
    #[serde(rename = "_notes")]
    notes: Vec<Note>,
    #[serde(rename = "_obstacles")]
    obstacles: Vec<Obstacle>,

    #[serde(rename = "_customData", default, skip_serializing_if = "Map::is_empty")]
    custom_data: Map<String, Value>,
}

#[derive(Deserialize, Serialize)]
pub struct Event {
    #[serde(rename = "_time")]
    time: f32,
    #[serde(rename = "_type")]
    ty: i32,
    #[serde(rename = "_value")]
    value: i32,

    #[serde(rename = "_customData", default, skip_serializing_if = "Map::is_empty")]
    custom_data: Map<String, Value>,
}

#[derive(Deserialize, Serialize)]
pub struct Note {
    #[serde(rename = "_time")]
    time: f32,
    #[serde(rename = "_lineIndex")]
    line_index: i32,
    #[serde(rename = "_lineLayer")]
    line_layer: i32,
    #[serde(rename = "_type")]
    ty: i32,
    #[serde(rename = "_cutDirection")]
    cut_direction: i32,

    #[serde(rename = "_customData", default, skip_serializing_if = "Map::is_empty")]
    custom_data: Map<String, Value>,
}

#[derive(Deserialize, Serialize)]
pub struct Obstacle {
    #[serde(rename = "_time")]
    time: f64,
    #[serde(rename = "_lineIndex")]
    line_index: i32,
    #[serde(rename = "_type")]
    ty: i32,
    #[serde(rename = "_duration")]
    duration: f32,
    #[serde(rename = "_width")]
    width: i32,

    #[serde(rename = "_customData", default, skip_serializing_if = "Map::is_empty")]
    custom_data: Map<String, Value>,
}
