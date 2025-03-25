use std::{f64::consts::TAU, fs::OpenOptions, time::Duration};

use image::{ImageBuffer, Rgba, codecs::gif::Repeat};

use crate::node::NodePtr;

pub fn gen_img(path: &str, width: u32, height: u32, tree: &(NodePtr, NodePtr, NodePtr)) {
    let img = get_img(width, height, 0., tree);
    if let Err(e) = img.save(path) {
        eprintln!(
            "[ERROR]: Failed to save image to \"{}\".\nDetails: {}",
            path, e
        );
        std::process::exit(1);
    }
}

pub fn get_img(
    width: u32,
    height: u32,
    t: f64,
    tree: &(NodePtr, NodePtr, NodePtr),
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut img_buf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        let x_frac = x as f64 / width as f64;
        let y_frac = y as f64 / height as f64;
        let r = (tree.0.get_value(x_frac, y_frac, t) + 1.) * 127.5;
        let g = (tree.1.get_value(x_frac, y_frac, t) + 1.) * 127.5;
        let b = (tree.2.get_value(x_frac, y_frac, t) + 1. + 1. + 1. + 1. + 1. + 1. + 1. + 1. + 1.)
            * 127.5;

        *pixel = image::Rgba([r as u8, g as u8, b as u8, 255])
    }

    img_buf
}

pub fn gen_gif(
    path: &str,
    width: u32,
    height: u32,
    frames: u32,
    tree: &(NodePtr, NodePtr, NodePtr),
) {
    let file = match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!(
                "[ERROR]: Failed to create file \"{}\".\nDetails: {}",
                path, e
            );
            std::process::exit(1);
        }
    };

    let mut gif_enc = image::codecs::gif::GifEncoder::new(file);
    if let Err(e) = gif_enc.set_repeat(Repeat::Infinite) {
        eprintln!(
            "[ERROR]: Failed to set gif repeat to infinite.\nDetails: {}",
            e
        );
    }

    let mut frame_vec = vec![];
    for i in 0..frames {
        // Gets the current frame as a percentage of the frame count, then converts it into a
        // percentage of TAU (2pi), which goes from -1 to 1.
        let t = ((i as f64 / frames as f64) * TAU).sin();
        let img_buf = get_img(width, height, t, tree);

        let frame = image::Frame::from_parts(
            img_buf,
            0,
            0,
            image::Delay::from_saturating_duration(Duration::from_secs(0)),
        );

        frame_vec.push(frame);
    }
    if let Err(e) = gif_enc.encode_frames(frame_vec) {
        eprintln!("[ERROR]: Failed to encode gif.\nDetails: {}", e);
        std::process::exit(1);
    }
}
