
pub struct Size {
    pub w: u16,
    pub h: u16,
}

impl Size {
    pub fn to_c_size(&self) -> libc::winsize
    {
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
    pub fn to_size(&self) -> Size
    {
        Size{
            w: self.x,
            h: self.y
        }
    }
    pub fn goto_string(&self) -> String
    {
        termion::cursor::Goto(self.x + 1, self.y + 1).to_string()
    }
}

#[derive(Clone)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

impl Rect {
    pub fn new(x: u16, y: u16, w: u16, h: u16) -> Rect
    {
        Rect {
            x,
            y,
            w,
            h,
        }
    }
    pub fn get_size(&self) -> Size
    {
        Size {w: self.w, h: self.h}
    }
    pub fn get_origine(&self) -> Coordinate
    {
        Coordinate {x: self.x, y: self.y}
    }
}

