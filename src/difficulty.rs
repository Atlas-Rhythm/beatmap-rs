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

#[cfg(feature = "extras")]
pub mod extras {
    pub enum EventType {
        Event0,
        Event1,
        Event2,
        Event3,
        Event4,
        Event5,
        Event6,
        Event7,
        Event8,
        Event9,
        Event10,
        Event11,
        Event12,
        Event13,
        Event14,
        Event15,

        Unknown(i32),
    }

    impl From<i32> for EventType {
        fn from(n: i32) -> Self {
            match n {
                0 => EventType::Event0,
                1 => EventType::Event1,
                2 => EventType::Event2,
                3 => EventType::Event3,
                4 => EventType::Event4,
                5 => EventType::Event5,
                6 => EventType::Event6,
                7 => EventType::Event7,
                8 => EventType::Event8,
                9 => EventType::Event9,
                10 => EventType::Event10,
                11 => EventType::Event11,
                12 => EventType::Event12,
                13 => EventType::Event13,
                14 => EventType::Event14,
                15 => EventType::Event15,

                _ => EventType::Unknown(n),
            }
        }
    }
    impl From<EventType> for i32 {
        fn from(t: EventType) -> Self {
            match t {
                EventType::Event0 => 0,
                EventType::Event1 => 1,
                EventType::Event2 => 2,
                EventType::Event3 => 3,
                EventType::Event4 => 4,
                EventType::Event5 => 5,
                EventType::Event6 => 6,
                EventType::Event7 => 7,
                EventType::Event8 => 8,
                EventType::Event9 => 9,
                EventType::Event10 => 10,
                EventType::Event11 => 11,
                EventType::Event12 => 12,
                EventType::Event13 => 13,
                EventType::Event14 => 14,
                EventType::Event15 => 15,

                EventType::Unknown(n) => n,
            }
        }
    }

    impl super::Event {
        pub fn ty(&self) -> EventType {
            self.ty.into()
        }
    }

    pub enum LineIndex {
        Unknown(i32),
    }

    pub enum LineLayer {
        Base,
        Top,
        Upper,

        Unknown(i32),
    }

    pub enum NoteType {
        Bomb,
        GhostNote,
        NoteA,
        NoteB,

        Unknown(i32),
    }

    pub enum ObstacleType {
        FullHeight,
        Top,

        Unknown(i32),
    }
}
