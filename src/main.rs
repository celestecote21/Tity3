extern crate termion;

use termion::get_tty;
use termion::raw::IntoRawMode;
use std::io::Write;
mod pty;
mod pane;
mod buffer_file;
mod windows;
mod window_notif;
mod keyboard;
use windows::*;

fn main() {
    // Get the standard input stream.
    let mut stdio_master = get_tty().unwrap().into_raw_mode().unwrap();
    let mut window = Window::new(stdio_master.try_clone().unwrap(), 1).unwrap();
    write!(stdio_master, "{}", termion::clear::All).unwrap();
    window.new_pane().unwrap();
    //window.wait();
    keyboard::read_keyboard(&mut window);
    write!(stdio_master, "{}", termion::cursor::Show).unwrap();
}
