use std::{fs, io::Error, path::{PathBuf, Path}};

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

    let paths = fs::read_dir(dir_path)?;
    
    Ok(paths.into_iter().map(|f| f.unwrap().path()).collect())
}


fn get_skyboxes(paths: Vec<PathBuf>) -> Vec<SkyBoxFilePath<'static>> {
    unimplemented!()
}

struct SkyBoxFilePath<'a> {
    left_side: &'a str,
    center: &'a str,
    right_side: &'a str,
    top_side: &'a str,
    bottom_side: &'a str,
}

struct SkyBoxImageGroup {
    left_side: DynamicImage,
    center: DynamicImage,
    right_side: DynamicImage,
    top_side: DynamicImage,
    bottom_side: DynamicImage,
}

impl SkyBoxImageGroup {
    fn new(file_group: &SkyBoxFilePath) -> SkyBoxImageGroup {
        Self {
            left_side: image::open(file_group.left_side).unwrap(),
            center: image::open(file_group.center).unwrap(),
            right_side: image::open(file_group.right_side).unwrap(),
            top_side: image::open(file_group.top_side).unwrap(),
            bottom_side: image::open(file_group.bottom_side).unwrap(),
        }
    }

    fn merge(self) {

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_skyboxes_group() {
       
    }
}
