extern crate termion;

use crate::buffer_file::*;
use crate::container::*;
use crate::layout::*;
use crate::pty::*;
use crate::size_utilis::*;
use crate::split::*;
use std::fs::File;
use std::io::{self, Read, Write};
use std::str;
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, RwLock};
use std::thread::{self, JoinHandle};

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
    chang_name: Sender<String>,
    rect: Rect,
    buffer: Arc<RwLock<StdoutBufferLock>>,
    cursor: Coordinate,
    y: u16,
}

impl Pane {
    pub fn new(
        stdio_master: File,
        parent_com: Sender<ChildToParent>,
        rect: Rect,
        id: String,
    ) -> Result<Pane, ContainerError> {
        let size = rect.get_size();
        let pty_handle = match Pty::create("/bin/bash", &size) {
            Err(_) => return Err(ContainerError::BadPane(PaneError::PaneCreate)),
            Ok(pty) => pty,
        };
        let mut pty_io_clone = pty_handle.clone();
        let out_buffer = Arc::new(RwLock::new(StdoutBufferLock::new(rect.clone()).unwrap()));
        let out_buffer_clone = out_buffer.clone();
        let parent_com_clone = parent_com.clone();
        let (chang_name, chang_name_rec) = mpsc::channel();
        let mut cpy_id = id.clone();
        thread::spawn(move || {
            loop {
                let mut packet = [0; 4096];
                let count = match pty_io_clone.read(&mut packet) {
                    Ok(read) => read,
                    _ => break,
                };
                match chang_name_rec.try_recv() {
                    Ok(new_name) => cpy_id = new_name,
                    _ => (),
                }
                let mut buffer = out_buffer.write().unwrap();

                let read = &packet[..count];
                buffer.write(&read).unwrap();
                drop(buffer);
                match parent_com_clone.send(ChildToParent::Refresh(cpy_id.clone())) {
                    Ok(_) => (),
                    _ => break,
                }
            }
            match parent_com_clone.send(ChildToParent::DestroyChild(cpy_id)) {
                _ => (),
            }
        });
        Ok(Pane {
            id,
            stdio_master,
            pty_input: pty_handle,
            parent_com,
            chang_name,
            rect,
            buffer: out_buffer_clone,
            cursor: Coordinate { x: 0, y: 0 },
            y: 0,
        })
    }

    pub fn draw(&mut self, id: &str) {
        /*if self.id != id {
            return;
        }*/
        let mut out = &self.stdio_master;
        let buffer_read = self.buffer.read().unwrap();
        let mut line_buf = [0 as u8; 4069];
        let mut cursor = Coordinate { x: 0, y: 0 };
        let mut read_size = buffer_read.read(&mut line_buf[..], &mut cursor);
        while read_size != 0 {
            write!(out, "{}", str::from_utf8(&line_buf[..read_size]).unwrap()).unwrap();
            read_size = buffer_read.read(&mut line_buf[..], &mut cursor);
        }
    }

    /// because it's a pane the data go directly to the pseudo terminal
    pub fn get_input(&mut self, data: [u8; 4096], size: usize) -> io::Result<()> {
        // TODO: test if it contain an enter to put the buffer_file in a new line
        let packet = &data[..size];
        self.pty_input.write_all(packet)?;
        self.pty_input.flush()?;
        Ok(())
    }

    pub fn add_child(mut self, cont: Container) -> Result<Container, ContainerError> {
        let id_split = self.id.clone();
        self.id.push('0');
        self.chang_name.send(self.id.clone()).unwrap();
        let nw_cont = Split::new(
            self.parent_com.clone(),
            self.rect.clone(),
            id_split,
            Direction::Horizontal,
            Some(Container::Pane(self)),
        )?;
        Ok(nw_cont.add_child(cont)?)
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn identifi(&self, id_test: &str) -> bool {
        self.id.eq(id_test)
    }

    fn clean_rect(&mut self) {
        let mut line = String::new();
        for _ in 0..(self.rect.w - 2) {
            line.push(' ');
        }
        let mut out = &self.stdio_master;
        // TODO: error handling
        for i in 0..self.rect.h {
            write!(
                out,
                "{}{}",
                termion::cursor::Goto(self.rect.x + 1, self.rect.y + i + 1),
                line
            )
            .unwrap();
        }
    }

    pub fn change_rect(&mut self, rect: &Rect) -> Result<(), PaneError> {
        let id = self.id.clone();
        self.clean_rect();
        // TODO: need to tell to the bufferfile and error handling
        self.buffer.write().unwrap().change_rect(rect);
        self.y = 0;
        self.rect.copy(rect);
        self.pty_input
            .resize(&self.rect.get_size())
            .map_err(|_| PaneError::PaneRezise)?;
        self.draw(&id);
        Ok(())
    }

    pub fn destroy(&mut self, id: &str) -> Result<(), ()> {
        if id != self.id && id != "-2" && id != "-1" {
            return Err(());
        }
        self.clean_rect();
        Ok(())
    }
    pub fn change_focus(&self, _dire: &MoveDir) {}
}
