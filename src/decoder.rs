use std::path::Path;
use std::str;

use image::{DynamicImage, GenericImage};
use image;

pub struct StegObject {
    steg_object: DynamicImage
}

impl StegObject {
    pub fn new (file_path: &str) -> StegObject {
        StegObject {
            steg_object: image::open(&Path::new(&file_path)).unwrap()
        }
    }

    pub fn decode (&self) -> Vec<u8> {
        let mut bit_vec: Vec<u8> = Vec::<u8>::new();
        let mut count = 0;
        let mut byte_vec = vec![0];
        let mut temp = 0;

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


/*
#[test]
fn test_message_is_decoded() {
    let path = "/home/hugh/Pictures/yurt.png";
    let so = StegObject::new(path);

    let message = decode(so);

    assert_eq!(message, 'h');
}*/
