use std::path::Path;
use std::fs::File;

use image::{DynamicImage, GenericImage, ImageFormat};
use image;

use bitreader::BitReader;

pub struct CoverImage {
    cover_image: DynamicImage,
}

impl CoverImage {
    pub fn new(file_path: &str) -> CoverImage {
        CoverImage { cover_image: image::open(&Path::new(&file_path)).unwrap() }
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

    // Multiplied by 8 because it's a Vec of bits not bytes
    for _ in 0..message.len() * 8 {
        bit_vector.push(reader.read_u8(1).unwrap());
    }

    bit_vector
}

// Encode the Image with the Vector of Bits
fn encode(c_image: &mut DynamicImage, bit_vec: &Vec<u8>) {
    let mut image_blocks = Vec::new();
    let mut img = c_image.as_mut_rgb8().unwrap();
    let (width, height) = c_image.dimensions();

    // 9 is used here because each 8 x 8 block will have a dct for each colour and each colour will
    // hold 3 bits of information.
    let max_iterations = if bit_vec.len() / 9 == 0 { bit_vec.len() / 9 } else { (bit_vec.len() / 9 ) + 1 };

    let image_blocks = tile_image(img, max_iterations);
}

fn tile_image(image: &mut DynamicImage, max_iterations: u32) -> Vec<Pixel> {
    let mut blocks = Vec::new();

    for block_number in 0..max_iterations {
        let index = block_number * 8;

        for row in 0..8 {
            for column in 0..8 {
                blocks.push(image.get_pixel_mut(row, column + index));
            }
        }
    }

    blocks
}

/**************************************************************************************************
 *                                                                                                *
 *                                         Tests                                                  *
 *                                                                                                *
 **************************************************************************************************/

#[cfg(test)]
#[test]
fn test_get_bit_vec_len() {
    let bits = get_bit_vec("h");

    assert_eq!(8, bits.len());
}

#[cfg(test)]
#[test]
fn test_get_bit_vec() {
    let test_bits = vec![0, 1, 1, 0, 1, 0, 0, 0];
    let bits = get_bit_vec("h");

    assert_eq!(bits, test_bits);
}

#[cfg(test)]
#[test]
fn test_encode_red_channel_lsb_set() {
    let mut img = image::open(&Path::new("/home/hugh/Pictures/scenery.jpg")).unwrap();
    let bit_vec = vec![0, 1, 1, 0, 1, 0, 0, 0];

    encode(&mut img, &bit_vec);

    let pixel = img.get_pixel(0, 0);

    assert_eq!(pixel.data[0] % 2, bit_vec[0]);
}

#[cfg(test)]
#[test]
fn test_encode_green_channel_lsb_set() {
    let mut img = image::open(&Path::new("/home/hugh/Pictures/scenery.jpg")).unwrap();
    let bit_vec = vec![0, 1, 1, 0, 1, 0, 0, 0];

    encode(&mut img, &bit_vec);

    let pixel = img.get_pixel(0, 0);

    assert_eq!(pixel.data[1] % 2, bit_vec[1]);
}

#[cfg(test)]
#[test]
fn test_encode_blue_channel_lsb_set() {
    let mut img = image::open(&Path::new("/home/hugh/Pictures/scenery.jpg")).unwrap();
    let bit_vec = vec![0, 1, 1, 0, 1, 0, 0, 0];

    encode(&mut img, &bit_vec);

    let pixel = img.get_pixel(0, 0);

    assert_eq!(pixel.data[2] % 2 as u8, bit_vec[2]);
}

#[cfg(test)]
#[test]
fn test_full_byte_is_encoded() {
    let mut img = image::open(&Path::new("/home/hugh/Pictures/scenery.jpg")).unwrap();
    let mut encoded_bit_vec = Vec::new();
    let mut count = 0;

    let bit_vec = vec![0, 1, 1, 0, 1, 0, 0, 0];

    encode(&mut img, &bit_vec);

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
