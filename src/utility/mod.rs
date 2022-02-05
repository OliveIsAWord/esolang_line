pub mod colors;

pub type Pos = i64;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Bounds {
    pub x1: Pos,
    pub y1: Pos,
    pub x2: Pos,
    pub y2: Pos,
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
