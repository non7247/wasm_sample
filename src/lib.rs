use wasm_bindgen::prelude::*;
use image::{Rgba, RgbaImage};

#[wasm_bindgen]
pub fn draw_image(width: u32, height: u32) -> Vec<u8> {
    let mut img = RgbaImage::new(width, height);
    draw(&mut img, 23, 0.0, 0.0, width, height);
    img.into_vec()
}

fn draw(img: &mut RgbaImage, k: i64, x: f64, y: f64, w: u32, h: u32) {
    let w1x = |x, y| 0.836 * x + 0.044 * y;
    let w1y = |x, y| -0.044 * x + 0.836 * y + 0.169;
    let w2x = |x, y| -0.141 * x + 0.302 * y;
    let w2y = |x, y| 0.302 * x + 0.141 * y + 0.127;
    let w3x = |x, y| 0.141 * x - 0.302 * y;
    let w3y = |x, y| 0.302 * x + 0.141 * y + 0.169;
    let w4x = |_x, _y| 0.0;
    let w4y = |_x, y| 0.175337 * y;

    if k > 0 {
        draw(img, k - 1, w1x(x, y), w1y(x, y), w, h);
        if lazyrand::rand_f64() < 0.3 {
            draw(img, k - 1, w2x(x, y), w2y(x, y), w, h);
        }
        if lazyrand::rand_f64() < 0.3 {
            draw(img, k - 1, w3x(x, y), w3y(x, y), w, h);
        }
        if lazyrand::rand_f64() < 0.3 {
            draw(img, k - 1, w4x(x, y), w4y(x, y), w, h);
        }
    }

    let ss = h as f64 * 0.97;
    let xx = (x * ss + (w as f64) * 0.5) as u32 - 1;
    let yy = ((h as f64) - y * ss) as u32 - 1;

    img.put_pixel(xx, yy, Rgba([0, 170, 0, 255]));
}
