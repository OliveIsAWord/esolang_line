use crate::utility::Bounds;
pub type Pos = i64;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct Point {
    pub x: Pos,
    pub y: Pos,
    pub dirs: u8, // Each bit represents a direction. 1 = right, 2 = right-up, 4 = up, etc.
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct Cursor {
    pub x: Pos,
    pub y: Pos,
    pub dir: u32, // Range 0-7 inclusive for each direction. 0 = right, 1 = right-up, 2 = up, etc.
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
        //if new_dir == self.dir {
        self.x += dx;
        self.y += dy;
        //}
        self.dir = new_dir;
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Path<'a> {
    pub points: &'a [Point],
    pub cursor: Cursor,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PathStop {
    Branch,
    Halted,
    Error(&'static str),
}

impl Path<'_> {
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

    pub fn check_step(&self) -> Result<u8, PathStop> {
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
                Err(PathStop::Error("Illegal branch of forward diagonal paths"))
            } else {
                Ok(0x02)
            }
        } else if dirs & 0x80 != 0 {
            Ok(0x80)
        } else if dirs & 0x04 != 0 {
            if dirs & 0x40 != 0 {
                Err(PathStop::Branch)
            } else {
                Ok(0x04)
            }
        } else if dirs & 0x40 != 0 {
            Ok(0x40)
        } else if dirs & 0x08 != 0 {
            if dirs & 0x20 != 0 {
                Err(PathStop::Error(
                    "Illegal branch of backwards diagonal paths",
                ))
            } else {
                Ok(0x08)
            }
        } else if dirs & 0x20 != 0 {
            Ok(0x20)
        } else {
            // The only other possible direction is directly backward
            Err(PathStop::Halted)
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
        let mut bound = Bounds::initial();
        for p in self.points {
            bound.add_point(p.x, p.y);
        }
        bound
    }
}
