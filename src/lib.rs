
use image::GenericImage;
use ipp_sys::{
    ippiMirror_8u_C3IR, ippiResizeGetSize_8u, IppiAxis, IppiInterpolationType, IppiPoint, IppiSize,
};
use std::path::Path;


#[derive(Debug)]
pub struct ImageObject {
    pub buffer: Vec<u8>,
    pub channels: i32,
    pub height: i32,
    pub width: i32,
    pub row_stride: i32,
    pub y_offset: i32,
    pub x_offset: i32,
}

impl ImageObject {
    pub fn from_jpeg<P>(path: P) -> ImageObject
    where
        P: AsRef<Path>,
    {
        let img = image::open(path).expect("fail to open image");
        let rgb_img = img.to_rgb();

        ImageObject {
            buffer: rgb_img.to_vec(),
            channels: 3,
            height: rgb_img.height() as i32,
            width: rgb_img.width() as i32,
            row_stride: rgb_img.width() as i32,
            y_offset: 0,
            x_offset: 0,
        }
    }

    pub fn flip_left_right(&mut self) {
        let roi = IppiSize {
            width: self.width,
            height: self.height,
        };

        let offset = (self.y_offset * self.row_stride + self.x_offset) * self.channels;

        let ipp_status = unsafe {
            ippiMirror_8u_C3IR(
                self.buffer.as_mut_ptr().offset(offset as isize),
                self.row_stride * self.channels,
                roi,
                IppiAxis::ippAxsVertical,
            )
        };

        if ipp_status != 0 {
            panic!("ippiMirror_8u_C3IR failed with status {}", ipp_status);
        }
    }

    pub fn resize(&mut self, new_height: i32, new_width: i32, antialiasing: u32) {
        let mut new_buffer: Vec<u8> =
            Vec::with_capacity((new_height * new_width * self.channels) as usize);

        let old_size = IppiSize {
            width: self.width,
            height: self.height,
        };

        let new_size = IppiSize {
            width: new_width,
            height: new_height,
        };

        let new_offset = IppiPoint { x: 0, y: 0 };

        let mut specification_size = 0i32;
        let mut initialization_buffer_size = 0i32;
        let mut scratch_buffer_size = 0i32;

        let mut ipp_status = unsafe {
            ippiResizeGetSize_8u(
                old_size,
                new_size,
                IppiInterpolationType::ippLinear,
                antialiasing,
                &mut specification_size,
                &mut initialization_buffer_size,
            )
        };

        if ipp_status != 0 {
            panic!("ippiResizeGetSize_8u failed with status {}", ipp_status);
        }

        let mut initialization_buffer: Vec<u8> = Vec::with_capacity(initialization_buffer_size as usize);

        let mut specification: Vec<u8> = Vec::with_capacity(specification_size as usize);

        if antialiasing != 0 {

        } 
    }
}