use image::{ImageBuffer, Rgb};

use crate::node::NodePtr;

pub fn gen_img(
    width: u32,
    height: u32,
    tree: &(NodePtr, NodePtr, NodePtr),
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut img_buf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        let x_frac = x as f64 / width as f64;
        let y_frac = y as f64 / height as f64;
        let r = tree.0.get_value(x_frac, y_frac) * 255.;
        let g = tree.1.get_value(x_frac, y_frac) * 255.;
        let b = tree.2.get_value(x_frac, y_frac) * 255.;

        *pixel = image::Rgb([r as u8, g as u8, b as u8])
    }

    img_buf
}
