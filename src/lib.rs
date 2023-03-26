use std::{
    collections::{hash_map::Entry, HashMap},
    env, fs,
    io::Error,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use rayon::prelude::*;

use image::{GenericImage, GenericImageView, ImageBuffer};

mod skybox_tile;
use skybox_tile::{SkyboxTile, SkyboxTilePosition, TILES_FOR_MERGE};

type TilesGroup = HashMap<String, Vec<SkyboxTile>>;

pub fn process_files(delete_input_files: bool) -> Result<(), Error> {
    let file_paths = get_file_paths()?;
    let skyboxes = get_skyboxes(file_paths);
    println!("Generating skyboxes");
    merge_all_files(skyboxes, delete_input_files);

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

    println!("Found {} png files", paths.len());

    Ok(paths)
}

fn get_skyboxes(paths: Vec<PathBuf>) -> HashMap<String, Vec<SkyboxTile>> {
    let tiles_ungrouped: Vec<SkyboxTile> = paths
        .into_iter()
        .filter_map(SkyboxTile::from_file)
        .collect();

    let mut tiles_grouped = TilesGroup::new();

    for ele in tiles_ungrouped {
        match tiles_grouped.entry(ele.prefix().to_string()) {
            Entry::Occupied(mut o) => o.get_mut().push(ele),
            Entry::Vacant(v) => {
                v.insert(vec![ele]);
            }
        }
    }

    let tiles_grouped: TilesGroup = tiles_grouped
        .into_iter()
        .filter(|(_, tiles)| {
            if tiles.len() < TILES_FOR_MERGE.len() {
                let present_tiles: Vec<&str> = tiles
                    .iter()
                    .filter_map(|tile| tile.path().file_name().and_then(|f| f.to_str()))
                    .collect();

                let missing_tiles: Vec<_> = TILES_FOR_MERGE
                    .iter()
                    .filter(|required_tile| !present_tiles.contains(required_tile))
                    .collect();

                eprintln!("Missing tiles: {:?}", missing_tiles);

                return false;
            }

            true
        })
        .collect();

    let skybox_names_cs = tiles_grouped
        .keys()
        .map(|f| SkyboxTile::result_file_name(f))
        .collect::<Vec<String>>()
        .join(",");
    println!("Files could generate skyboxes: {skybox_names_cs}");

    tiles_grouped
}

fn merge_all_files(mut tiles: HashMap<String, Vec<SkyboxTile>>, delete_input_files: bool) {
    tiles.par_drain().for_each(|r| {
        let (prefix, mut tiles) = r;

        let first_file = image::open(&tiles[0].path())
            .expect("failed to open first image to calculate dimensions");

        let (width, height) = first_file.dimensions();

        drop(first_file);

        let result_file = ImageBuffer::new(width * 4, height * 3);
        let reserve_file_mut = Arc::new(Mutex::new(result_file));

        tiles.par_iter().for_each(|tile| {
            // TODO: process failure
            let pic = image::open(&tile.path()).unwrap();
            let (pic_width, pic_height) = pic.dimensions();

            if pic_height != height || pic_width != width {
                eprintln!(
                    "Not all tiles on skybox {} have same dimensions. Skipping skybox",
                    &prefix
                );

                return;
            }

            match tile.position() {
                SkyboxTilePosition::Left => reserve_file_mut
                    .lock()
                    .expect("result file lock has failed")
                    .copy_from(&pic, 0, height)
                    .expect("failed to copy tile to result image"),
                SkyboxTilePosition::Right => reserve_file_mut
                    .lock()
                    .expect("result file lock has failed")
                    .copy_from(&pic, width * 2, height)
                    .expect("failed to copy tile to result image"),
                SkyboxTilePosition::Up => reserve_file_mut
                    .lock()
                    .expect("result file lock has failed")
                    .copy_from(&pic, width, 0)
                    .expect("failed to copy tile to result image"),
                SkyboxTilePosition::Down => reserve_file_mut
                    .lock()
                    .expect("result file lock has failed")
                    .copy_from(&pic, width, height * 2)
                    .expect("failed to copy tile to result image"),
                SkyboxTilePosition::Front => reserve_file_mut
                    .lock()
                    .expect("result file lock has failed")
                    .copy_from(&pic, width, height)
                    .expect("failed to copy tile to result image"),
                SkyboxTilePosition::Back => reserve_file_mut
                    .lock()
                    .expect("result file lock has failed")
                    .copy_from(&pic, width * 3, height)
                    .expect("failed to copy tile to result image"),
            };
        });

        reserve_file_mut
            .lock()
            // TODO: process failure
            .unwrap()
            .save_with_format(
                SkyboxTile::result_file_name(&prefix),
                image::ImageFormat::Png,
            )
            .expect("could not save result fyle");

        if delete_input_files {
            tiles.drain(..).for_each(|tile| tile.delete());
        }
    })
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
            .map(|f| f.path().to_owned())
            .collect();

        let skyboxes = get_skyboxes(paths);

        similar_asserts::assert_eq!(
            HashMap::from([("skybox_01a".to_owned(), expected_skybox_tiles)]),
            skyboxes
        );
    }

    #[test]
    fn get_skyboxes_multiple_files() {
        let expected_skybox_tiles_1 = generate_skybox_tiles("skybox_01a");
        let expected_skybox_tiles_2 = generate_skybox_tiles("skybox_02a");

        let paths: Vec<PathBuf> = expected_skybox_tiles_1
            .iter()
            .chain(expected_skybox_tiles_2.iter())
            .map(|f| f.path().to_owned())
            .collect();

        let skyboxes = get_skyboxes(paths);

        similar_asserts::assert_eq!(
            HashMap::from([
                ("skybox_01a".to_owned(), expected_skybox_tiles_1),
                ("skybox_02a".to_owned(), expected_skybox_tiles_2)
            ]),
            skyboxes
        );
    }

    fn generate_skybox_tiles(prefix: &str) -> Vec<SkyboxTile> {
        vec!["left", "right", "up", "down", "front", "back"]
            .into_iter()
            .filter_map(|f| SkyboxTile::from_file(PathBuf::from(format!("{prefix}{f}.png"))))
            .collect()
    }
}
