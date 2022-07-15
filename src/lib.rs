use std::{
    fs,
    io::Error,
    path::{Path, PathBuf},
};

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

    let paths = fs::read_dir(dir_path)?
        .into_iter()
        .map(|f| f.unwrap().path())
        .collect();

    Ok(paths)
}

fn get_skyboxes(paths: Vec<PathBuf>) -> Vec<SkyBoxFiles> {
    unimplemented!()
}

#[derive(Debug, PartialEq)]
struct SkyBoxFiles {
    left: PathBuf,
    right: PathBuf,
    up: PathBuf,
    down: PathBuf,
    front: PathBuf,
    back: PathBuf,
}

impl SkyBoxFiles {
    fn merge(self) {
        let left = image::open(self.left).unwrap();
        let right = image::open(self.right).unwrap();
        let up = image::open(self.up).unwrap();
        let down = image::open(self.down).unwrap();
        let front = image::open(self.front).unwrap();
        let back = image::open(self.back).unwrap();
    }
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

        let paths = vec![
            l_path.clone(),
            r_path.clone(),
            u_path.clone(),
            d_path.clone(),
            f_path.clone(),
            b_path.clone(),
        ];

        let expected = SkyBoxFiles {
            left: l_path,
            right: r_path,
            up: u_path,
            down: d_path,
            front: f_path,
            back: b_path,
        };

        let skyboxes = get_skyboxes(paths);

        assert!(skyboxes.len() == 1);

        assert_eq!(*skyboxes.first().unwrap(), expected);
    }
}
