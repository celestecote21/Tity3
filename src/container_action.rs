use crate::container::*;
use crate::size_utilis::*;
use crate::layout::*;

pub fn draw_container(cont: &mut Container) {
    match cont {
        Container::Split(sp) => {
            sp.draw();
        }
        Container::Pane(pa) => {
            pa.draw();
        }
        _ => panic!("not ful container can't be drawn"),
    }
}

pub fn get_id_container(cont: &Container) -> String {
    match cont {
        Container::Split(sp) => sp.get_id(),
        Container::Pane(pa) => pa.get_id(),
        _ => panic!("can't get this id"),
    }
}

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

pub fn get_container_type(cont: &Container) -> ContainerType {
    match cont {
        Container::Split(sp) => ContainerType::SSplit,
        Container::Pane(pa) => ContainerType::Pane,
        _ => ContainerType::Other,
    }
}

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

pub fn change_focus_container(dir: &MoveDir, cont: &mut Container)
{
    match cont {
        Container::Split(sp) => sp.change_focus(dir),
        Container::Pane(pa) => pa.change_focus(dir),
        _ => panic!("this type of container have child"),
    };
}
