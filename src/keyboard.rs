use crate::windows::WindowsConf;

pub fn parse_input(data: [u8; 4096], size: usize, config: &WindowsConf) -> ([u8; 4096], usize) {
    let mut i = 0;
    loop {
        if i >= size {
            break;
        }
        if data[i] == 27 && parse_command(data[i + 1], config) {
            // TODO: if there is data after the keycode this will not take it in account
            return (data, 0);
        }
        i += 1;
    }
    (data, size)
}

pub fn parse_command(keycode: u8, config: &WindowsConf) -> bool {
    //println!("{}", keycode);
    let pos = match config
        .get_keymap()
        .iter()
        .position(|map| map.keycode == keycode)
    {
        Some(p) => p,
        None => return false,
    };
    config.get_keymap()[pos].take_action(config);
    true
}
