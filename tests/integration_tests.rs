extern crate rsteglib;
extern crate image;

use rsteglib::encoder::CoverImage;
use rsteglib::decoder::StegObject;

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
    StegObject::new(path);
}

#[test]
fn test_decode_method() {
    let path = "/home/hugh/Pictures/yurt.png";
    //let bit_vec = vec![0, 1, 1, 0, 1, 0, 0, 0];
    let vec = StegObject::new(path).decode();
    let message = String::from_utf8(vec).unwrap();

    println!("{}", message);

    assert_eq!(message, "h");
}





