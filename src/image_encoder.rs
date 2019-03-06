use image::{RgbImage, ImageBuffer};
use bitreader::BitReader;

#[derive(Debug)]
pub struct CoverImage {
    message: &'static str,
    output_image_path: &'static str,
    cover_image: RgbImage,
    message_as_bits: Vec<u8>
}

impl CoverImage {
    pub fn new() -> CoverImage {
        CoverImage {
            message: "",
            output_image_path: "",
            cover_image: ImageBuffer::new(0, 0),
            message_as_bits: vec![]
        }
    }

    pub fn set_cover_image(mut self, input_path: &'static str) -> Self {
        self.cover_image = image::open(input_path).unwrap().as_rgb8().unwrap().clone();

        self.cover_image.get_pixel_mut(0, 0);

        self
    }

    pub fn set_message(mut self, message: &'static str) -> Self {
        self.message = message;

        self
    }

    pub fn set_output_image_path(mut self, output_path: &'static str) -> Self {
        self.output_image_path = output_path;

        self
    }

    pub fn encode(&mut self) {
        self.get_message_as_bits();
    }

    fn get_message_as_bits(&mut self) {
        let mut reader = BitReader::new(&self.message.as_bytes());

        for _ in 0..self.message.len() * 8 {
            self.message_as_bits.push(reader.read_u8(1).unwrap());
        }
    }
}

#[test]
fn get_message_as_bits_test() {
    let test_bits = vec![0, 1, 1, 0, 1, 0, 0, 0];
    let mut cover_image = CoverImage::new().set_message("h");

    cover_image.get_message_as_bits();

    assert_eq!(cover_image.message_as_bits, test_bits);
}