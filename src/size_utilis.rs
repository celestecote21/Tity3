use std::ops;

pub struct Size {
    pub w: u16,
    pub h: u16,
}

impl Size {
    pub fn to_c_size(&self) -> libc::winsize {
        libc::winsize {
            ws_row: self.h,
            ws_col: self.w,
            ws_xpixel: 0,
            ws_ypixel: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Coordinate {
    pub x: u16,
    pub y: u16,
}

impl Coordinate {
    pub fn to_size(&self) -> Size {
        Size {
            w: self.x,
            h: self.y,
        }
    }
    pub fn goto_string(&self) -> String {
        termion::cursor::Goto(self.x + 1, self.y + 1).to_string()
    }

    pub fn copy(&mut self, other: &Coordinate) {
        self.x = other.x;
        self.y = other.y;
    }
    pub fn add(&self, other: Coordinate) -> Coordinate {
        Coordinate {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

impl Rect {
    pub fn new(x: u16, y: u16, w: u16, h: u16) -> Rect {
        Rect { x, y, w, h }
    }

    pub fn from_tupple(tuple: (u16, u16)) -> Rect {
        Rect {
            x: 0,
            y: 0,
            w: tuple.0,
            h: tuple.1,
        }
    }

    pub fn get_size(&self) -> Size {
        Size {
            w: self.w,
            h: self.h,
        }
    }

    pub fn get_origine(&self) -> Coordinate {
        Coordinate {
            x: self.x,
            y: self.y,
        }
    }

    pub fn copy(&mut self, rect: &Rect) {
        self.x = rect.x;
        self.y = rect.y;
        self.w = rect.w;
        self.h = rect.h;
    }
}
