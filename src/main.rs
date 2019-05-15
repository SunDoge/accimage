extern crate accimage;

use accimage::ImageObject;

fn main() {
    println!("Hello, world!");
    println!("{:?}", unsafe { *ipp_sys::ippGetLibVersion() });

    let mut image_object = ImageObject::from_jpeg("1920x1080.png");

    // println!("{:?}", image_object);

    image_object.flip_left_right();

    image::save_buffer(
        "image.png",
        &image_object.buffer,
        image_object.width as u32,
        image_object.height as u32,
        image::RGB(8),
    )
    .expect("fail to save");
}
