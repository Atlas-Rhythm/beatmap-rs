use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
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

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
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

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
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

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
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
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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

        VoidEvent,

        Unknown(i32),
    }

    impl EventType {
        pub fn is_bmp_change_event(self) -> bool {
            self == EventType::Event10
        }
        pub fn is_rotation_event(self) -> bool {
            self == EventType::Event14 || self == EventType::Event15
        }
        pub fn is_early_rotation_event(self) -> bool {
            self == EventType::Event14
        }
        pub fn is_late_rotation_event(self) -> bool {
            self == EventType::Event15
        }
        pub fn is_early_event(self) -> bool {
            self == EventType::Event10 || self == EventType::Event14
        }
        pub fn is_spawn_affecting_event(self) -> bool {
            self.is_rotation_event() || self.is_bmp_change_event()
        }
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

                -1 => EventType::VoidEvent,

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

                EventType::VoidEvent => -1,

                EventType::Unknown(n) => n,
            }
        }
    }

    impl super::Event {
        pub fn ty(&self) -> EventType {
            self.ty.into()
        }
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum LineLayer {
        Base,
        Upper,
        Top,

        Unknown(i32),
    }

    impl From<i32> for LineLayer {
        fn from(n: i32) -> Self {
            match n {
                0 => LineLayer::Base,
                1 => LineLayer::Upper,
                2 => LineLayer::Top,

                _ => LineLayer::Unknown(n),
            }
        }
    }
    impl From<LineLayer> for i32 {
        fn from(l: LineLayer) -> Self {
            match l {
                LineLayer::Base => 0,
                LineLayer::Upper => 1,
                LineLayer::Top => 2,

                LineLayer::Unknown(n) => n,
            }
        }
    }

    impl super::Note {
        pub fn line_layer(&self) -> LineLayer {
            self.line_layer.into()
        }
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum NoteType {
        NoteA,
        NoteB,
        GhostNote,
        Bomb,

        Unknown(i32),
    }

    impl NoteType {
        pub fn is_basic_note(self) -> bool {
            self == NoteType::NoteA || self == NoteType::NoteB
        }
    }

    impl From<i32> for NoteType {
        fn from(n: i32) -> Self {
            match n {
                0 => NoteType::NoteA,
                1 => NoteType::NoteB,
                2 => NoteType::GhostNote,
                3 => NoteType::Bomb,

                _ => NoteType::Unknown(n),
            }
        }
    }
    impl From<NoteType> for i32 {
        fn from(t: NoteType) -> Self {
            match t {
                NoteType::NoteA => 0,
                NoteType::NoteB => 1,
                NoteType::GhostNote => 2,
                NoteType::Bomb => 3,

                NoteType::Unknown(n) => n,
            }
        }
    }

    impl super::Note {
        pub fn ty(&self) -> NoteType {
            self.ty.into()
        }
    }

    pub enum ObstacleType {
        FullHeight,
        Top,

        Unknown(i32),
    }

    impl From<i32> for ObstacleType {
        fn from(n: i32) -> Self {
            match n {
                0 => ObstacleType::FullHeight,
                1 => ObstacleType::Top,

                _ => ObstacleType::Unknown(n),
            }
        }
    }
    impl From<ObstacleType> for i32 {
        fn from(t: ObstacleType) -> Self {
            match t {
                ObstacleType::FullHeight => 0,
                ObstacleType::Top => 1,

                ObstacleType::Unknown(n) => n,
            }
        }
    }

    impl super::Obstacle {
        pub fn ty(&self) -> ObstacleType {
            self.ty.into()
        }
    }
}
