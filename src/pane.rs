extern crate termion;

use std::thread;
use std::io::{self, Write, Read};
use std::sync::{Arc, RwLock};
use std::sync::mpsc::Sender;
use std::fs::File;
use std::any::Any;
use crate::pty::*;
use crate::buffer_file::*;
use crate::size_utilis::*;
use crate::container::*;
use crate::split::*;
use crate::layout::*;

#[derive(PartialEq)]
pub struct PaneIdentifier {
    wind_id: u32,
    pane_id: u32,
}

#[derive(Debug)]
pub enum PaneError {
    PaneCreate,
    PaneControl,
    PaneRezise,
}

pub struct Pane {
    id: String,
    stdio_master: File,
    pty_input: Pty,
    parent_com: Sender<ChildToParent>,
    rect: Rect,
    buffer: Arc<RwLock<StdoutBufferLock>>,
    cursor: Coordinate,
}

impl Pane {
    pub fn new(stdio_master: File,
           parent_com: Sender<ChildToParent>,
           rect: Rect,
           id: String)
        -> Result<Pane, ContainerError>
    {
        let size = rect.get_size();
        let pty_handle = match Pty::create("/bin/bash", &size) {
            Err(_) => return Err(ContainerError::BadPane(PaneError::PaneCreate)),
            Ok(pty) => pty,
        };
        let mut pty_io_clone = pty_handle.clone();
        let out_buffer = Arc::new(RwLock::new(StdoutBufferLock::new(rect.clone()).unwrap()));
        let out_buffer_clone = out_buffer.clone();
        let parent_com_clone = parent_com.clone();
        let cpy_id = id.clone();
        let cursor = Coordinate {x: rect.x, y: rect.y};
        thread::spawn(move || {
            loop {
                let mut packet = [0; 4096];
                let count = match pty_io_clone.read(&mut packet) {
                    Ok(read) => read,
                    _ => break,
                };
                let mut buffer = out_buffer.write().unwrap();

                let read = &packet[..count];
                buffer.write(&read).unwrap();
                drop(buffer);
                parent_com_clone.send(ChildToParent::Refresh).unwrap();
            }
            parent_com_clone.send(ChildToParent::DestroyChild(cpy_id)).unwrap();
        });
        Ok(Pane{
            id,
            stdio_master,
            pty_input: pty_handle,
            parent_com,
            rect,
            buffer: out_buffer_clone,
            cursor
        })
    }

    pub fn draw(&self)
    {
        unimplemented!()
    }

    /// because it's a pane the data go directly to the pseudo terminal
    pub fn get_input(&mut self, data: [u8; 4096], size: usize) -> io::Result<()>
    {
        let packet = &data[..size];
        self.pty_input.write_all(packet)?;
        self.pty_input.flush()?;
        Ok(())
    }

    pub fn add_child(self, cont: Container)
        -> Result<Container, ContainerError>
    {
        let nw_cont = Split::new(self.stdio_master.try_clone().unwrap(),
                                           self.parent_com.clone(),
                                           self.rect.clone(),
                                           self.id.clone(),
                                           Direction::Horizontal,
                                           Some(Container::Pane(self)))?;

        //Ok(Container::SplitCont(nw_cont));
        todo!()
    }

    pub fn get_id(&self) -> String
    {
        self.id.clone()
    }

    pub fn get_type(&self) -> ContainerType
    {
        ContainerType::Pane
    }

    pub fn identifi(&self, id_test: &String) -> bool
    {
        self.id.eq(id_test)
    }

    /// a pane is always a leaf because it doesn't have any child
    pub fn is_leaf(&self) -> bool
    {
        true
    }

    pub fn as_pane(self) -> Result<Pane, ContainerError>
    {
        Ok(self)
    }

    pub fn as_any(&self) -> &dyn Any {
        self
    }

    /*fn to_mini_container(&self)
        -> MiniContainer
    {
        MiniContainer::new(
            self.stdio_master.try_clone().unwrap(),
            Some(self.parent_com.clone()),
            self.rect.clone(),
            self.id.clone())
    }*/

    pub fn expand_w(&mut self) -> Result<(), PaneError>
    {
        //TODO: need to chenge in the bufferout too
        self.rect.w+= 1;
        self.pty_input.resize(&self.rect.get_size()).map_err(|_| PaneError::PaneRezise)
    }

    pub fn expand_h(&mut self) -> Result<(), PaneError>
    {
        //TODO: need to chenge in the bufferout too
        self.rect.h += 1;
        self.pty_input.resize(&self.rect.get_size()).map_err(|_| PaneError::PaneRezise)
    }
}
