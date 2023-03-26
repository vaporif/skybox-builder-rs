use std::{
    fs,
    path::{Path, PathBuf}, ffi::OsStr,
};

pub static TILES_FOR_MERGE: &[&str] = &[
    LEFT_FILENAME_SUFFIX,
    RIGHT_FILENAME_SUFFIX,
    UP_FILENAME_SUFFIX,
    DOWN_FILENAME_SUFFIX,
    FRONT_FILENAME_SUFFIX,
    BACK_FILENAME_SUFFIX,
];

const LEFT_FILENAME_SUFFIX: &str = "left";
const RIGHT_FILENAME_SUFFIX: &str = "right";
const UP_FILENAME_SUFFIX: &str = "up";
const DOWN_FILENAME_SUFFIX: &str = "down";
const FRONT_FILENAME_SUFFIX: &str = "front";
const BACK_FILENAME_SUFFIX: &str = "back";

#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "testable", derive(Clone))]
pub struct SkyboxTile {
    path: PathBuf,
    prefix: String,
    position: SkyboxTilePosition,
}

impl SkyboxTile {
    pub fn from_file(path: PathBuf) -> Option<Self> {
        let file_name = path.file_name().and_then(|f| f.to_str())?;
        let (position, prefix) = Self::get_position_and_prefix(file_name)?;

        let prefix = prefix.to_owned();
        Some(SkyboxTile {
            path,
            prefix,
            position,
        })
    }

    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn position(&self) -> &SkyboxTilePosition {
        &self.position
    }

    pub fn delete(self) {
        if let Err(error) = fs::remove_file(&self.path) {
            eprintln!(
                "Error removing file {}: {error}",
                self.path.file_name().unwrap().to_str().unwrap()
            )
        }
    }

    fn get_position_and_prefix(file_name: &str) -> Option<(SkyboxTilePosition, &str)> {
        let extension = Self::get_extension_from_filename(file_name)?;
        match file_name {
            s if s.ends_with(&format!("{LEFT_FILENAME_SUFFIX}.{extension}")) => Some((
                SkyboxTilePosition::Left,
                file_name.trim_end_matches(LEFT_FILENAME_SUFFIX),
            )),
            s if s.ends_with(&format!("{RIGHT_FILENAME_SUFFIX}.{extension}")) => Some((
                SkyboxTilePosition::Right,
                file_name.trim_end_matches(RIGHT_FILENAME_SUFFIX),
            )),
            s if s.ends_with(&format!("{UP_FILENAME_SUFFIX}.{extension}")) => Some((
                SkyboxTilePosition::Up,
                file_name.trim_end_matches(UP_FILENAME_SUFFIX),
            )),
            s if s.ends_with(&format!("{DOWN_FILENAME_SUFFIX}.{extension}")) => Some((
                SkyboxTilePosition::Down,
                file_name.trim_end_matches(DOWN_FILENAME_SUFFIX),
            )),
            s if s.ends_with(&format!("{FRONT_FILENAME_SUFFIX}.{extension}")) => Some((
                SkyboxTilePosition::Front,
                file_name.trim_end_matches(FRONT_FILENAME_SUFFIX),
            )),
            s if s.ends_with(&format!("{BACK_FILENAME_SUFFIX}.{extension}")) => Some((
                SkyboxTilePosition::Back,
                file_name.trim_end_matches(BACK_FILENAME_SUFFIX),
            )),
            _ => None,
        }
    }

    fn get_extension_from_filename(filename: &str) -> Option<&str> {
        Path::new(filename)
            .extension()
            .and_then(OsStr::to_str)
    }

    pub fn result_file_name(prefix: &str) -> String {
        format!("{}skybox.png", &prefix)
    }
}

#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "testable", derive(Clone))]
pub enum SkyboxTilePosition {
    Left,
    Right,
    Up,
    Down,
    Front,
    Back,
}
