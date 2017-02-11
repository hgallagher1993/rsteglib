extern crate image;
extern crate bitreader;

use std::path::Path;
use std::fs::File;

use image::{DynamicImage, GenericImage, ImageFormat};

use bitreader::BitReader;

pub struct StegObject {
    cover_image: DynamicImage
}

impl StegObject
{
    pub fn new(file_path: &str) -> StegObject {
        StegObject {
            cover_image: image::open(&Path::new(&file_path)).unwrap()
        }
    }

    pub fn encode_with(&mut self, message: &str) {
        let bit_vector = get_bit_vec(message);

        encode(&mut self.cover_image, &bit_vector);

        let ref mut fout = File::create(&Path::new("/home/hugh/Pictures/yurt.png")).unwrap();

        let _ = self.cover_image.save(fout, ImageFormat::PNG).unwrap();
    }
}

// Get a Vector of Bits to Encode the Image with
fn get_bit_vec(message: &str) -> Vec<u8> {
    let mut bit_vector = Vec::new();
    let mut reader = BitReader::new(&message.as_bytes());


    // Multipled by 8 because it's a Vec of bits not bytes
    for _ in 0..message.len() * 8 {
        bit_vector.push(reader.read_u8(1).unwrap());
    }

    bit_vector
}

// Encode the Image with the Vector of Bits
fn encode(c_image: &mut DynamicImage, bit_vec: &Vec<u8>) {
    let (width, height) = c_image.dimensions();

    let mut img = c_image.as_mut_rgb8().unwrap();

    let mut index = 0;

    'outer: for y_co_ord in 0..height {
        for x_co_ord in 0..width {
            let pixel = img.get_pixel_mut(x_co_ord, y_co_ord);

            for channel in 0..3 {
                if index >= bit_vec.len() {
                    println!("{}", pixel.data[1]);
                    break 'outer;
                }

                pixel.data[channel] = (pixel.data[channel] & 0xFE) | bit_vec[index];

                index += 1;
            }
        }
    }
}

/***************************************************************************************************
 *                                                                                                 *
 *                                         Tests                                                   *
 *                                                                                                 *
 ***************************************************************************************************/

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

    encode(&mut img.as_mut_rgb8().unwrap(), &bit_vec);

    let pixel = img.get_pixel(0, 0);

    assert_eq!(pixel.data[0] % 2, bit_vec[0]);
}

#[test]
fn test_encode_green_channel_lsb_set() {
    let mut img = image::open(&Path::new("/home/hugh/Pictures/scenery.jpg")).unwrap();
    let bit_vec = vec![0, 1, 1, 0, 1, 0, 0, 0];

    encode(&mut img.as_mut_rgb8().unwrap(), &bit_vec);

    let pixel = img.get_pixel(0, 0);

    assert_eq!(pixel.data[1] % 2, bit_vec[1]);
}

#[test]
fn test_encode_blue_channel_lsb_set() {
    let mut img = image::open(&Path::new("/home/hugh/Pictures/scenery.jpg")).unwrap();
    let bit_vec = vec![0, 1, 1, 0, 1, 0, 0, 0];

    encode(&mut img.as_mut_rgb8().unwrap(), &bit_vec);

    let pixel = img.get_pixel(0, 0);

    assert_eq!(pixel.data[2] % 2 as u8, bit_vec[2]);
}

#[test]
fn test_full_byte_is_encoded() {
    let mut img = image::open(&Path::new("/home/hugh/Pictures/scenery.jpg")).unwrap();
    let mut encoded_bit_vec = Vec::new();
    let mut count = 0;

    let bit_vec = vec![0, 1, 1, 0, 1, 0, 0, 0];

    encode(&mut img.as_mut_rgb8().unwrap(), &bit_vec);

    'outer: for x_co_ord in 0..3 {
        let pixel = img.get_pixel(x_co_ord, 0);

        for channel in 0..3 {
            if count >= 8 {
                break 'outer;
            }

            encoded_bit_vec.push(pixel.data[channel] % 2);

            count += 1;
        }
    }

    assert_eq!(encoded_bit_vec, bit_vec);
}


