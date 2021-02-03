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
mod size_utilis;
mod container;
mod split;
mod layout;
mod transformation;
use windows::*;
use size_utilis::Rect;

fn main() {

    // Get the standard input stream.
    let mut stdio_master = get_tty().unwrap().into_raw_mode().unwrap();
    let base_rect = Rect::from_tupple(termion::terminal_size().unwrap());
    write!(stdio_master, "{}", termion::clear::All).unwrap();
    start_wind(stdio_master.try_clone().unwrap(), base_rect, String::from("0"));
    //window.new_pane().unwrap();
    //window.wait();
    //keyboard::read_keyboard(&mut window);
    write!(stdio_master, "{}", termion::cursor::Show).unwrap();
}
