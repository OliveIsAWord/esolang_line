use crate::{cursor_images, Bounds, Cursor, Point, Pos, Program};
use image::{self, Rgb, RgbImage};

const MARGIN: u32 = 20;
const UNIT_SIZE: u32 = 20;
const HALF_UNIT_SIZE: u32 = (UNIT_SIZE + 1) / 2; // division rounding up

static WHITE: Rgb<u8> = Rgb([255, 255, 255]);
static BLACK: Rgb<u8> = Rgb([0, 0, 0]);
static RED: Rgb<u8> = Rgb([255, 0, 0]);
static GREEN: Rgb<u8> = Rgb([0, 255, 0]);
#[allow(dead_code)]
static BLUE: Rgb<u8> = Rgb([0, 255, 0]);

pub fn make_image(program: &Program, path: &str) -> image::error::ImageResult<()> {
    let bounds = program.get_bounds();
    let width = UNIT_SIZE * (bounds.x2 - bounds.x1) as u32 + 2 * MARGIN + 1;
    let height = UNIT_SIZE * (bounds.y2 - bounds.y1) as u32 + 2 * MARGIN + 1;
    let mut img = RgbImage::from_pixel(width, height, WHITE);

    for p in program.points {
        draw_point(&mut img, p, bounds);
        //break;
    }
    draw_cursor(&mut img, program.cursor, bounds);
    img.save(path)?;
    Ok(())
}

static LINE_DIRS: [(u8, i32, i32); 8] = [
    (0x01, 1, 0),
    (0x02, 1, -1),
    (0x04, 0, -1),
    (0x08, -1, -1),
    (0x10, -1, 0),
    (0x20, -1, 1),
    (0x40, 0, 1),
    (0x80, 1, 1),
];

fn draw_point(img: &mut RgbImage, point: &Point, bounds: Bounds) {
    let (x, y) = to_coord(point.x, point.y, bounds);
    img.put_pixel(x, y, BLACK);
    for (mask, dx, dy) in LINE_DIRS {
        if point.dirs & mask != 0 {
            draw_line(img, x, y, dx, dy);
        }
    }
}

fn draw_line(img: &mut RgbImage, mut x: u32, mut y: u32, dx: i32, dy: i32) {
    for _ in 1..=HALF_UNIT_SIZE {
        x = x.wrapping_add(dx as u32);
        y = y.wrapping_add(dy as u32);
        img.put_pixel(x, y, BLACK);
    }
}

fn draw_cursor(img: &mut RgbImage, cursor: Cursor, bounds: Bounds) {
    let raster = if cursor.dir % 2 == 0 {
        cursor_images::CARDINAL
    } else {
        cursor_images::DIAGONAL
    };
    let rotation = cursor.dir / 2;
    let (cx, cy) = to_coord(cursor.x, cursor.y, bounds);
    for x in 0..raster.width {
        for y in 0..raster.height {
            let pixel_color = match raster.get(x, y) {
                cursor_images::PixelType::Empty => continue,
                cursor_images::PixelType::Fill => RED,
                cursor_images::PixelType::Border => GREEN,
            };
            let (x, y) = (x as isize, y as isize);
            let ox = raster.offset_x as isize;
            let oy = raster.offset_y as isize;
            let (draw_x, draw_y) = match rotation {
                0 => (x - ox, y - oy),
                1 => (y - oy, ox - x),
                2 => (ox - x, oy - y),
                3 => (oy - y, x - ox),
                _ => unreachable!(),
            };
            let imgx = cx.wrapping_add(draw_x as u32);
            let imgy = cy.wrapping_add(draw_y as u32);
            img.put_pixel(imgx, imgy, pixel_color);
        }
    }
}

fn to_coord(x: Pos, y: Pos, bounds: Bounds) -> (u32, u32) {
    let (px, py) = bounds.normalize(x, y);
    (px * UNIT_SIZE + MARGIN, py * UNIT_SIZE + MARGIN)
}
