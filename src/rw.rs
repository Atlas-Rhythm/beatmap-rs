use crate::{info::Difficulty as InfoDifficulty, Difficulty, Info};
use std::{
    collections::HashMap,
    convert::TryInto,
    io::{Read, Seek, Write},
};
use thiserror::Error;
use zip::{ZipArchive, ZipWriter};

#[derive(Debug, Clone, PartialEq)]
pub struct Beatmap {
    pub info: Info,
    pub song: Vec<u8>,
    pub cover: Vec<u8>,
    pub sets: HashMap<String, HashMap<InfoDifficulty, Difficulty>>,
}

impl Beatmap {
    pub fn read<R: Read + Seek>(reader: R) -> Result<Self, Error> {
        let mut zip = ZipArchive::new(reader)?;

        let info: Info = {
            let mut file = zip.by_name("info.dat")?;
            serde_json::from_reader(&mut file)?
        };

        let song = {
            let filename = &info.song_filename;
            let mut file = zip.by_name(filename)?;
            let mut buffer = Vec::with_capacity(file.size().try_into()?);
            file.read_to_end(&mut buffer)?;
            buffer
        };

        let cover = {
            let filename = &info.cover_image_filename;
            let mut file = zip.by_name(filename)?;
            let mut buffer = Vec::with_capacity(file.size().try_into()?);
            file.read_to_end(&mut buffer)?;
            buffer
        };

        let mut sets = HashMap::with_capacity(info.difficulty_beatmap_sets.len());
        for set in &info.difficulty_beatmap_sets {
            let name = set.beatmap_characteristic_name.clone();

            let mut difficulties = HashMap::with_capacity(set.difficulty_beatmaps.len());
            for map in &set.difficulty_beatmaps {
                let filename = &map.beatmap_filename;
                let mut file = zip.by_name(filename)?;

                let info_difficulty = map.difficulty;

                let difficulty = serde_json::from_reader(&mut file)?;

                difficulties.insert(info_difficulty, difficulty);
            }

            sets.insert(name, difficulties);
        }

        Ok(Self {
            info,
            song,
            cover,
            sets,
        })
    }

    pub fn write<W: Write + Seek>(&self, writer: W) -> Result<(), Error> {
        let mut zip = ZipWriter::new(writer);

        zip.start_file("info.dat", Default::default())?;
        serde_json::to_writer(&mut zip, &self.info)?;

        let song_filename = &self.info.song_filename;
        zip.start_file(song_filename, Default::default())?;
        zip.write_all(&self.song)?;

        let cover_filename = &self.info.cover_image_filename;
        zip.start_file(cover_filename, Default::default())?;
        zip.write_all(&self.cover)?;

        for (name, difficulties) in &self.sets {
            let info_set = self
                .info
                .difficulty_beatmap_sets
                .iter()
                .find(|s| &s.beatmap_characteristic_name == name)
                .ok_or(Error::InconsistentBeatmap)?;

            for (difficulty, map) in difficulties {
                let info_map = info_set
                    .difficulty_beatmaps
                    .iter()
                    .find(|m| &m.difficulty == difficulty)
                    .ok_or(Error::InconsistentBeatmap)?;

                let filename = &info_map.beatmap_filename;
                zip.start_file(filename, Default::default())?;
                serde_json::to_writer(&mut zip, map)?;
            }
        }

        zip.finish()?;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("io error: {0}")]
    IO(#[from] std::io::Error),
    #[error("zip error: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("overflow error: {0}")]
    Overflow(#[from] std::num::TryFromIntError),
    #[error("inconsistent beatmap info and sets")]
    InconsistentBeatmap,
}
