mod cursor_images;
mod make_image;
mod path;
mod utility;

use make_image::make_image;
use path::{Cursor, Path, Point};
use utility::Pos;

use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::time::Instant;

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

fn main() {
    let args: Vec<String> = env::args().collect();
    let f;
    if let Some(fp) = args.get(1) {
        f = read_file(fp).expect("error reading file");
    } else {
        eprintln!("USAGE: {} file_path", args[0]);
        return;
    };
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
        println!("{}μs", elapsed);
        sum_time += elapsed;
        trials += 1;
    }
    println!("Mean time: {}", sum_time / trials);
}

fn read_file(filepath: &str) -> io::Result<Vec<u8>> {
    let file = File::open(filepath)?;
    file.bytes().collect()
}

fn create_program<'a>(bytes: &mut ByteBuffer, points: &'a mut Vec<Point>) -> Option<Path<'a>> {
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
    Some(Path { points, cursor })
}
