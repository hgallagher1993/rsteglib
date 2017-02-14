extern crate rsteglib;
extern crate image;

use rsteglib::encoder::CoverImage;

#[test]
fn test_create_struct_encoder() {
    let path = "/home/hugh/Pictures/scenery.jpg";
    CoverImage::new(path);
}

#[test]
fn test_encode_with_method() {
    let path = "/home/hugh/Pictures/scenery.jpg";
    let o = CoverImage::new(path).encode_with("h");
}

#[test]
fn test_create_struct_decoder() {
    let path = "/home/hugh/Pictures/yurt.png";
    CoverImage::new(path);
}






