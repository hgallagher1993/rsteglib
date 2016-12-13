extern crate image;
extern crate bitreader;

use std::path::Path;

use image::*;

use bitreader::BitReader;

pub struct StegoObject {
    cover_image: DynamicImage
}

impl StegoObject
{
    pub fn new(file_path: &str) -> StegoObject {
        StegoObject {
            cover_image: image::open(&Path::new(&file_path)).unwrap()
        }
    }

    pub fn encode_with(&mut self, message: &str) -> RgbImage {
        let bit_vector = get_bit_vec(message);

        let encoded_image = encode(&mut self.cover_image, &bit_vector);

        encoded_image
    }
}

fn get_bit_vec(message: &str) -> Vec<u8> {
    let mut bit_vector = Vec::new();
    let mut reader = BitReader::new(&message.as_bytes());


    for _ in 0..message.len() * 8 {
        bit_vector.push(reader.read_u8(1).unwrap());
    }

    bit_vector
}

fn encode(c_image: &mut DynamicImage, bit_vec: &Vec<u8>) -> RgbImage {
    let mut rgb_img = c_image.to_rgb();
    let mut bit_to_hide = 0u8;

    for (index, pixel) in rgb_img.pixels_mut().enumerate() {
        if index >= 8 {
            break;
        }

        bit_to_hide = bit_vec[index];

        pixel.data[index % 3] = (pixel.data[index % 3] & 0xFE) | bit_to_hide;
    }

    rgb_img
}

/*
    Tests
*/

#[test]
fn test_get_bit_vec_len() {
    let bits = get_bit_vec("h");

    assert_eq!(8, bits.len());
}

#[test]
fn test_get_bit_vec() {
    let test_bits = vec![0, 1, 1, 0, 1, 0, 0, 0];
    let bits = get_bit_vec("h");

    assert_eq!(bits, test_bits);
}

#[test]
fn test_encode_red_channel_lsb_set() {
    let mut img = image::open(&Path::new("/home/hugh/Pictures/scenery.jpg")).unwrap();
    let bit_vec = vec![0, 1, 1, 0, 1, 0, 0, 0];

    let encoded_image = encode(&mut img, &bit_vec);

    let pixel = img.get_pixel(0, 0);

    assert_eq!(pixel.data[0] % 2, 0);
}

#[test]
fn test_encode_green_channel_lsb_set() {
    let mut img = image::open(&Path::new("/home/hugh/Pictures/scenery.jpg")).unwrap();
    let bit_vec = vec![0, 1, 1, 0, 1, 0, 0, 0];

    let encoded_image = encode(&mut img, &bit_vec);

    let pixel = img.get_pixel(0, 0);

    assert_eq!(pixel.data[1] % 2, 1);
}

#[test]
fn test_encode_blue_channel_lsb_set() {
    let mut img = image::open(&Path::new("/home/hugh/Pictures/scenery.jpg")).unwrap();
    let bit_vec = vec![0, 1, 1, 0, 1, 0, 0, 0];

    let encoded_image = encode(&mut img, &bit_vec);

    let pixel = img.get_pixel(0, 0);

    assert_eq!(pixel.data[2] % 2, 1);
}

/*#[test]
fn test_full_byte_is_encoded() {
    let mut img = image::open(&Path::new("/home/hugh/Pictures/scenery.jpg")).unwrap();
    let bit_vec = vec![0, 1, 1, 0, 1, 0, 0, 0];

    let encoded_image = encode(&mut img, &bit_vec);


}*/


