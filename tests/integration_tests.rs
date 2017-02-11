extern crate rsteglib;
extern crate image;

use rsteglib::StegObject;
use image::GenericImage;

#[test]
fn test_create_struct() {
    let path = "/home/hugh/Pictures/scenery.jpg";
    StegObject::new(path);
}

#[test]
fn test_encode_with_method() {
    let path = "/home/hugh/Pictures/scenery.jpg";
    let o = StegObject::new(path).encode_with("h");
}





