use crate::size_utilis::*;

#[derive(Clone, Copy)]
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

    pub fn get_direction(&self) -> Direction {
        self.direction
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn adding_child_test() {
        let base_rect = Rect::from_tupple(termion::terminal_size().unwrap());
        let mut layout = Layout::new(base_rect.clone(), Direction::Horizontal);
        assert_eq!(base_rect, layout.add_child());
        assert_eq!(base_rect.w / 2, layout.add_child().w);
        assert_eq!(base_rect.w / 3, layout.add_child().w);
    }

    #[test]
    fn del_child_test() {
        let base_rect = Rect::from_tupple(termion::terminal_size().unwrap());
        let mut layout = Layout::new(base_rect.clone(), Direction::Horizontal);
        assert_eq!(base_rect, layout.add_child());
        assert_eq!(base_rect.w / 2, layout.add_child().w);
        assert_eq!(base_rect.w / 3, layout.add_child().w);
        assert_eq!(base_rect.w / 2, layout.del_child().w);
        assert_eq!(base_rect.w, layout.del_child().w);
    }
}
