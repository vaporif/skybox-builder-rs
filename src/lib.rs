use std::{fs, io::Error, path::PathBuf};

use image::{GenericImage, GenericImageView, ImageBuffer};

pub fn merge_all_files() -> Result<(), Error> {
    let file_paths = get_file_paths()?;
    for skybox in get_skyboxes(file_paths) {
        skybox.merge();
    }

    Ok(())
}

fn get_file_paths() -> Result<Vec<PathBuf>, Error> {
    let rd = fs::read_dir(".")?;

    let paths: Vec<PathBuf> = rd
        .filter_map(Result::ok)
        .map(|f| f.path())
        .filter(|f| f.is_file())
        .filter(|f| f.extension().unwrap_or_default() == "png")
        .collect();

    for path in paths.iter() {
        println!("{:?}", &path);
    }

    Ok(paths)
}

fn get_skyboxes(paths: Vec<PathBuf>) -> Vec<SkyBoxFiles> {
    if paths.len() > 6 {
        panic!("Single skybox is supported");
    }

    if paths.len() < 6 {
        panic!("Ensure all skybox tiles are present");
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
        .filter_map(|f| f)
        .collect();

    vec![SkyBoxFiles { tiles }]
}

struct SkyBoxFiles {
    tiles: Vec<SkyboxTile>,
}

#[derive(PartialEq)]
struct SkyboxTile {
    path: PathBuf,
    position: SkyboxTilePosition,
}

#[derive(PartialEq, Debug)]
enum SkyboxTilePosition {
    Left,
    Right,
    Up,
    Down,
    Front,
    Back,
}

impl SkyBoxFiles {
    fn merge(self) {
        if self.tiles.len() != 6 {
            eprintln!("Not all tiles are set for skybox");
        }

        let first_file = image::open(&self.tiles[0].path).expect("file opened for merge");
        let (width, height) = first_file.dimensions();

        let mut result_file = ImageBuffer::new(width * 4, height * 3);

        for tile in self.tiles.into_iter() {
            let pic = image::open(tile.path).unwrap();

            match tile.position {
                SkyboxTilePosition::Left => result_file
                    .copy_from(&pic, 0, height)
                    .expect("copy success"),
                SkyboxTilePosition::Right => result_file
                    .copy_from(&pic, width * 2, height)
                    .expect("copy success"),
                SkyboxTilePosition::Up => {
                    result_file.copy_from(&pic, width, 0).expect("copy success")
                }
                SkyboxTilePosition::Down => result_file
                    .copy_from(&pic, width, height * 2)
                    .expect("copy success"),
                SkyboxTilePosition::Front => result_file
                    .copy_from(&pic, width, height)
                    .expect("copy success"),
                SkyboxTilePosition::Back => result_file
                    .copy_from(&pic, width * 3, height)
                    .expect("copy success"),
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

    use super::*;

    #[test]
    fn get_skyboxes_group_single_file() {
        let l_path = PathBuf::from("skybox_01a_left.png");
        let r_path = PathBuf::from("skybox_01a_right.png");
        let u_path = PathBuf::from("skybox_01a_up.png");
        let d_path = PathBuf::from("skybox_01a_down.png");
        let f_path = PathBuf::from("skybox_01a_front.png");
        let b_path = PathBuf::from("skybox_01a_back.png");

        //assert!(l_path.file_name().unwrap().to_owned().into_string().unwrap().ends_with("left.png"));

        let paths = vec![
            l_path.clone(),
            r_path.clone(),
            u_path.clone(),
            d_path.clone(),
            f_path.clone(),
            b_path.clone(),
        ];

        // let expected = SkyBoxFiles {
        //     left: Some(l_path),
        //     right: Some(r_path),
        //     up: Some(u_path),
        //     down: Some(d_path),
        //     front: Some(f_path),
        //     back: Some(b_path),
        // };

        // let skyboxes = get_skyboxes(paths);

        // assert!(skyboxes.len() == 1);

        // assert_eq!(*skyboxes.first().unwrap(), expected);
    }
}
