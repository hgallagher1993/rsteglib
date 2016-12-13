extern crate rsteglib;
extern crate image;

use rsteglib::StegoObject;
use image::GenericImage;

#[test]
fn test_create_struct() {
    let path = "/home/hugh/Pictures/scenery.jpg";
    StegoObject::new(path);
}

#[test]
fn test_encode_with_method() {
    let path = "/home/hugh/Pictures/scenery.jpg";
    let o = StegoObject::new(path).encode_with("h");
}





