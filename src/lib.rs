use std::{fs, io::Error, path::{Path, PathBuf}};

use image::DynamicImage;

pub fn merge_all_files(path: &str) -> Result<(), Error> {
    let file_paths = get_file_paths(path)?;
    let skyboxes = get_skyboxes(file_paths);

    for skybox in skyboxes.iter().map(|f| SkyBoxImageGroup::new(f)) {
        skybox.merge();
    }

    Ok(())
}

fn get_file_paths(mut dir_path: &str) -> Result<Vec<PathBuf>, Error> {
    if dir_path.is_empty() {
        dir_path = &"./";
    }

    let paths = fs::read_dir(dir_path)?
        .into_iter()
        .map(|f| f.unwrap().path())
        .collect();

    Ok(paths)
}

fn get_skyboxes(paths: Vec<PathBuf>) -> Vec<SkyBoxFilePath<'static>> {
    unimplemented!()
}

struct SkyBoxFilePath<'a> {
    left: &'a Path,
    right: &'a Path,
    up: &'a Path,
    down: &'a Path,
    front: &'a Path,
    back: &'a Path,
}

struct SkyBoxImageGroup {
    left: DynamicImage,
    right: DynamicImage,
    up: DynamicImage,
    down: DynamicImage,
    front: DynamicImage,
    back: DynamicImage,
}

impl SkyBoxImageGroup {
    fn new(file_group: &SkyBoxFilePath) -> SkyBoxImageGroup {
        Self {
            left: image::open(file_group.left).unwrap(),
            right: image::open(file_group.right).unwrap(),
            up: image::open(file_group.up).unwrap(),
            down: image::open(file_group.down).unwrap(),
            front: image::open(file_group.front).unwrap(),
            back: image::open(file_group.back).unwrap(),
        }
    }

    fn merge(self) {}
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn get_skyboxes_group_single_file() {
        let l_path = PathBuf::from("skybox_01a_left");
        let r_path = PathBuf::from("skybox_01a_right");
        let u_path = PathBuf::from("skybox_01a_up");
        let d_path = PathBuf::from("skybox_01a_down");
        let f_path = PathBuf::from("skybox_01a_front");
        let b_path = PathBuf::from("skybox_01a_back");
        let paths = vec![PathBuf::new()];
    }
}
