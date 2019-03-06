extern crate rsteglib;

use rsteglib::image_encoder::CoverImage;

#[test]
fn create() {
    let cover_image = CoverImage::new()
        .set_cover_image("/home/hugh/Pictures/static.politico.com.jpeg")
        .set_message("whatever")
        .set_output_image_path("hdsd");
}
