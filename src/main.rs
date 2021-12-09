// Project Plans:
// - text based file format
// - interpreter
// - compiler
// - create path from image
mod cursor_images;
mod make_image;
use make_image::make_image;

use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::time::Instant;

pub type Pos = i64;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Bounds {
    x1: Pos,
    y1: Pos,
    x2: Pos,
    y2: Pos,
}

impl Bounds {
    pub fn normalize(&self, x: Pos, y: Pos) -> (u32, u32) {
        assert!(self.x1 <= x && x <= self.x2);
        assert!(self.y1 <= y && y <= self.y2);
        // We flip y, since a smaller y makes a higher pixel on the image
        ((x - self.x1) as u32, (self.y2 - y) as u32)
    }

    pub fn zeros() -> Self {
        Self {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
        }
    }

    pub fn initial() -> Self {
        Self {
            x1: Pos::MAX,
            y1: Pos::MAX,
            x2: Pos::MIN,
            y2: Pos::MIN,
        }
    }

    pub fn add_point(&mut self, x: Pos, y: Pos) {
        self.x1 = self.x1.min(x);
        self.y1 = self.y1.min(y);
        self.x2 = self.x2.max(x);
        self.y2 = self.y2.max(y);
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct Point {
    x: Pos,
    y: Pos,
    dirs: u8, // Each bit represents a direction. 1 = right, 2 = right-up, 4 = up, etc.
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct Cursor {
    x: Pos,
    y: Pos,
    dir: u32, // Range 0-7 inclusive for each direction. 0 = right, 1 = right-up, 2 = up, etc.
}

impl Cursor {
    pub fn normalize(&self, p: &Point) -> u8 {
        p.dirs.rotate_right(self.dir)
    }
    pub fn move_in_dir(&mut self, move_dir: u8) {
        let world_dir = move_dir.rotate_left(self.dir);
        let dx = match world_dir {
            0x01 | 0x02 | 0x80 => 1,
            0x10 | 0x20 | 0x08 => -1,
            _ => 0,
        };
        let dy = match world_dir {
            0x02 | 0x04 | 0x08 => 1,
            0x20 | 0x40 | 0x80 => -1,
            _ => 0,
        };
        debug_assert!(dx != 0 || dy != 0);
        let new_dir = (self.dir + move_dir.trailing_zeros()) % 8;
        if new_dir == self.dir {
            self.x += dx;
            self.y += dy;
        }
        self.dir = new_dir;
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Program<'a> {
    points: &'a [Point],
    cursor: Cursor,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ProgramStop {
    Branch,
    Halted,
    Error(&'static str),
}

impl Program<'_> {
    pub fn step(&mut self) -> bool {
        match self.check_step() {
            Ok(dir) => {
                self.move_cursor(dir);
                //println!("Moving 0x{:02x}\n", dir);
                true
            }
            Err(e) => {
                println!("{:?}\n", e);
                false
            }
        }
    }

    pub fn check_step(&self) -> Result<u8, ProgramStop> {
        // Get cursor's position and find normalized directions
        let cur_point = self.get_point_at_cursor();
        let raw = self.cursor.normalize(cur_point);
        // bitwise AND for ignoring backwards direction
        let dirs = raw & 0xEF;
        // println!(
        //     "Current Dir: {} -> {}\n{:?}\n{:?}",
        //     raw, dirs, self.cursor, cur_point
        // );
        if dirs & 0x01 != 0 {
            Ok(0x01)
        } else if dirs & 0x02 != 0 {
            if dirs & 0x80 != 0 {
                Err(ProgramStop::Error(
                    "Illegal branch of forward diagonal paths",
                ))
            } else {
                Ok(0x02)
            }
        } else if dirs & 0x80 != 0 {
            Ok(0x80)
        } else if dirs & 0x04 != 0 {
            if dirs & 0x40 != 0 {
                Err(ProgramStop::Branch)
            } else {
                Ok(0x04)
            }
        } else if dirs & 0x40 != 0 {
            Ok(0x40)
        } else if dirs & 0x08 != 0 {
            if dirs & 0x20 != 0 {
                Err(ProgramStop::Error(
                    "Illegal branch of backwards diagonal paths",
                ))
            } else {
                Ok(0x08)
            }
        } else if dirs & 0x20 != 0 {
            Ok(0x20)
        } else {
            // The only other possible direction is directly backward
            Err(ProgramStop::Halted)
        }
    }

    pub fn move_cursor(&mut self, move_dir: u8) {
        self.cursor.move_in_dir(move_dir);
    }

    pub fn get_point_at_cursor(&self) -> &Point {
        for p in self.points {
            if p.x == self.cursor.x && p.y == self.cursor.y {
                return p;
            }
        }
        unreachable!("Didn't find point at position {:?}", self.cursor);
    }

    pub fn get_bounds(&self) -> Bounds {
        if self.points.is_empty() {
            return Bounds::zeros();
        }
        let mut b = Bounds::initial();
        for p in self.points {
            b.add_point(p.x, p.y);
        }
        b
    }
}

#[derive(Debug, Clone)]
struct ByteBuffer<'a> {
    bytes: &'a [u8],
    index: usize,
}

impl<'a> ByteBuffer<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { index: 0, bytes }
    }
    pub fn next(&mut self) -> Option<u8> {
        let x = self.bytes.get(self.index);
        match x {
            Some(val) => {
                self.index += 1;
                Some(*val)
            }
            None => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.index >= self.bytes.len()
    }

    pub fn read_compound_int(&mut self) -> Option<Pos> {
        let byte = self.next()?;
        let lookahead = byte.leading_ones();
        let mask = 0xFF_u8.checked_shr(lookahead).unwrap_or(0);
        let mut num = (byte & mask) as Pos;
        for _ in 0..lookahead {
            let b = self.next()?;
            //println!("Num {} consuming byte {}", num, b);
            num = (num << 8) + b as Pos;
        }
        Some(num)
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let f = read_file(&args[1])?;
    let mut bytes = ByteBuffer::new(&f);
    //println!("{:?}", &f);
    let mut points = Vec::new();
    let mut program = create_program(&mut bytes, &mut points).unwrap();
    drop(bytes);
    //println!("{:?}\n", &program);
    let mut sum_time = 0;
    let mut trials = 0;
    for i in 0..=69 {
        let now = Instant::now();
        make_image(&program, &format!("images/{}.png", i)).unwrap();
        if !program.step() {
            break;
        }
        let elapsed = now.elapsed().as_micros();
        println!("{}", elapsed);
        sum_time += elapsed;
        trials += 1;
    }
    println!("Mean time: {}", sum_time / trials);
    Ok(())
}

fn read_file(filepath: &str) -> io::Result<Vec<u8>> {
    let file = File::open(filepath)?;
    file.bytes().collect()
}

fn create_program<'a>(bytes: &mut ByteBuffer, points: &'a mut Vec<Point>) -> Option<Program<'a>> {
    let cursor_byte = bytes.next()?;
    assert_eq!(cursor_byte.count_ones(), 1, "Bad cursor direction");
    //println!("{:?}", cursor_byte);
    while !bytes.is_empty() {
        let x = bytes.read_compound_int()?;
        let y = bytes.read_compound_int()?;
        let dirs = bytes.next()?;
        let p = Point { x, y, dirs };
        //println!("{:?}", p);
        points.push(p);
    }
    let (x, y) = (points[0].x, points[0].y);
    let dir = cursor_byte.trailing_zeros();
    let cursor = Cursor { x, y, dir };
    Some(Program { points, cursor })
}
