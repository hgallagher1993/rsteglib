use image::{ImageBuffer, RgbImage};
use bitreader::{BitReader};
use itertools::Itertools;
use discrete_transforms::dct_2d::Dct2D;

#[derive(Debug)]
pub struct CoverImage {
    cover_image: RgbImage,
    message: &'static str,
    output_image_path: &'static str,
    message_as_bits: Vec<u8>,
    tiles: Vec<u8>,
    dct_tiles: Vec<f64>,
    modified_pixels: Vec<f64>
}

impl CoverImage {
    pub fn new() -> CoverImage {
        CoverImage {
            cover_image: ImageBuffer::new(0, 0),
            message: "",
            output_image_path: "",
            message_as_bits: vec![],
            tiles: vec![],
            dct_tiles: vec![],
            modified_pixels: vec![]
        }
    }

    pub fn set_cover_image(&mut self, input_path: &'static str) -> &mut Self {
        self.cover_image = image::open(input_path).unwrap().as_rgb8().unwrap().clone();

        self
    }

    pub fn set_message(&mut self, message: &'static str) -> &mut Self {
        self.message = message;

        self
    }

    pub fn set_output_image_path(&mut self, output_path: &'static str) -> &mut Self {
        self.output_image_path = output_path;

        self
    }

    pub fn encode(&mut self) {
        self.get_message_as_bits();
        self.tile_image();
        self.encode_message();

        self.cover_image.save(self.output_image_path).unwrap();
    }

    fn get_message_as_bits(&mut self) {
        let mut reader = BitReader::new(&self.message.as_bytes());

        for _ in 0..self.message.len() * 8 {
            self.message_as_bits.push(reader.read_u8(1).unwrap());
        }
    }

    fn tile_image(&mut self) {
        let (width, height) = self.cover_image.dimensions();
        let mut count = 0;

        for row_index in 0..(height / 8) as u32 {
            for col_index in 0..(width / 8) as u32 {
                for channel in 0..3 {
                    if count == self.message_as_bits.len() {
                        break
                    }

                    for row in 0..8 {
                        for column in 0..8 {
                            let pixel = self.cover_image.get_pixel(column + (col_index * 8),
                                                                      row + (row_index * 8));

                            self.tiles.push(pixel.data[channel]);
                        }
                    }

                    count += 1;
                }
            }
        }
    }

    fn encode_message(&mut self) {
        let mut dct = Dct2D::new();

        for chunk in &self.tiles.iter().chunks(64) {
            let input = chunk.map(|x| *x as f64).collect_vec();

            dct.set_input(input);

            self.dct_tiles.extend(&dct.forward());
        }
    }
}

#[test]
fn get_message_as_bits_test() {
    let test_bits = vec![0, 1, 1, 0, 1, 0, 0, 0];
    let mut cover_image = CoverImage::new();

    cover_image.set_message("h");
    cover_image.get_message_as_bits();

    assert_eq!(cover_image.message_as_bits, test_bits);
}

#[test]
fn tile_image_length_test() {
    let mut cover_image = CoverImage::new();

    cover_image.set_message("h");
    cover_image.set_cover_image("src/testing.jpg");

    // 64 = length of 1 tile, message.len() * 8 = number of tiles needed; 1 tile per bit
    let length = 64 * cover_image.message.len() * 8;

    cover_image.get_message_as_bits();
    cover_image.tile_image();

    let tile_vec_length = cover_image.tiles.len();

    assert_eq!(length, tile_vec_length);
}

#[test]
fn test_random_pixel_in_tile() {
    let mut cover_image = CoverImage::new();

    cover_image.set_message("h");
    cover_image.set_cover_image("src/testing.jpg");
    cover_image.get_message_as_bits();
    cover_image.tile_image();

    let pixel = cover_image.cover_image.get_pixel(8, 8);

    let r_pixel = pixel.data[0];
    let g_pixel = pixel.data[1];
    let b_pixel = pixel.data[2];

    let r_pixel_from_tile = cover_image.tiles[63];
    let g_pixel_from_tile = cover_image.tiles[127];
    let b_pixel_from_tile = cover_image.tiles[191];

    assert_eq!(r_pixel, r_pixel_from_tile);
    assert_eq!(g_pixel, g_pixel_from_tile);
    assert_eq!(b_pixel, b_pixel_from_tile);
}

#[test]
fn delete() {
    let mut cover_image = CoverImage::new();

    cover_image.set_message("h");
    cover_image.set_cover_image("src/testing.jpg");
    cover_image.set_output_image_path("output.jpg");
    cover_image.encode();
}