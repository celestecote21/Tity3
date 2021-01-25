extern crate termion;

use std::thread;
use std::io::{Write, Read};
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
    tx_input_control: Sender<bool>,
    rx_draw_output: Receiver<bool>,
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
        let (tx_input_control, rx_input_control) = mpsc::channel();
        let (tx_draw_output, rx_draw_output) = mpsc::channel();
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
            tx_input_control,
            rx_draw_output,
        };
        let mut pty_handle_in = pty_handle.try_clone().unwrap();
        let mut pty_handle_out = pty_handle.try_clone().unwrap();
        let mut tty_master_in = stdio_master.try_clone().unwrap();
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
                match tx_draw_output.send(true) {
                    Err(_) => break,
                    _ => ()
                };
                drop(buffer);
                notif_wind.send(WindowNotif::Refresh).unwrap();
            }
            notif_wind.send(WindowNotif::SupressPane(cpy_id)).unwrap();
        });
        // input of the pty
        thread::spawn(move || {
            let mut have_focus = true;
            loop {
                if have_focus {
                    match rx_input_control.try_recv() {
                        Ok(true) => (),
                        Ok(false) => {
                            have_focus = false;
                            continue;
                        }
                        Err(TryRecvError::Empty) => (),
                        _ => return,
                    }
                    match pipe(&mut tty_master_in, &mut pty_handle_out) {
                        Err(_) => return,
                        _ => (),
                    }
                } else { // if has not the focus block until recive the focus
                    have_focus = rx_input_control.recv().unwrap();
                }
            }
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

    pub fn take_input(&self) -> Result<(), PaneError> {
        self.tx_input_control.send(true).map_err(|_| PaneError::PaneControl)
    }

    pub fn drop_input(&self) -> Result<(), PaneError> {
        self.tx_input_control.send(false).map_err(|_| PaneError::PaneControl)
    }
}
