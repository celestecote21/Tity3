use crate::size_utilis::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Vertical,
    Horizontal,
}

impl Direction {
    pub fn check_move_dir(&self, dir: &MoveDir) -> bool {
        match self {
            Direction::Vertical => {
                if dir == &MoveDir::Down || dir == &MoveDir::Up {
                    true
                } else {
                    false
                }
            }
            Direction::Horizontal => {
                if dir == &MoveDir::Left || dir == &MoveDir::Right {
                    true
                } else {
                    false
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum MoveDir {
    Left,
    Right,
    Up,
    Down,
}

pub struct Layout {
    base_rect: Rect,
    nb_child: usize,
    direction: Direction,
    next_id: usize,
}

impl Layout {
    pub fn new(base_rect: Rect, direction: Direction) -> Layout {
        Layout {
            base_rect,
            nb_child: 0,
            direction,
            next_id: 1,
        }
    }

    pub fn add_child(&mut self) -> Rect {
        self.nb_child += 1;
        self.next_id += 1;
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

    pub fn get_next_id(&self) -> usize {
        self.next_id
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
        assert_eq!(2, layout.get_next_id());
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

    #[test]
    fn adding_child_test_vert() {
        let base_rect = Rect::from_tupple(termion::terminal_size().unwrap());
        let mut layout = Layout::new(base_rect.clone(), Direction::Vertical);
        assert_eq!(base_rect, layout.add_child());
        assert_eq!(base_rect.h / 2, layout.add_child().h);
        assert_eq!(base_rect.h / 3, layout.add_child().h);
    }

    #[test]
    fn del_child_test_vert() {
        let base_rect = Rect::from_tupple(termion::terminal_size().unwrap());
        let mut layout = Layout::new(base_rect.clone(), Direction::Vertical);
        assert_eq!(base_rect, layout.add_child());
        assert_eq!(base_rect.h / 2, layout.add_child().h);
        assert_eq!(base_rect.h / 3, layout.add_child().h);
        assert_eq!(base_rect.h / 2, layout.del_child().h);
        assert_eq!(base_rect.h, layout.del_child().h);
    }
}
