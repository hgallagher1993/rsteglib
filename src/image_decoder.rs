use image::{RgbImage, ImageBuffer};
use discrete_transforms::dct_2d::Dct2D;
use itertools::Itertools;

#[derive(Debug)]
pub struct StegObject {
    steg_image: RgbImage,
    tiles: Vec<u8>,
    message_length: u32,
    dct_tiles: Vec<f64>,
    quantized_tiles: Vec<i32>,
    message_bits: Vec<u8>,
    message: String
}

impl StegObject {
    pub fn new() -> StegObject {
        StegObject {
            steg_image: ImageBuffer::new(0, 0),
            tiles: vec![],
            message_length: 0,
            dct_tiles: vec![],
            quantized_tiles: vec![],
            message_bits: vec![],
            message: String::new()
        }
    }

    pub fn set_steg_image(&mut self, input_path: &'static str) -> &mut Self {
        self.steg_image = image::open(input_path).unwrap().as_rgb8().unwrap().clone();

        self
    }

    pub fn set_message_length(&mut self, length: u32 ) -> &mut Self{
        self.message_length = length;

        self
    }

    pub fn decode(&mut self) {
        self.tile_image();

        let mut dct = Dct2D::new();

        for chunk in &self.tiles.iter().chunks(64) {
            let input = chunk.map(|x| *x as f64).collect_vec();

            dct.set_input(input);

            self.dct_tiles.extend(&dct.forward());
        }

        self.quantize();
        self.create_message();
    }

    fn tile_image(&mut self) {
        let (width, height) = self.steg_image.dimensions();
        let mut count = 0;

        for row_index in 0..(height / 8) as u32 {
            for col_index in 0..(width / 8) as u32 {
                for channel in 0..3 {
                    if count == self.message_length {
                        break
                    }

                    for row in 0..8 {
                        for column in 0..8 {
                            let pixel = self.steg_image.get_pixel(column + (col_index * 8),
                                                                     row + (row_index * 8));

                            pixel_count += 1;

                            self.tiles.push(pixel.data[channel]);
                        }
                    }

                    count += 1;
                }
            }
        }
    }

    fn quantize(&mut self) {
        for x in 0..self.dct_tiles.len() / 64 {
            let quantized_coeff = self.dct_tiles[x * 64].round() / 16.0;

            self.quantized_tiles.push(quantized_coeff.round() as i32);
        }
    }

    fn create_message(&mut self) {
        for chunk in &self.quantized_tiles.iter().chunks(8) {
            let mut byte = 0_u8;

            let mut message_bits = chunk.map(|b| (*b % 2) as u8).collect_vec();

            message_bits.reverse();

            for (index, bit) in message_bits.into_iter().enumerate() {
                let temp = bit << index as u8;
                byte = byte + temp;
            }

            self.message.push_str(&String::from_utf8(vec![byte; 1]).unwrap());
        }

        println!("{:?}", self.message);
    }
}

#[test]
fn test_decode() {
    let mut steg_object = StegObject::new();

    steg_object.set_message_length(8);
    steg_object.set_steg_image("output.png");
    steg_object.decode();
}