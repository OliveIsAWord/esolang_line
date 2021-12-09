// This has too many generics, my eyes are bleeding
// Awful awful awful awful
// This was an educational experience, I hope I never see this much generic again in my life

use std::fs::File;
use std::io::{self, Error, ErrorKind, Read};

type Pos = u64;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
struct Point {
    x: Pos,
    y: Pos,
    dirs: DirBits,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
struct DirBits(u8);

fn main() -> io::Result<()> {
    println!("{:?}", read_file()?);
    Ok(())
}

trait ReadNext<Iter, Val>
where
    Iter: Iterator<Item = io::Result<Val>>,
{
    fn read_next(&mut self) -> io::Result<Val>;
}
impl<Iter: Iterator<Item = io::Result<Val>>, Val> ReadNext<Iter, Val> for Iter {
    fn read_next(&mut self) -> io::Result<Val> {
        match self.next() {
            Some(x) => x,
            None => Err(Error::from(ErrorKind::UnexpectedEof)),
        }
    }
}

fn read_file() -> io::Result<Vec<Point>> {
    let file = File::open("test.lin")?;
    let mut bytes = file.bytes().peekable();
    let mut points: Vec<Point> = Vec::new();
    loop {
        if let None = bytes.peek() {
            break;
        }
        let x = read_compound_int(&mut bytes)?;
        let y = read_compound_int(&mut bytes)?;
        let dirs = bytes.read_next()?;
        points.push(Point {
            x,
            y,
            dirs: DirBits(dirs),
        });
    }
    Ok(points)
}

fn read_compound_int<T, Iter>(bytes: &mut T) -> io::Result<Pos>
where
    T: ReadNext<Iter, u8>,
    Iter: Iterator<Item = Result<u8, std::io::Error>>,
{
    let byte = bytes.read_next()?;
    let lookahead = byte.leading_ones();
    let mask = 0xFF_u8.checked_shr(lookahead).unwrap_or(0);
    let mut num: Pos = (byte & mask) as Pos;
    for _ in 0..lookahead {
        let b = bytes.read_next()?;
        //println!("Num {} consuming byte {}", num, b);
        num = (num << 8) + b as Pos;
    }
    Ok(num)
}
