use std::path::Path;
use std::str;
use std::f64;

use image::{DynamicImage, GenericImage};
use image;

use num_traits::float::Float;

pub struct StegObject {
    steg_object: DynamicImage,
}

impl StegObject {
    pub fn new(file_path: &str) -> StegObject {
        StegObject {
            steg_object: image::open(&Path::new(&file_path)).unwrap()
        }
    }

    pub fn decode(&self) -> Vec<u8> {
        let mut bit_vec: Vec<u8> = Vec::<u8>::new();
        let mut count = 0;
        let mut byte_vec = vec![0];
        let mut temp = 0;

        let mut tiled_image_vec = tile_image(&self.steg_object);

        let mut cu = 0.0;
        let mut cv = 0.0;
        let mut total = 0.0;
        let mut inverse_total = 0.0;
        let mut index: usize = 0;
        let mut count = 0;
        let mut dct_coeffs: Vec<f64> = Vec::new();
        let mut idct_coeffs: Vec<f64> = Vec::new();
        let mut colour_value: u8 = 0;
        let mut freq_value: f64 = 0.0;

        for iteration in 0..8 {
            for channel in 0..3 {
                // Forward Transform
                for v in 0..8 {
                    for u in 0..8 {
                        for y in 0..8 {
                            for x in 0..8 {
                                index = u + (v * 8) + (iteration * 64) as usize;

                                colour_value = tiled_image_vec[index].data[channel];

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

                        dct_coeffs.push(total);
                    }
                }
            }
        }

        'outer: for x_co_ord in 0..3 {
            let pixel = self.steg_object.get_pixel(x_co_ord, 0);

            for channel in 0..3 {
                if count >= 8 {
                    break 'outer;
                }

                bit_vec.push(pixel.data[channel] % 2);

                count += 1;
            }
        }

        bit_vec.reverse();

        for (index, val) in bit_vec.into_iter().enumerate() {
            temp = val << index;
            byte_vec[0] = byte_vec[0] + temp;

            println!("{}. . .{}. . .{}", index, temp, byte_vec[0]);
        }

        byte_vec
    }
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
