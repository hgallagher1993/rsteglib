use std::f64;

use num_traits::float::Float;
use image;

pub struct DctObject {
    pub iteration: u32,
    pub inverse_coeffs: Vec<f64>,
    pub forward_coeffs: Vec<f64>,
    cu: f64,
    cv: f64,
    total: f64,
    index: usize
}

impl DctObject {
    pub fn new() -> DctObject {
        DctObject {
            cu: 0.0,
            cv: 0.0,
            total: 0.0,
            iteration: 0,
            index: 0,
            forward_coeffs: Vec::new(),
            inverse_coeffs: Vec::new()
        }
    }

    pub fn dct(&mut self, tiled_image: &mut Vec<image::Rgba<u8>>, channel: usize) {
        let mut colour_value: u8 = 0;

        for v in 0..8 {
            for u in 0..8 {
                for y in 0..8 {
                    for x in 0..8 {
                        self.index = u + (v * 8) + (self.iteration * 64) as usize;

                        colour_value = tiled_image[self.index].data[channel];

                        self.total = self.total + (v as f64 * f64::consts::PI * (2.0 * (y as f64) + 1.0) / 16.0).cos()
                                                * (u as f64 * f64::consts::PI * (2.0 * (x as f64) + 1.0) / 16.0).cos()
                                                * colour_value as f64;
                    }
                }

                self.ac_dc_check(u, v);

                // 0.25, Cu and Cv are scaling factors
                self.total = self.total * 0.25 * self.cu * self.cv;

                self.forward_coeffs.push(self.total);
            }
        }
    }

    pub fn i_dct(&mut self) {
        let mut freq_value: f64 = 0.0;

        for v in 0..8 {
            for u in 0..8 {
                for y in 0..8 {
                    for x in 0..8 {
                        self.index = u + (v * 8) + (self.iteration * 64) as usize;

                        freq_value = self.forward_coeffs[self.index];

                        self.ac_dc_check(u, v);

                        self.total = self.total * self.cu * self.cv + (v as f64 * f64::consts::PI * (2.0 * (y as f64) + 1.0) / 16.0).cos()
                                                * (u as f64 * f64::consts::PI * (2.0 * (x as f64) + 1.0) / 16.0).cos()
                                                * freq_value as f64;
                    }
                }

                self.total = self.total * 0.25;

                self.inverse_coeffs.push(self.total);
            }
        }
    }

    fn ac_dc_check(&mut self, u: usize, v: usize) {
        if u == 0 {
            self.cu = 1.0 / 2.0.sqrt()
        } else {
            self.cu = 1.0
        }

        if v == 0 {
            self.cv = 1.0 / 2.0.sqrt()
        } else {
            self.cv = 1.0
        }
    }
}