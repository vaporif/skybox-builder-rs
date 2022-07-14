use std::fs;

use image::DynamicImage;

pub fn merge_all_files(path: &str) {
    for file in fs::read_dir(path).expect("wrong path") {
        println!("{}", file.unwrap().path().display());
    }

    let img = image::open("tests/images/jpg/progressive/cat.jpg").unwrap();
}

struct SkyBoxFilePathGroup<'a> {
    left_side: &'a str,
    center: &'a str,
    right_side: &'a str,
    top_side: &'a str,
    bottom_side: &'a str
}

struct SkyBoxImageGroup {
    left_side: DynamicImage,
    center: DynamicImage,
    right_side: DynamicImage,
    top_side: DynamicImage,
    bottom_side: DynamicImage
}

impl SkyBoxImageGroup {
    fn new(file_group: SkyBoxFilePathGroup) -> SkyBoxImageGroup {
        Self {
            left_side: image::open(file_group.left_side).unwrap(),
            center: image::open(file_group.center).unwrap(),
            right_side: image::open(file_group.right_side).unwrap(),
            top_side: image::open(file_group.top_side).unwrap(),
            bottom_side: image::open(file_group.bottom_side).unwrap(),
        }
    }
}
