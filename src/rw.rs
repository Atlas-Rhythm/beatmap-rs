use crate::{info::Difficulty as InfoDifficulty, Difficulty, Info};
use std::{
    collections::HashMap,
    convert::TryInto,
    io::{Read, Seek, Write},
};
use thiserror::Error;
use zip::{read::ZipFile, result::ZipError, ZipArchive, ZipWriter};

#[derive(Debug, Clone, PartialEq)]
pub struct Beatmap {
    pub info: Info,
    pub song: Vec<u8>,
    pub cover: Vec<u8>,
    pub sets: HashMap<String, HashMap<InfoDifficulty, Difficulty>>,
}

fn zip_by_name<'a, R: Read + Seek>(
    zip: &'a mut ZipArchive<R>,
    name: &str,
) -> Result<ZipFile<'a>, Error> {
    match zip.by_name(name) {
        Ok(f) => Ok(f),
        Err(e) => match e {
            ZipError::FileNotFound => Err(Error::MissingFile(name.to_owned())),
            _ => Err(e.into()),
        },
    }
}

impl Beatmap {
    pub fn read<R: Read + Seek>(reader: R) -> Result<Self, Error> {
        let mut zip = ZipArchive::new(reader)?;

        let info: Info = {
            let mut file = zip_by_name(&mut zip, "info.dat")?;
            serde_json::from_reader(&mut file)?
        };

        let song = {
            let filename = &info.song_filename;
            let mut file = zip_by_name(&mut zip, filename)?;
            let mut buffer = Vec::with_capacity(file.size().try_into()?);
            file.read_to_end(&mut buffer)?;
            buffer
        };

        let cover = {
            let filename = &info.cover_image_filename;
            let mut file = zip_by_name(&mut zip, filename)?;
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
                let mut file = zip_by_name(&mut zip, filename)?;

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
    #[error("missing file `{0}` in zip")]
    MissingFile(String),
    #[error("inconsistent beatmap info and sets")]
    InconsistentBeatmap,
}

#[cfg(test)]
mod tests {
    use super::Beatmap;
    use std::io::{self, Cursor};

    #[test]
    fn read_write() {
        let mut request = ureq::get("https://beatsaver.com/api/download/key/570")
            .set(
                "User-Agent",
                concat!("beatmap-rs/", env!("CARGO_PKG_VERSION")),
            )
            .build();
        let response = request.call();

        if response.error() {
            panic!("http error: {}", response.into_string().unwrap());
        }

        let mut file = Cursor::new(Vec::new());
        io::copy(&mut response.into_reader(), &mut file).unwrap();
        file.set_position(0);

        let old = Beatmap::read(&mut file).unwrap();

        let mut buffer = Cursor::new(Vec::new());
        old.write(&mut buffer).unwrap();
        buffer.set_position(0);

        let new = Beatmap::read(&mut buffer).unwrap();

        assert_eq!(old, new);
    }
}
