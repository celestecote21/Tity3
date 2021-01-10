use std::thread;
use std::fs::File;
use std::sync::mpsc::{self, Sender, Receiver};
use crate::pane::*;
use crate::window_notif::*;

#[derive(Debug)]
pub enum WindowError {
    WindowOpen,
    WindowNewPane,
}

pub struct Window {
    next_pane_id: u32,
    wind_id: u32,
    tx_notif_channel: Sender<WindowNotif>,
    stdio_master: File,
    panes_handle: thread::JoinHandle<()>
}

impl Window {
    pub fn new(stdio_master: File, wind_id: u32) -> Result<Window, WindowError>
    {
        let (tx_notif_channel, rx_notif_channel) = mpsc::channel();
        let panes_handle = thread::spawn( move || {
            handle_panes(rx_notif_channel);
        });
        let res = Window {
            next_pane_id: 1,
            wind_id,
            tx_notif_channel,
            stdio_master,
            panes_handle,
        };
        Ok(res)
    }

    pub fn new_pane(&mut self) -> Result<(), WindowError>
    {
        let stdio_master = match self.stdio_master.try_clone() {
            Ok(file) => file,
            _ => return Err(WindowError::WindowNewPane)
        };
        let rect = Rect::new((self.next_pane_id - 1) as u16 * 64, 1, 64, 32);
        let paneid = PaneIdentifier::new(self.wind_id, self.next_pane_id);
        let pane = match Pane::new(
            stdio_master,
            self.tx_notif_channel.clone(),
            rect,
            paneid) {
            Ok(pane) => pane,
            _ => return Err(WindowError::WindowNewPane),
        };
        self.next_pane_id += 1;
        self.tx_notif_channel.send(WindowNotif::AddPane(pane))
            .map_err(|_| WindowError::WindowNewPane)
            .and_then(|_| {
                Ok(())
            })
    }
    pub fn wait(self)
    {
        self.panes_handle.join().unwrap();
    }
}

pub struct WindowsInterne {
    index_focus: Option<usize>,
}

impl WindowsInterne {
    pub fn new() -> WindowsInterne
    {
        WindowsInterne {
            index_focus: None,
        }
    }
    pub fn get_focused(&self) -> Option<usize>
    {
        self.index_focus
    }
    pub fn set_focused(&mut self, index: usize)
    {
        self.index_focus = Some(index);
    }
}

fn handle_panes(rx_receive_notif: Receiver<WindowNotif>)
{
    let mut list_pane = Vec::new();
    let mut internal_wind = WindowsInterne::new();
    loop {
        match rx_receive_notif.recv() {
            Ok (notif) => handle_windows_notif(notif, &mut list_pane, &mut internal_wind),
            _ => break
        }
        if list_pane.is_empty() {
            break
        }
    }
}
