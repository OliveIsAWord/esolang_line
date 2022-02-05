#![allow(dead_code)]

use image::Rgb;

pub type Color = Rgb<u8>;

pub static WHITE: Color = Rgb([255, 255, 255]);
pub static BLACK: Color = Rgb([0, 0, 0]);
pub static RED: Color = Rgb([255, 0, 0]);
pub static GREEN: Color = Rgb([0, 255, 0]);
pub static BLUE: Color = Rgb([0, 255, 0]);
