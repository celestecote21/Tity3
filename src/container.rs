use crate::layout::Direction;
use crate::pane::*;
use crate::size_utilis::*;
use crate::split::Split;
use std::fs::File;
use std::sync::mpsc::Sender;

pub type ContainerList = Vec<Container>;
pub type ContainerTran = (MiniContainer, ContainerType);

pub enum ContainerError {
    BadTransform,
    BadPane(PaneError),
}

pub enum ContainerType {
    SSplit,
    VSplit,
    Pane,
    Window,
}

pub enum ChildToParent {
    Refresh,
    AddChild(Container),
    DestroyChild(String),
    GetInputData([u8; 4096], usize),
}

pub enum Container {
    Split(Split),
    Pane(Pane),
    MiniCont(MiniContainer),
}

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

pub fn get_input_container(data: [u8; 4096], size: usize, cont: &mut Container) {
    match cont {
        Container::Split(sp) => {
            sp.get_input(data, size);
        }
        Container::Pane(pa) => {
            pa.get_input(data, size);
        }
        _ => panic!("not ful container can't get input"),
    }
}

pub struct MiniContainer {
    stdio_master: File,
    parent_com_op: Option<Sender<ChildToParent>>,
    rect: Rect,
    id: String,
    to_container: ContainerType,
}

impl MiniContainer {
    pub fn new(
        stdio_master: File,
        parent_com_op: Option<Sender<ChildToParent>>,
        rect: Rect,
        id: String,
        to_container: ContainerType,
    ) -> MiniContainer {
        MiniContainer {
            stdio_master,
            parent_com_op,
            rect,
            id,
            to_container,
        }
    }

    pub fn complet(
        self,
        parent_com_op: Option<Sender<ChildToParent>>,
        rect_op: Option<Rect>,
    ) -> Result<Container, ContainerError> {
        if parent_com_op.is_none() && self.parent_com_op.is_none() {
            return Err(ContainerError::BadTransform);
        }
        let parent_com = match self.parent_com_op {
            None => parent_com_op.unwrap(),
            Some(com) => match parent_com_op {
                Some(com_apr) => com_apr,
                None => com,
            },
        };
        let rect = match rect_op {
            Some(re) => re,
            None => self.rect,
        };
        match self.to_container {
            ContainerType::Pane => Ok(Container::Pane(Pane::new(
                self.stdio_master,
                parent_com,
                rect,
                self.id,
            )?)),
            ContainerType::SSplit => Ok(Container::Split(Split::new(
                self.stdio_master,
                parent_com,
                rect,
                self.id,
                Direction::Horizontal,
                None,
            )?)),
            ContainerType::SSplit => Ok(Container::Split(Split::new(
                self.stdio_master,
                parent_com,
                rect,
                self.id,
                Direction::Vertical,
                None,
            )?)),
            _ => return Err(ContainerError::BadTransform),
        }
    }
}
