use std::{path::{PathBuf, Path}, fs};

const LEFT_PNG_FILE_NAME: &str = "left.png";
const RIGHT_PNG_FILE_NAME: &str = "right.png";
const UP_PNG_FILE_NAME: &str = "up.png";
const DOWN_PNG_FILE_NAME: &str = "down.png";
const FRONT_PNG_FILE_NAME: &str = "front.png";
const BACK_PNG_FILE_NAME: &str = "back.png";

#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "testable", derive(Clone))]
pub struct SkyboxTile {
    path: PathBuf,
    prefix: String,
    position: SkyboxTilePosition,
}

impl SkyboxTile {
    pub fn from_file(
        path: PathBuf
    ) -> Option<Self> {
        let file_name = path.file_name().and_then(|f| f.to_str())?;
        let (position, prefix) = Self::get_position_and_prefix(file_name)?;

        Some(SkyboxTile {
            path,
            prefix,
            position
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

    fn get_position_and_prefix(file_name: &str) -> Option<(SkyboxTilePosition, String)> {
        match file_name {
            s if s.ends_with(LEFT_PNG_FILE_NAME) => Some((SkyboxTilePosition::Left, file_name.trim_end_matches(LEFT_PNG_FILE_NAME).to_owned())),
            s if s.ends_with(RIGHT_PNG_FILE_NAME) => Some((SkyboxTilePosition::Right, file_name.trim_end_matches(RIGHT_PNG_FILE_NAME).to_owned())),
            s if s.ends_with(UP_PNG_FILE_NAME) => Some((SkyboxTilePosition::Up, file_name.trim_end_matches(UP_PNG_FILE_NAME).to_owned())),
            s if s.ends_with(DOWN_PNG_FILE_NAME) => Some((SkyboxTilePosition::Down, file_name.trim_end_matches(DOWN_PNG_FILE_NAME).to_owned())),
            s if s.ends_with(FRONT_PNG_FILE_NAME) => Some((SkyboxTilePosition::Front, file_name.trim_end_matches(FRONT_PNG_FILE_NAME).to_owned())),
            s if s.ends_with(BACK_PNG_FILE_NAME) => Some((SkyboxTilePosition::Back, file_name.trim_end_matches(BACK_PNG_FILE_NAME).to_owned())),
            _ => None
        }
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
