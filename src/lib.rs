use std::{
    fs,
    io::Error,
    path::PathBuf,
};

use image::{GenericImageView, DynamicImage, ImageBuffer, RgbImage, GenericImage, Rgba};

pub fn merge_all_files(path: &str) -> Result<(), Error> {
    let file_paths = get_file_paths(path)?;
    for skybox in get_skyboxes(file_paths) {
        skybox.merge();
    }

    Ok(())
}

fn get_file_paths(mut dir_path: &str) -> Result<Vec<PathBuf>, Error> {
    if dir_path.is_empty() {
        dir_path = &"./";
    }

    let rd = fs::read_dir(dir_path)?;

    let paths = rd
        .filter_map(Result::ok)
        .map(|f| f.path())
        .filter(|f| f.is_file())
        .filter(|f| f.extension().unwrap_or_default() == "png")
        .collect();

    Ok(paths)
}

fn get_skyboxes(paths: Vec<PathBuf>) -> Vec<SkyBoxFiles> {
    if paths.len() > 6 {
        panic!("Single skybox is supported");
    }

    if paths.len() < 6 {
        panic!("Ensure all skybox tiles are present");
    }

    let tiles: Vec<SkyboxTile> = paths.into_iter().map(|path| {
        match path.file_name().and_then(|f| f.to_str()) {
            Some(p) if p.ends_with("left.png") => Some(SkyboxTile {path, position: SkyboxTilePosition::Left}),
            Some(p) if p.ends_with("right.png") => Some(SkyboxTile {path, position: SkyboxTilePosition::Right}),
            Some(p) if p.ends_with("up.png") => Some(SkyboxTile {path, position: SkyboxTilePosition::Up}),
            Some(p) if p.ends_with("down.png") => Some(SkyboxTile {path, position: SkyboxTilePosition::Down}),
            Some(p) if p.ends_with("front.png") => Some(SkyboxTile {path, position: SkyboxTilePosition::Front}),
            Some(p) if p.ends_with("back.png") => Some(SkyboxTile {path, position: SkyboxTilePosition::Back}),
            Some(_) | None => None
        }
    }).filter_map(|f| f).collect();

    vec![SkyBoxFiles{tiles}]
}

struct SkyBoxFiles {
    tiles:Vec<SkyboxTile>
}

#[derive(PartialEq)]
struct SkyboxTile {
    path: PathBuf,
    position: SkyboxTilePosition
}

impl SkyboxTile {
    fn read_tile(self) -> (DynamicImage, u32, u32) {
        let pic = image::open(self.path).unwrap();
        let (width, height) = pic.dimensions();
        (pic, width, height)
    }
}

#[derive(PartialEq, Debug)]
enum SkyboxTilePosition {
    Left,
    Right,
    Up,
    Down,
    Front,
    Back
}

impl SkyBoxFiles {
    fn merge(self) {
        if self.tiles.len() != 6 {
            eprintln!("Not all tiles are set for skybox");
        }

        let mut result_file: Option<ImageBuffer<Rgba<u8>, Vec<u8>>> = Option::None;
        let mut dimensions:Option<(u32, u32)> = Option::None;

        for tile in self.tiles.into_iter() {
            let pic = image::open(tile.path).unwrap();
            if result_file.as_ref() == Option::None {
                let (width, height) = pic.dimensions();
                dimensions = Some((width, height));
                result_file = Some(ImageBuffer::new(width*4, height*3));
            }

            let (width, height) = dimensions.unwrap();

            let result_file = result_file.unwrap();

            match tile.position {
                SkyboxTilePosition::Left => result_file.copy_from(&pic, 0, height),
                SkyboxTilePosition::Right => result_file.copy_from(&pic, width*2, height),
                SkyboxTilePosition::Up => result_file.copy_from(&pic, width, 0),
                SkyboxTilePosition::Down => result_file.copy_from(&pic, width, height*2),
                SkyboxTilePosition::Front => result_file.copy_from(&pic, width, height),
                SkyboxTilePosition::Back => result_file.copy_from(&pic, width*3, height)
            };
        }

        result_file.unwrap().save_with_format("skybox.png", image::ImageFormat::Png).expect("File saved");
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
