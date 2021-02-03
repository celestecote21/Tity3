use std::io;
use std::any::Any;
use std::sync::mpsc::Sender;
use std::fs::File;
use crate::size_utilis::*;
use crate::split::Split;
use crate::pane::*;
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
    Window,
}

pub enum ContainerMover {
    SplitCont(Split),
    PaneCont(Pane),
}

pub enum ChildToParent {
    Refresh,
    AddChild(MiniContainer, ContainerType),
    DestroyChild(String),
    GetInputData([u8; 4096], usize),
}

pub trait Container {
    fn new(stdio_master: File,
           parent_com: Sender<ChildToParent>,
           rect: Rect,
           id: String)
    -> Result<Self, ContainerError>
        where Self: Sized;
    fn draw(&self);
    fn get_input(&mut self, data: [u8; 4096], size: usize) -> io::Result<()>;
    fn add_child(self, cont: MiniContainer, cont_type: ContainerType)
        -> Result<Box<dyn Container>, ContainerError>;
    fn get_id(&self) -> String;
    fn get_type(&self) -> ContainerType;
    fn identifi(&self, id_test: &String) -> bool;
    fn is_leaf(&self) ->bool;
    fn as_any(&self) -> &dyn Any;
    fn as_pane(self) -> Result<Pane, ContainerError>;
    fn to_mini_container(&self) -> MiniContainer;
}

pub struct MiniContainer {
    stdio_master: File,
    parent_com_op: Option<Sender<ChildToParent>>,
    rect: Rect,
    id: String
}

impl MiniContainer {
    pub fn new(stdio_master: File,
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
                    direction: Direction,
                    may_be_child: Option<ContainerMover>)
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
        if may_be_child.is_some() {
            Ok(Split::new_split_with_child(self.stdio_master,
                                           parent_com,
                                           rect,
                                           self.id,
                                           direction,
                                           may_be_child.unwrap())?)
        } else {
            Ok(Split::new_split(self.stdio_master,
                                parent_com,
                                rect,
                                self.id,
                                direction)?)
        }
    }

    pub fn to_pane(self,
                    parent_com_op: Option<Sender<ChildToParent>>,
                    rect_op: Option<Rect>)
        -> Result<Pane, ContainerError>
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
        Ok(Pane::new(self.stdio_master, parent_com, rect, self.id)?)
    }
}
