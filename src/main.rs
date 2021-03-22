extern crate termion;

use std::io::{Read, Write};
use termion::get_tty;
use termion::raw::IntoRawMode;
mod buffer_file;
mod container;
mod container_action;
mod enum_key;
mod keyboard;
mod layout;
mod pane;
mod pty;
mod size_utilis;
mod split;
mod windows;
use container::*;
use size_utilis::Rect;
use windows::*;

fn main() {
    // Get the standard input stream.
    let mut stdio_master = get_tty().unwrap().into_raw_mode().unwrap();
    let base_rect = Rect::from_tupple(termion::terminal_size().unwrap());
    write!(stdio_master, "{}", termion::clear::All).unwrap();
    let (wind_com, wind_thread) = match start_wind(
        stdio_master.try_clone().unwrap(),
        base_rect,
        "0".to_string(),
    ) {
        Ok(a) => a,
        _ => panic!("can't open the windows"),
    };
    let mut stdio_clone = stdio_master.try_clone().unwrap();
    loop {
        let mut packet = [0; 4096];
        let count = match stdio_clone.read(&mut packet) {
            Ok(c) => c,
            Err(_) => break,
        };
        match wind_com.send(ChildToParent::GetInputData(packet, count)) {
            Ok(_) => (),
            Err(_) => break,
        }
    }
    //window.new_pane().unwrap();
    //window.wait();
    //keyboard::read_keyboard(&mut window);
    write!(stdio_master, "{}", termion::cursor::Show).unwrap();
}
