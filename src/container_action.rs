//! # container_action
//!
//! A file containing all the action that a Container can do

use crate::container::*;
use crate::layout::*;
use crate::size_utilis::*;

/// Call the draw fonction of corresponding struct inside the Container
pub fn draw_container(cont: &mut Container, id: &str) {
    match cont {
        Container::Split(sp) => {
            sp.draw(id);
        }
        Container::Pane(pa) => {
            pa.draw(id);
        }
        _ => panic!("not ful container can't be drawn"),
    }
}

pub fn draw_cursor_container(cont: &mut Container) {
    match cont {
        Container::Split(sp) => sp.draw_cursor(),
        Container::Pane(pa) => pa.draw_cursor(),
        _ => panic!("not ful container can't be drawn"),
    }
}

/// Call the destroy fonction of corresponding struct inside the Container
pub fn destroy_container(cont: &mut Container, id: &str) -> Result<(), ()> {
    match cont {
        Container::Split(sp) => sp.destroy(id),
        Container::Pane(pa) => pa.destroy(id),
        _ => panic!("not ful container can't be drawn"),
    }
}

/// Call the get_id fonction of corresponding struct inside the Container
pub fn get_id_container(cont: &Container) -> &str {
    match cont {
        Container::Split(sp) => sp.get_id(),
        Container::Pane(pa) => pa.get_id(),
        _ => panic!("can't get this id"),
    }
}

/// Call the get_input fonction of corresponding struct inside the Container
pub fn get_input_container(data: [u8; 4096], size: usize, cont: &mut Container) {
    match cont {
        Container::Split(sp) => {
            sp.get_input(data, size);
        }
        Container::Pane(pa) => {
            pa.get_input(data, size).unwrap(); // TODO: need error handling
        }
        _ => panic!("not full container can't get input"),
    }
}

/// Call the get_type fonction of corresponding struct inside the Container
pub fn get_container_type(cont: &Container) -> ContainerType {
    match cont {
        Container::Split(sp) => sp.get_type(),
        Container::Pane(pa) => ContainerType::Pane,
        _ => ContainerType::Other,
    }
}

/// Call the change_rect fonction of corresponding struct inside the Container
pub fn change_rect_container(rect: &Rect, cont: &mut Container) {
    match cont {
        Container::Split(sp) => {
            sp.change_rect(rect);
        }
        Container::Pane(pa) => {
            pa.change_rect(rect).unwrap(); // TODO: need error handling
        }
        _ => panic!("not full container can't get input"),
    }
}

/// Call the add_child fonction of corresponding struct inside the Container
pub fn add_child_container(
    cont: Container,
    nw_child: Container,
) -> Result<Container, ContainerError> {
    match cont {
        Container::Split(sp) => sp.add_child(nw_child),
        Container::Pane(pa) => pa.add_child(nw_child),
        _ => panic!("this type of container have child"),
    }
}

/// Call the change_focus fonction of corresponding struct inside the Container
pub fn change_focus_container(dir: &MoveDir, cont: &mut Container) {
    match cont {
        Container::Split(sp) => sp.change_focus(dir),
        Container::Pane(pa) => pa.change_focus(dir),
        _ => panic!("this type of container have child"),
    };
}

pub fn container_focus_is_movable(cont: &Container, dir: &MoveDir) -> bool {
    match cont {
        Container::Split(sp) => sp.is_focus_movable(dir),
        Container::Pane(pa) => false,
        _ => panic!("this type of container have child"),
    }
}
