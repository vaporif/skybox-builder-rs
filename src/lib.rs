use std::{
    collections::{hash_map::Entry, HashMap},
    env, fs,
    io::Error,
    path::PathBuf,
};

use image::{GenericImage, GenericImageView, ImageBuffer};

const SKYBOX_TILES_AMOUNT: usize = 6;

const LEFT_PNG_FILE_NAME: &str = "left.png";
const RIGHT_PNG_FILE_NAME: &str = "right.png";
const UP_PNG_FILE_NAME: &str = "up.png";
const DOWN_PNG_FILE_NAME: &str = "down.png";
const FRONT_PNG_FILE_NAME: &str = "front.png";
const BACK_PNG_FILE_NAME: &str = "back.png";

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
    if paths.len() < SKYBOX_TILES_AMOUNT {
        panic!("Ensure all skybox tiles are present");
    }

    let tiles_ungrouped: Vec<SkyboxTile> = paths
        .into_iter()
        .filter_map(|path| {
            match path
                .file_name()
                .and_then(|f| f.to_str())
                .map(|f| f.to_owned())
            {
                Some(p) if p.ends_with(LEFT_PNG_FILE_NAME) => Some(SkyboxTile::new(
                    path,
                    &p,
                    SkyboxTilePosition::Left,
                    LEFT_PNG_FILE_NAME,
                )),
                Some(p) if p.ends_with(RIGHT_PNG_FILE_NAME) => Some(SkyboxTile::new(
                    path,
                    &p,
                    SkyboxTilePosition::Right,
                    RIGHT_PNG_FILE_NAME,
                )),
                Some(p) if p.ends_with(UP_PNG_FILE_NAME) => Some(SkyboxTile::new(
                    path,
                    &p,
                    SkyboxTilePosition::Up,
                    UP_PNG_FILE_NAME,
                )),
                Some(p) if p.ends_with(DOWN_PNG_FILE_NAME) => Some(SkyboxTile::new(
                    path,
                    &p,
                    SkyboxTilePosition::Down,
                    DOWN_PNG_FILE_NAME,
                )),
                Some(p) if p.ends_with(FRONT_PNG_FILE_NAME) => Some(SkyboxTile::new(
                    path,
                    &p,
                    SkyboxTilePosition::Front,
                    FRONT_PNG_FILE_NAME,
                )),
                Some(p) if p.ends_with(BACK_PNG_FILE_NAME) => Some(SkyboxTile::new(
                    path,
                    &p,
                    SkyboxTilePosition::Back,
                    BACK_PNG_FILE_NAME,
                )),
                Some(_) | None => None,
            }
        })
        .collect();

    let mut tiles = HashMap::<String, Vec<SkyboxTile>>::new();

    for ele in tiles_ungrouped {
        match tiles.entry(ele.prefix.clone()) {
            Entry::Occupied(mut o) => o.get_mut().push(ele),
            Entry::Vacant(v) => {
                v.insert(vec![ele]);
                ()
            }
        }
    }

    vec![SkyBoxTiles { tiles }]
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "testable", derive(Clone))]
struct SkyBoxTiles {
    tiles: HashMap<String, Vec<SkyboxTile>>,
}

#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "testable", derive(Clone))]
struct SkyboxTile {
    path: PathBuf,
    prefix: String,
    position: SkyboxTilePosition,
}

impl SkyboxTile {
    fn new(
        path: PathBuf,
        file_name: &str,
        position: SkyboxTilePosition,
        file_suffix_to_omit: &str,
    ) -> Self {
        SkyboxTile {
            path,
            prefix: file_name.trim_end_matches(file_suffix_to_omit).to_owned(),
            position,
        }
    }
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
        for (prefix, tiles) in self.tiles {
            if tiles.len() != SKYBOX_TILES_AMOUNT {
                eprintln!(
                    "Not all tiles are set for skybox {}. Skipping skybox",
                    prefix
                );
                continue;
            }

            let first_file = image::open(&tiles[0].path)
                .expect("First tile should be opened to calculate dimensions");
            let (width, height) = first_file.dimensions();

            drop(first_file);

            let mut result_file = ImageBuffer::new(width * 4, height * 3);

            for tile in tiles.into_iter() {
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
                .save_with_format(format!("{}_skybox.png", prefix), image::ImageFormat::Png)
                .expect("File saved");
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use assertx::*;
//     use std::path::PathBuf;

//     use super::*;

//     #[test]
//     fn get_skyboxes_single_file() {
//         let expected_skybox_tiles = generate_skybox_tiles("skybox_01a");

//         let paths = expected_skybox_tiles
//             .tiles
//             .iter()
//             .map(|f| f.path.clone())
//             .collect();

//         let skyboxes = get_skyboxes(paths);

//         assert_contains_exactly!(skyboxes, vec![expected_skybox_tiles]);
//     }

//     #[test]
//     #[ignore]
//     fn get_skyboxes_multiple_files() {
//         let prefix_1 = String::from("skybox_01a");
//         let prefix_2 = String::from("skybox_02a");
//         let expected_skybox_tiles_1 = generate_skybox_tiles(&prefix_1);
//         let expected_skybox_tiles_2 = generate_skybox_tiles(&prefix_2);

//         let paths: Vec<PathBuf> = expected_skybox_tiles_1
//             .tiles
//             .iter()
//             .chain(expected_skybox_tiles_2.tiles.iter())
//             .filter(|f| f.path.starts_with(&prefix_1))
//             .map(|f| f.path.clone())
//             .collect();

//         let skyboxes = get_skyboxes(paths);

//         assert_contains_exactly!(
//             skyboxes,
//             vec![expected_skybox_tiles_1, expected_skybox_tiles_2]
//         );
//     }

//     fn generate_skybox_tiles(prefix: &str) -> SkyBoxTiles {
//         let left = SkyboxTile {
//             path: PathBuf::from(format!("{}_left.png", prefix)),
//             prefix: prefix.to_owned(),
//             position: SkyboxTilePosition::Left,
//         };
//         let rigth = SkyboxTile {
//             path: PathBuf::from(format!("{}_right.png", prefix)),
//             prefix: prefix.to_owned(),
//             position: SkyboxTilePosition::Right,
//         };
//         let up = SkyboxTile {
//             path: PathBuf::from(format!("{}_up.png", prefix)),
//             prefix: prefix.to_owned(),
//             position: SkyboxTilePosition::Up,
//         };
//         let down = SkyboxTile {
//             path: PathBuf::from(format!("{}_down.png", prefix)),
//             prefix: prefix.to_owned(),
//             position: SkyboxTilePosition::Down,
//         };
//         let front = SkyboxTile {
//             path: PathBuf::from(format!("{}_front.png", prefix)),
//             prefix: prefix.to_owned(),
//             position: SkyboxTilePosition::Front,
//         };
//         let back = SkyboxTile {
//             path: PathBuf::from(format!("{}_back.png", prefix)),
//             prefix: prefix.to_owned(),
//             position: SkyboxTilePosition::Back,
//         };

//         SkyBoxTiles {
//             tiles: HashMap::from((prefix.to_owned(), vec![left, rigth, up, down, front, back])),
//         }
//     }
// }
