use std::{
    collections::{hash_map::Entry, HashMap},
    env, fs,
    io::Error,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use rayon::prelude::*;

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
    let skyboxes = get_skyboxes(file_paths);
    println!("Processing skybox tiles");
    dbg!(&skyboxes);
    merge(skyboxes);

    Ok(())
}

fn get_file_paths() -> Result<Vec<PathBuf>, Error> {
    let path = env::current_dir().expect("Should be able to read current directory");

    println!("Processing dir {}", path.display());

    let paths: Vec<PathBuf> = fs::read_dir(path)?
        .filter_map(Result::ok)
        .map(|f| f.path())
        .filter(|f| f.is_file() && f.extension().unwrap_or_default() == "png")
        .collect();

    print!("Found {} files", paths.len());

    Ok(paths)
}

fn get_skyboxes(paths: Vec<PathBuf>) -> HashMap<String, Vec<SkyboxTile>> {
    if paths.len() < SKYBOX_TILES_AMOUNT {
        eprintln!("Ensure all skybox tiles are present");
        panic!();
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
                _ => None,
            }
        })
        .collect();

    let mut tiles = HashMap::<String, Vec<SkyboxTile>>::new();

    for ele in tiles_ungrouped {
        match tiles.entry(ele.prefix.clone()) {
            Entry::Occupied(mut o) => o.get_mut().push(ele),
            Entry::Vacant(v) => {
                v.insert(vec![ele]);
            }
        }
    }

    tiles
}

fn merge(mut tiles: HashMap<String, Vec<SkyboxTile>>) {
    tiles.par_drain().for_each(|r| {
        let (prefix, mut tiles) = r;
        if tiles.len() != SKYBOX_TILES_AMOUNT {
            eprintln!(
                "Not all tiles are set for skybox {}. Skipping skybox",
                prefix
            );
            return;
        }

        let first_file = image::open(&tiles[0].path)
            .expect("First tile should be opened to calculate dimensions");

        let (width, height) = first_file.dimensions();

        drop(first_file);

        let result_file = ImageBuffer::new(width * 4, height * 3);
        let reserve_file_mut = Arc::new(Mutex::new(result_file));

        tiles.par_drain(..).for_each(|tile| {
            let pic = image::open(tile.path).unwrap();
            let (pic_width, pic_height) = pic.dimensions();

            if pic_height != height || pic_width != width {
                eprintln!(
                    "Not all tiles on skybox {} have same dimensions. Skipping skybox",
                    &prefix
                );

                return;
            }

            match tile.position {
                SkyboxTilePosition::Left => reserve_file_mut
                    .lock()
                    .unwrap()
                    .copy_from(&pic, 0, height)
                    .expect("skybox tile copy success"),
                SkyboxTilePosition::Right => reserve_file_mut
                    .lock()
                    .unwrap()
                    .copy_from(&pic, width * 2, height)
                    .expect("skybox tile copy success"),
                SkyboxTilePosition::Up => reserve_file_mut
                    .lock()
                    .unwrap()
                    .copy_from(&pic, width, 0)
                    .expect("skybox tile copy success"),
                SkyboxTilePosition::Down => reserve_file_mut
                    .lock()
                    .unwrap()
                    .copy_from(&pic, width, height * 2)
                    .expect("skybox tile copy success"),
                SkyboxTilePosition::Front => reserve_file_mut
                    .lock()
                    .unwrap()
                    .copy_from(&pic, width, height)
                    .expect("skybox tile copy success"),
                SkyboxTilePosition::Back => reserve_file_mut
                    .lock()
                    .unwrap()
                    .copy_from(&pic, width * 3, height)
                    .expect("skybox tile copy success"),
            };
        });

        reserve_file_mut
            .lock()
            .unwrap()
            .save_with_format(format!("{}skybox.png", &prefix), image::ImageFormat::Png)
            .expect("File saved");
    });
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn get_skyboxes_single_file() {
        let expected_skybox_tiles = generate_skybox_tiles("skybox_01a");

        let paths = expected_skybox_tiles
            .iter()
            .map(|f| f.path.clone())
            .collect();

        let skyboxes = get_skyboxes(paths);

        similar_asserts::assert_eq!(
            HashMap::from([("skybox_01a".to_owned(), expected_skybox_tiles)]),
            skyboxes
        );
    }

    #[test]
    fn get_skyboxes_multiple_files() {
        let prefix_1 = String::from("skybox_01a");
        let prefix_2 = String::from("skybox_02a");
        let expected_skybox_tiles_1 = generate_skybox_tiles(&prefix_1);
        let expected_skybox_tiles_2 = generate_skybox_tiles(&prefix_2);

        let paths: Vec<PathBuf> = expected_skybox_tiles_1
            .iter()
            .chain(expected_skybox_tiles_2.iter())
            .map(|f| f.path.clone())
            .collect();

        let skyboxes = get_skyboxes(paths);

        similar_asserts::assert_eq!(
            HashMap::from([
                (prefix_1, expected_skybox_tiles_1),
                (prefix_2, expected_skybox_tiles_2)
            ]),
            skyboxes
        );
    }

    fn generate_skybox_tiles(prefix: &str) -> Vec<SkyboxTile> {
        let left = SkyboxTile {
            path: PathBuf::from(format!("{}left.png", prefix)),
            prefix: prefix.to_owned(),
            position: SkyboxTilePosition::Left,
        };
        let rigth = SkyboxTile {
            path: PathBuf::from(format!("{}right.png", prefix)),
            prefix: prefix.to_owned(),
            position: SkyboxTilePosition::Right,
        };
        let up = SkyboxTile {
            path: PathBuf::from(format!("{}up.png", prefix)),
            prefix: prefix.to_owned(),
            position: SkyboxTilePosition::Up,
        };
        let down = SkyboxTile {
            path: PathBuf::from(format!("{}down.png", prefix)),
            prefix: prefix.to_owned(),
            position: SkyboxTilePosition::Down,
        };
        let front = SkyboxTile {
            path: PathBuf::from(format!("{}front.png", prefix)),
            prefix: prefix.to_owned(),
            position: SkyboxTilePosition::Front,
        };
        let back = SkyboxTile {
            path: PathBuf::from(format!("{}back.png", prefix)),
            prefix: prefix.to_owned(),
            position: SkyboxTilePosition::Back,
        };

        vec![left, rigth, up, down, front, back]
    }
}
