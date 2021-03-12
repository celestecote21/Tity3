use crate::layout::*;
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
    MoveFocus(MoveDir),
}

pub enum Container {
    Split(Split),
    Pane(Pane),
    MiniCont(MiniContainer),
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
