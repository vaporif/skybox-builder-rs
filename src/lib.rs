use std::{
    fs,
    io::Error,
    path::{self, PathBuf},
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

    let mut skybox = SkyBoxFiles::default();

    for path in paths {
        match path.file_name().and_then(|f| f.to_str()) {
            Some(p) if p.ends_with("left.png") => skybox.left = Some(path),
            Some(p) if p.ends_with("right.png") => skybox.right = Some(path),
            Some(p) if p.ends_with("up.png") => skybox.up = Some(path),
            Some(p) if p.ends_with("down.png") => skybox.down = Some(path),
            Some(p) if p.ends_with("front.png") => skybox.front = Some(path),
            Some(p) if p.ends_with("back.png") => skybox.back = Some(path),
            Some(_) => println!("file {:?} has incorrect naming", path.file_name()),
            None => continue
        }
    }

    vec![skybox]
}

#[derive(Debug, PartialEq)]
struct SkyBoxFiles {
    left: Option<PathBuf>,
    right: Option<PathBuf>,
    up: Option<PathBuf>,
    down: Option<PathBuf>,
    front: Option<PathBuf>,
    back: Option<PathBuf>,
}

impl SkyBoxFiles {
    fn merge(self) {
        if let None = self
            .left
            .as_ref()
            .and(self.right.as_ref())
            .and(self.up.as_ref())
            .and(self.down.as_ref())
            .and(self.front.as_ref())
            .and(self.back.as_ref())
        {
            println!("Not all files are present for merging");
        }
        let left = image::open(self.left.unwrap()).expect("can't open file for read");
        let right = image::open(self.right.unwrap()).expect("can't open file for read");
        let up = image::open(self.up.unwrap()).expect("can't open file for read");
        let down = image::open(self.down.unwrap()).expect("can't open file for read");
        let front = image::open(self.front.unwrap()).expect("can't open file for read");
        let back = image::open(&self.back.unwrap()).expect("can't open file for read");
    }
}

impl Default for SkyBoxFiles {
    fn default() -> SkyBoxFiles {
        SkyBoxFiles {
            left: None,
            right: None,
            up: None,
            down: None,
            front: None,
            back: None,
        }
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

        let expected = SkyBoxFiles {
            left: Some(l_path),
            right: Some(r_path),
            up: Some(u_path),
            down: Some(d_path),
            front: Some(f_path),
            back: Some(b_path),
        };

        let skyboxes = get_skyboxes(paths);

        assert!(skyboxes.len() == 1);

        assert_eq!(*skyboxes.first().unwrap(), expected);
    }
}
