use std::{env, fs, io::Error, path::PathBuf};

use image::{GenericImage, GenericImageView, ImageBuffer};

const SKYBOX_TILES_AMOUNT: usize = 6;

pub fn merge_all_files() -> Result<(), Error> {
    let file_paths = get_file_paths()?;
    for skybox in get_skyboxes(file_paths) {
        println!("Processing skybox tiles");
        dbg!(&skybox);
        skybox.merge();
    }

    Ok(())
}

fn get_file_paths() -> Result<Vec<PathBuf>, Error> {
    let path = env::current_dir().expect("Should be able to read current directory");

    println!("Processign dir {}", path.display());

    let paths: Vec<PathBuf> = fs::read_dir(path)?
        .filter_map(Result::ok)
        .map(|f| f.path())
        .filter(|f| f.is_file() && f.extension().unwrap_or_default() == "png")
        .collect();

    print!("Found {} files", paths.len());

    Ok(paths)
}

fn get_skyboxes(paths: Vec<PathBuf>) -> Vec<SkyBoxTiles> {
    match paths.len() {
        s if s > SKYBOX_TILES_AMOUNT => panic!("Directory should have only {} tile files for now as merging of single skybox is supported", SKYBOX_TILES_AMOUNT),
        s if s < SKYBOX_TILES_AMOUNT => panic!("Ensure all skybox tiles are present"),
        _ => {}
    }

    let tiles: Vec<SkyboxTile> = paths
        .into_iter()
        .map(|path| match path.file_name().and_then(|f| f.to_str()) {
            Some(p) if p.ends_with("left.png") => Some(SkyboxTile {
                path,
                position: SkyboxTilePosition::Left,
            }),
            Some(p) if p.ends_with("right.png") => Some(SkyboxTile {
                path,
                position: SkyboxTilePosition::Right,
            }),
            Some(p) if p.ends_with("up.png") => Some(SkyboxTile {
                path,
                position: SkyboxTilePosition::Up,
            }),
            Some(p) if p.ends_with("down.png") => Some(SkyboxTile {
                path,
                position: SkyboxTilePosition::Down,
            }),
            Some(p) if p.ends_with("front.png") => Some(SkyboxTile {
                path,
                position: SkyboxTilePosition::Front,
            }),
            Some(p) if p.ends_with("back.png") => Some(SkyboxTile {
                path,
                position: SkyboxTilePosition::Back,
            }),
            Some(_) | None => None,
        })
        .flatten()
        .collect();

    vec![SkyBoxTiles { tiles }]
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "testable", derive(Clone))]
struct SkyBoxTiles {
    tiles: Vec<SkyboxTile>,
}

#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "testable", derive(Clone))]
struct SkyboxTile {
    path: PathBuf,
    position: SkyboxTilePosition,
}

#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "testable", derive(Clone))]
enum SkyboxTilePosition {
    Left,
    Right,
    Up,
    Down,
    Front,
    Back,
}

impl SkyBoxTiles {
    fn merge(self) {
        if self.tiles.len() != SKYBOX_TILES_AMOUNT {
            eprintln!("Not all tiles are set for skybox. Skipping skybox");
            return;
        }

        let first_file = image::open(&self.tiles[0].path)
            .expect("First tile should be opened to calculate dimensions");
        let (width, height) = first_file.dimensions();

        drop(first_file);

        let mut result_file = ImageBuffer::new(width * 4, height * 3);

        for tile in self.tiles.into_iter() {
            let pic = image::open(tile.path).unwrap();

            match tile.position {
                SkyboxTilePosition::Left => result_file
                    .copy_from(&pic, 0, height)
                    .expect("skybox tile copy success"),
                SkyboxTilePosition::Right => result_file
                    .copy_from(&pic, width * 2, height)
                    .expect("skybox tile copy success"),
                SkyboxTilePosition::Up => result_file
                    .copy_from(&pic, width, 0)
                    .expect("skybox tile copy success"),
                SkyboxTilePosition::Down => result_file
                    .copy_from(&pic, width, height * 2)
                    .expect("skybox tile copy success"),
                SkyboxTilePosition::Front => result_file
                    .copy_from(&pic, width, height)
                    .expect("skybox tile copy success"),
                SkyboxTilePosition::Back => result_file
                    .copy_from(&pic, width * 3, height)
                    .expect("skybox tile copy success"),
            };
        }

        result_file
            .save_with_format("skybox.png", image::ImageFormat::Png)
            .expect("File saved");
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use assertx::*;

    use super::*;

    #[test]
    fn get_skyboxes_single_file() {
        let expected_skybox_tiles = generate_skybox_tiles("skybox_01a");

        let paths = expected_skybox_tiles.tiles.iter().map(|f| f.path.clone()).collect();

        let skyboxes = get_skyboxes(paths);

        assert_contains_exactly!(skyboxes, vec![expected_skybox_tiles]);
    }

    #[test]
    fn get_skyboxes_multiple_files() {
        let prefix_1 = String::from("skybox_01a");
        let prefix_2 = String::from("skybox_02a");
        let expected_skybox_tiles_1 = generate_skybox_tiles(&prefix_1);
        let expected_skybox_tiles_2 = generate_skybox_tiles(&prefix_2);

        let paths: Vec<PathBuf> = expected_skybox_tiles_1.tiles.iter().chain(expected_skybox_tiles_2.tiles.iter()).filter(|f| f.path.starts_with(&prefix_1)).map(|f| f.path.clone()).collect();

        let skyboxes = get_skyboxes(paths);

        assert_contains_exactly!(skyboxes, vec![expected_skybox_tiles_1, expected_skybox_tiles_2]);
    }

    fn generate_skybox_tiles(prefix: &str) -> SkyBoxTiles {
        let left = SkyboxTile { path: PathBuf::from(format!("{}_left.png", prefix)), position: SkyboxTilePosition::Left};
        let rigth = SkyboxTile { path: PathBuf::from(format!("{}_right.png", prefix)), position: SkyboxTilePosition::Right};
        let up = SkyboxTile { path: PathBuf::from(format!("{}_up.png", prefix)), position: SkyboxTilePosition::Up};
        let down = SkyboxTile { path: PathBuf::from(format!("{}_down.png", prefix)), position: SkyboxTilePosition::Down};
        let front = SkyboxTile { path: PathBuf::from(format!("{}_front.png", prefix)), position: SkyboxTilePosition::Front};
        let back = SkyboxTile { path: PathBuf::from(format!("{}_back.png", prefix)), position: SkyboxTilePosition::Back};

        SkyBoxTiles { tiles: vec![left, rigth, up, down, front, back] }
    }
}
