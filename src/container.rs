use std::io;
use std::sync::mpsc::Sender;
use std::fs::File;
use crate::size_utilis::*;
use crate::split::Split;
use crate::pane::PaneError;
use crate::layout::Direction;

pub type ContainerList = Vec<Box<dyn Container>>;
pub type ContainerTran = (MiniContainer, ContainerType);

pub enum ContainerError {
    BadTransform,
    BadPane(PaneError),
}

pub enum ContainerType {
    SSplit,
    VSplit,
    Pane,
}

pub enum ChildToParent {
    Refresh,
    AddChild(MiniContainer, ContainerType),
    DestroyChild(String),
    GetInputData([u8; 4096], usize),
}

pub trait Container {
    fn new(stdio_master: File, parent_com: Sender<ChildToParent>, rect: Rect, id: String) -> Self
        where Self: Sized;
    fn draw(&self);
    fn get_input(&mut self, data: [u8; 4096], size: usize) -> io::Result<()>;
    fn get_id(&self) -> String;
    fn identifi(&self, id_test: &String) -> bool;
}

pub struct MiniContainer {
    stdio_master: File,
    parent_com_op: Option<Sender<ChildToParent>>,
    rect: Rect,
    id: String
}

impl MiniContainer {
    fn new(stdio_master: File,
           parent_com_op: Option<Sender<ChildToParent>>,
           rect: Rect,
           id: String) -> MiniContainer
    {
        MiniContainer {
            stdio_master,
            parent_com_op,
            rect,
            id,
        }
    }

    pub fn to_split(self,
                    parent_com_op: Option<Sender<ChildToParent>>,
                    rect_op: Option<Rect>,
                    direction: Direction)
        -> Result<Split, ContainerError>
    {
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
        Ok(Split::new_split(self.stdio_master, parent_com, rect, self.id, direction))
    }

    pub fn to_pane(self,
                    parent_com_op: Option<Sender<ChildToParent>>,
                    rect_op: Option<Rect>)
        -> Result<Split, ContainerError>
    {
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
        Ok(Split::new(self.stdio_master, parent_com, rect, self.id))
    }
}
