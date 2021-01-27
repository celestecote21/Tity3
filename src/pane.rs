extern crate termion;

use std::thread;
use std::io::{self, Write, Read};
use std::sync::{Arc, RwLock, mpsc};
use std::sync::mpsc::{TryRecvError, Sender, Receiver};
use std::fs::File;
use std::str;
use crate::pty::*;
use crate::buffer_file::*;
use crate::window_notif::WindowNotif;
use crate::size_utilis::*;

#[derive(PartialEq)]
pub struct PaneIdentifier {
    wind_id: u32,
    pane_id: u32,
}

pub struct PaneInfo {
    id: PaneIdentifier,
    rect: Rect,
}

pub struct Pane {
    x: u16,
    y: u16,
    size: Size,
    pub id: PaneIdentifier,
    buffer: Arc<RwLock<StdoutBufferLock>>,
    cursor: Coordinate,
    tty_master_out: File,
    pty_child: Pty,
}

#[derive(Debug)]
pub enum PaneError {
    PaneCreate,
    PaneControl,
    PaneRezise,
}

impl PaneIdentifier {
    pub fn new(wind_id: u32, pane_id: u32) -> PaneIdentifier
    {
        PaneIdentifier{wind_id, pane_id}
    }
    pub fn clone(&self) -> PaneIdentifier
    {
        PaneIdentifier{
            wind_id: self.wind_id,
            pane_id: self.pane_id}
    }
}

impl Pane {
    pub fn new(stdio_master: File,
               notif_wind: Sender<WindowNotif>,
               rect: Rect,
               id: PaneIdentifier) -> Result<Pane, PaneError>
    {
        let size = rect.get_size();
        let pty_handle = match Pty::create("/bin/bash", &size) {
            Err(_) => return Err(PaneError::PaneCreate),
            Ok(pty) => pty,
        };
        let cpy_id = id.clone();
        let res = Pane{
            x: rect.x,
            y: rect.y,
            size,
            buffer: Arc::new(RwLock::new(StdoutBufferLock::new(rect.clone()).unwrap())),
            cursor: Coordinate {x: rect.x, y: rect.y},
            id,
            tty_master_out: stdio_master.try_clone().unwrap(),
            pty_child: pty_handle.clone(),
        };
        let mut pty_handle_in = pty_handle.try_clone().unwrap();
        let out_buffer = res.buffer.clone();
        // output of the pty
        thread::spawn(move || {
            loop {
                let mut packet = [0; 4096];
                let count = match pty_handle_in.read(&mut packet) {
                    Ok(read) => read,
                    _ => break,
                };
                let mut buffer = out_buffer.write().unwrap();

                let read = &packet[..count];
                buffer.write(&read).unwrap();
                drop(buffer);
                notif_wind.send(WindowNotif::Refresh).unwrap();
            }
            notif_wind.send(WindowNotif::SupressPane(cpy_id)).unwrap();
        });
        Ok(res)
    }

    pub fn draw(&mut self) {
        let mut out = &self.tty_master_out;
        let buffer_read = self.buffer.read().unwrap();
        let mut buffer = [0 as u8; 4069];
        self.cursor.y = self.y;
        let mut read_size = (buffer_read).read(&mut buffer[..], &mut self.cursor).unwrap();
        while read_size != 0 {
            write!(out, "{}", str::from_utf8(&buffer[..read_size]).unwrap()).unwrap();
            read_size = buffer_read.read(&mut buffer[..], &mut self.cursor).unwrap();
        }
        self.cursor.y = 0;
    }

    pub fn get_input(&mut self, data: &[u8], size: usize) -> io::Result<()> {
        //match pipe(&mut tty_master_in, &mut pty_handle_out)
        let packet = &data[..size];
        self.tty_master_out.write_all(&packet)?;
        self.tty_master_out.flush()?;
        Ok(())
    }

    // fonction temporaire la methode d'ecriture dans le stout changera
    pub fn clear(&self) {
        let mut out = &self.tty_master_out;
        write!(out, "{}", termion::clear::All).unwrap();
    }
    
    pub fn expand_w(&mut self) -> Result<(), PaneError> {
        self.size.w += 1;
        self.pty_child.resize(&self.size).map_err(|_| PaneError::PaneRezise)
    }

    pub fn expand_h(&mut self) -> Result<(), PaneError> {
        self.size.h += 1;
        self.pty_child.resize(&self.size).map_err(|_| PaneError::PaneRezise)
    }
}
