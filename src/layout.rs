use crate::container::*;
use crate::size_utilis::*;

pub enum Direction {
    Vertical,
    Horizontal,
}

pub struct Layout {
    base_rect: Rect,
    nb_child: usize,
    direction: Direction,
}

impl Layout {
    pub fn new(base_rect: Rect, direction: Direction) -> Layout {
        Layout {
            base_rect,
            nb_child: 0,
            direction,
        }
    }

    pub fn add_child(&mut self) -> Rect {
        self.nb_child += 1;
        match &self.direction {
            Direction::Vertical => Rect::new(
                self.base_rect.x,
                self.base_rect.y,
                self.base_rect.w,
                self.base_rect.h / self.nb_child as u16,
            ),
            Direction::Horizontal => Rect::new(
                self.base_rect.x,
                self.base_rect.y,
                self.base_rect.w / self.nb_child as u16,
                self.base_rect.h,
            ),
        }
    }
    pub fn del_child(&mut self) -> Rect {
        self.nb_child -= 1;
        match &self.direction {
            Direction::Vertical => Rect::new(
                self.base_rect.x,
                self.base_rect.y,
                self.base_rect.w,
                self.base_rect.h / self.nb_child as u16,
            ),
            Direction::Horizontal => Rect::new(
                self.base_rect.x,
                self.base_rect.y,
                self.base_rect.w / self.nb_child as u16,
                self.base_rect.h,
            ),
        }
    }
}
