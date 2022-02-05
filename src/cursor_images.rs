// @generated
// Includes: cardinal.png diagonal.png
// Auto-generated on 2021-12-08 00:34:38
// Not intended for manual editing

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct BitRaster<'a> {
    pub width: usize,
    pub height: usize,
    pub offset_x: usize,
    pub offset_y: usize,
    pub pixels: &'a [bool],
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PixelType {
    Empty,
    Fill,
    Border,
}

impl BitRaster<'_> {
    pub fn get(&self, x: usize, y: usize) -> PixelType {
        if self.get_raw(x, y) {
            PixelType::Fill
        } else if self.is_bordering(x, y) {
            PixelType::Border
        } else {
            PixelType::Empty
        }
    }

    pub fn is_bordering(&self, x: usize, y: usize) -> bool {
        for dx in x.saturating_sub(1)..self.width.min(x + 2) {
            for dy in y.saturating_sub(1)..self.height.min(y + 2) {
                // if x == dx && y == dy {
                //     continue;
                // }
                if self.get_raw(dx, dy) {
                    return true;
                }
            }
        }
        false
    }

    fn get_raw(&self, x: usize, y: usize) -> bool {
        self.pixels[y * self.width + x]
    }
}

const W: bool = true;
#[allow(non_upper_case_globals)]
const o: bool = false;

const CARDINAL_BITS: [bool; 12 * 13] = [
    o, o, o, o, o, o, o, o, o, o, o, o, o, W, o, o, o, o, o, o, o, o, o, o, o, W, W, W, o, o, o, o,
    o, o, o, o, o, o, W, W, W, W, o, o, o, o, o, o, o, o, W, W, W, W, W, W, o, o, o, o, o, o, o, W,
    W, W, W, W, W, W, o, o, o, o, o, W, W, W, W, W, W, W, W, o, o, o, o, W, W, W, W, W, W, W, o, o,
    o, o, W, W, W, W, W, W, o, o, o, o, o, o, W, W, W, W, o, o, o, o, o, o, o, W, W, W, o, o, o, o,
    o, o, o, o, o, W, o, o, o, o, o, o, o, o, o, o, o, o, o, o, o, o, o, o, o, o, o, o,
];

pub const CARDINAL: BitRaster = BitRaster {
    width: 12,
    height: 13,
    offset_x: 10,
    offset_y: 6,
    pixels: &CARDINAL_BITS,
};

const DIAGONAL_BITS: [bool; 12 * 12] = [
    o, o, o, o, o, o, o, o, o, o, o, o, o, o, o, o, o, o, o, o, W, W, W, o, o, o, o, o, o, o, W, W,
    W, W, W, o, o, o, o, o, W, W, W, W, W, W, W, o, o, W, W, W, W, W, W, W, W, W, o, o, o, o, W, W,
    W, W, W, W, W, W, o, o, o, o, o, o, W, W, W, W, W, o, o, o, o, o, o, o, o, W, W, W, W, o, o, o,
    o, o, o, o, o, o, W, W, o, o, o, o, o, o, o, o, o, o, W, W, o, o, o, o, o, o, o, o, o, o, o, W,
    o, o, o, o, o, o, o, o, o, o, o, o, o, o, o, o,
];

pub const DIAGONAL: BitRaster = BitRaster {
    width: 12,
    height: 12,
    offset_x: 10,
    offset_y: 1,
    pixels: &DIAGONAL_BITS,
};
