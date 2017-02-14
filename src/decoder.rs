use std::path::Path;

use image::DynamicImage;
use image;

pub struct StegObject {
    steg_object: DynamicImage
}

impl StegObject {
    pub fn new (file_path: &str) -> StegObject {
        StegObject {
            steg_object: image::open(&Path::new(&file_path)).unwrap()
        }
    }

    //pub fn decode (&self) -> &str {

    //}
}