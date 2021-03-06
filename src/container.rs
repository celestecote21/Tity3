use crate::layout::Direction;
use crate::pane::*;
use crate::size_utilis::*;
use crate::split::Split;
use std::fs::File;
use std::sync::mpsc::Sender;

pub type ContainerList = Vec<Container>;

#[derive(Debug)]
pub enum ContainerError {
    BadTransform,
    CreationError,
    BadPane(PaneError),
}

#[derive(PartialEq)]
pub enum ContainerType {
    SSplit,
    VSplit,
    Pane,
    Window,
    Other,
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

    pub fn duplic(&self, to_container: ContainerType) -> Result<MiniContainer, ContainerError> {
        let stdio_clone = match self.stdio_master.try_clone() {
            Ok(f) => f,
            Err(_) => return Err(ContainerError::BadTransform),
        };
        Ok(MiniContainer {
            stdio_master: stdio_clone,
            parent_com_op: self.parent_com_op.clone(),
            rect: self.rect.clone(),
            id: self.id.clone(),
            to_container,
        })
    }

    pub fn complet(
        self,
        parent_com_op: Option<Sender<ChildToParent>>,
        rect_op: Option<Rect>,
        id_op: Option<String>,
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
        let id = match id_op {
            Some(i) => i,
            None => self.id,
        };
        match self.to_container {
            ContainerType::Pane => Ok(Container::Pane(Pane::new(
                self.stdio_master,
                parent_com,
                rect,
                id,
            )?)),
            ContainerType::SSplit => Ok(Container::Split(Split::new(
                self.stdio_master,
                parent_com,
                rect,
                id,
                Direction::Horizontal,
                None,
            )?)),
            ContainerType::VSplit => Ok(Container::Split(Split::new(
                self.stdio_master,
                parent_com,
                rect,
                id,
                Direction::Vertical,
                None,
            )?)),
            _ => return Err(ContainerError::BadTransform),
        }
    }
}
