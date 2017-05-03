use std::path::Path;
use std::fs::File;
use std::f64;

use image::{ DynamicImage, GenericImage, ImageFormat };
use image;

use bitreader::BitReader;

use num_traits::float::Float;

pub struct CoverImage {
    cover_image: DynamicImage
}

impl CoverImage {
    pub fn new(file_path: &str) -> CoverImage {
        CoverImage {
            cover_image: image::open(&Path::new(&file_path)).unwrap()
        }
    }

    pub fn encode_with(&self, message: &str) {
        let bit_vector = get_bit_vec(message);

        encode(&self.cover_image, &bit_vector);

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

// Encode the image with the Vector of Bits
fn encode(c_image: &DynamicImage, bit_vec: &Vec<u8>) {
    let mut tiled_image_vec = tile_image(&c_image);

    //let (width, height) = c_image.dimensions();

    let encoded_image = encode_image(&mut tiled_image_vec, bit_vec);
}

fn tile_image(c_image: &DynamicImage) -> Vec<image::Rgba<u8>> {
    let (width, height) = c_image.dimensions();

    let mut image_blocks = Vec::new();

    for row_index in 0..(height / 8) as u32 {
        for col_index in 0..(width / 8) as u32 {
            for row in 0..8 {
                for column in 0..8 {
                    image_blocks.push(c_image.get_pixel(column + (col_index * 8), row + (row_index * 8)));
                }
            }
        }
    }

    image_blocks
}

fn encode_image(tiled_image: &mut Vec<image::Rgba<u8>>, message: &Vec<u8>) {
    let mut cu = 0.0;
    let mut cv = 0.0;
    let mut total = 0.0;
    let mut inverse_total = 0.0;
    let mut index: usize = 0;
    let mut count = 0;
    let mut rgb_coeffs: Vec<f64> = Vec::new();
    let mut colour_value: u8 = 0;

    let num_of_iterations = if message.len() / 3 == 0 {
                                    (message.len() / 3) as u32
                                } else {
                                    ((message.len() / 3) + 1) as u32
                                };

    for iteration in 0..num_of_iterations {
        for channel in 0..3 {
            for v in 0..8 {
                for u in 0..8 {
                    // Forward transform
                    for y in 0..8 {
                        for x in 0..8 {
                            index = u + (v * 8) + (iteration * 64) as usize;

                            colour_value = tiled_image[index].data[channel];

                            if u == 0 {
                                cu = 1.0 / 2.0.sqrt()
                            } else {
                                cu = 1.0
                            }

                            if v == 0 {
                                cv = 1.0 / 2.0.sqrt()
                            } else {
                                cv = 0.0
                            }

                            total = total + (v as f64 * f64::consts::PI * (2.0 * (y as f64) + 1.0) / 16.0).cos()
                                          * (u as f64 * f64::consts::PI * (2.0 * (x as f64) + 1.0) / 16.0).cos()
                                          * colour_value as f64;
                        }
                    }

                    // 0.25, Cu and Cv are scaling factors
                    total = total * 0.25 * cu * cv;

                    rgb_coeffs.push(total);
                }
            }

            // Encode the message
            let coeff_to_mod = (27 + (iteration * 64)) as usize;

            if rgb_coeffs[coeff_to_mod].trunc() % 2.0 == 0.0 {
                if message[count] == 1 {
                    rgb_coeffs[coeff_to_mod] = rgb_coeffs[coeff_to_mod] + 1.0
                }
            } else {
                if message[count] == 0 {
                    rgb_coeffs[coeff_to_mod] = rgb_coeffs[coeff_to_mod] + 1.0
                }
            }

            // Inverse transform
            for v in 0..8 {
                for u in 0..8 {
                    if u == 0 {
                        cu = 0.0
                    } else {
                        cu = 1.0 / 2.0.sqrt()
                    }

                    if v == 0 {
                        cv = 0.0
                    } else {
                        cv = 1.0 / 2.0.sqrt()
                    }

                    inverse_total = inverse_total + (v as f64 * f64::consts::PI * (2.0 * (y as f64) + 1.0) / 16.0).cos()
                                                  * (u as f64 * f64::consts::PI * (2.0 * (x as f64) + 1.0) / 16.0).cos()
                                                  * colour_value as f64;
                }
            }
        }
    }
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

#[cfg(test)]
#[test]
fn test_tile_image_length() {
    let img = image::open(&Path::new("/home/hugh/Pictures/colour.jpg")).unwrap();

    let vec = tile_image(&img);

    assert_eq!(vec.len(), 91200);
}

#[cfg(test)]
#[test]
fn test_tile_image_first_pixel() {
    let img = image::open(&Path::new("/home/hugh/Pictures/project.jpg")).unwrap();

    let pixel = img.get_pixel(0, 0);

    let vec = tile_image(&img);

    assert_eq!(vec[0], pixel);
}

#[cfg(test)]
#[test]
fn test_tile_image_random_pixel1() {
    let img = image::open(&Path::new("/home/hugh/Pictures/project.jpg")).unwrap();

    let pixel = img.get_pixel(15, 7);

    let vec = tile_image(&img);

    assert_eq!(vec[127], pixel);
}

#[cfg(test)]
#[test]
fn test_tile_image_random_pixel2() {
    let img = image::open(&Path::new("/home/hugh/Pictures/project.jpg")).unwrap();

    let pixel = img.get_pixel(40, 16);

    let vec = tile_image(&img);

    assert_eq!(vec[6720], pixel);
}

#[cfg(test)]
#[test]
fn test_tile_image_random_pixel3() {
    let img = image::open(&Path::new("/home/hugh/Pictures/project.jpg")).unwrap();

    let pixel = img.get_pixel(15, 7);

    let vec = tile_image(&img);

    assert_eq!(vec[127], pixel);
}

#[cfg(test)]
#[test]
fn test_tile_image_last_pixel() {
    let img = image::open(&Path::new("/home/hugh/Pictures/project.jpg")).unwrap();

    let pixel = img.get_pixel(399, 399);

    let vec = tile_image(&img);

    assert_eq!(vec[159999], pixel);
}
