use image::{RgbImage, ImageBuffer};

#[derive(Debug)]
pub struct CoverImage {
    message: &'static str,
    output_image_path: &'static str,
    cover_image: RgbImage
}

impl CoverImage {
    pub fn new() -> CoverImage {
        CoverImage {
            message: "",
            output_image_path: "",
            cover_image: ImageBuffer::new(0, 0)
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

    }
}