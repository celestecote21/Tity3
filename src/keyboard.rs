use crate::container::*;
use std::fs::File;
use std::io::Read;
use std::mem;
use std::process::Command;
use std::sync::mpsc::Sender;

pub fn parse_input(
    data: [u8; 4096],
    size: usize,
    command_sender: &Sender<ChildToParent>,
) -> ([u8; 4096], usize) {
    for i in 0..size {
        println!("{}, ", data[i]);
    }
    (data, size)
}

#[repr(C)]
pub struct InputEvent {
    tv_sec: isize,  // from timeval struct
    tv_usec: isize, // from timeval struct
    pub type_: u16,
    pub code: u16,
    pub value: i32,
}

pub fn read_keyboard(window: &mut Container) {
    let mut device_file = File::open(get_keyboard_device_filename()).unwrap();
    let mut buff: [u8; mem::size_of::<InputEvent>()] = [0; mem::size_of::<InputEvent>()];
    let meta_code = 125;
    let mut meta_press = false;
    loop {
        let num_read = device_file.read(&mut buff).unwrap(); // read the event
        if num_read != mem::size_of::<InputEvent>() {
            panic!("error reading the event");
        }
        let event: InputEvent = unsafe { mem::transmute(buff) }; // copi to the struct
        if event.type_ == 1 {
            if event.value == 1 {
                if event.code == 1 {
                    break;
                }
                if event.code == meta_code {
                    meta_press = true;
                } else if meta_press == true {
                    handle_action(event.code, window);
                }
            } else if event.value == 0 && event.code == meta_code {
                meta_press = false
            }
        }
    }
}

fn handle_action(code: u16, window: &mut Container) {
    if code == 21 {
        //window.new_pane().unwrap();
    }
}

fn get_keyboard_device_filename() -> String {
    let mut command_str = "grep -E 'Handlers|EV' /proc/bus/input/devices".to_string();
    command_str.push_str("| grep -B1 120013");
    command_str.push_str("| grep -Eo event[0-9]+");

    let res = Command::new("sh")
        .arg("-c")
        .arg(command_str)
        .output()
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });
    let res_str = std::str::from_utf8(&res.stdout).unwrap();

    let mut filenames = Vec::new();
    for file in res_str.trim().split('\n') {
        let mut filename = "/dev/input/".to_string();
        filename.push_str(file);
        filenames.push(filename);
    }
    filenames.swap_remove(0)
}
