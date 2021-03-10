#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

extern crate byte_slice_cast;

use crate::size_utilis::*;
use std::io;
use std::ops;
use std::ptr;

pub struct StdoutBufferLock {
    stdoutBuffer: StdoutBuffer,
}

impl StdoutBufferLock {
    pub fn new(pane_rect: Rect) -> Result<StdoutBufferLock, io::Result<()>> {
        Ok(StdoutBufferLock {
            stdoutBuffer: StdoutBuffer::new(pane_rect)?,
        })
    }
}

impl ops::Deref for StdoutBufferLock {
    type Target = StdoutBuffer;

    fn deref(&self) -> &StdoutBuffer {
        &self.stdoutBuffer
    }
}

impl ops::DerefMut for StdoutBufferLock {
    fn deref_mut(&mut self) -> &mut StdoutBuffer {
        &mut self.stdoutBuffer
    }
}

/// each pane will have a StdoutBuffer:
/// the thread containing the process will send all the stdout here
/// and this wil process the data to form a list of line
/// the line will not be greater than the size of the pane
/// so the StdoutBuffer line buffer need to be update if the pane size change
pub struct StdoutBuffer {
    line_list: Vec<String>,
    line_end: bool,
    pane_rect: Rect,
    last_y: u16,
}

impl StdoutBuffer {
    pub fn new(pane_rect: Rect) -> Result<StdoutBuffer, io::Result<()>> {
        Ok(StdoutBuffer {
            line_list: Vec::new(),
            line_end: false,
            pane_rect,
            last_y: 0,
        })
    }

    /// will be a lot bigger because this need to test if the line is not to big
    /// and it need also to handle all the ainsi sequence
    pub fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut string_building = None;
        if self.line_end == false {
            string_building = self.line_list.pop();
        }
        let mut string_building = match string_building {
            Some(str) => str,
            None => String::new(),
        };
        let buf_len = buf.len();
        let mut i = 0;
        while i < buf_len {
            if (i + 1 < buf_len && buf[i] as char == '\r' && buf[i + 1] as char == '\n') || buf[i] as char == '\n' {
                self.line_list.push(string_building);
                string_building = String::new();
                if buf[i] as char == '\r' {
                    string_building.push_str("mmm");
                    i += 1;
                }
                i += 1;
                continue;
            }
            string_building.push(buf[i] as char);
            if buf[i] as char == '\r' {
                string_building.clear();
            }
            i += 1;
        }
        self.line_list.push(string_building);
        self.last_y = (self.line_list.len() - 1) as u16;
        Ok(buf_len)
    }

    pub fn change_rect(&mut self, rect: &Rect) {
        self.pane_rect.copy(rect);
        //TODO: Rezise string inside the Vec
    }

    pub fn read(&self, buf: &mut [u8], cursor: &mut Coordinate) -> usize {
        let mut line = match self.line_list.get(cursor.y as usize) {
            Some(s) => s.to_string(),
            None => return 0,
        };
        if cursor.x != 0 {
            line.drain(..cursor.x as usize);
        }
        let window_cursor = Coordinate {
            x: cursor.x + self.pane_rect.x,
            y: cursor.y + self.pane_rect.y,
        };
        let test_str = format!("{} {}", window_cursor.x, window_cursor.y);
        line.insert_str(0, &test_str);
        line.insert_str(0, &window_cursor.goto_string());
        if line.len() > 4096 {
            line.drain(4096..);
            cursor.x += 4096;
        } else {
            cursor.x = 0;
        }
        cursor.y += 1;
        unsafe {
            let dst_ptr = buf.as_mut_ptr();
            let src_ptr = line.as_ptr();
            ptr::copy_nonoverlapping(src_ptr, dst_ptr, line.len());
        }
        line.len()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use std::str;

    #[test]
    fn test_write() {
        let mut stout_buff = StdoutBuffer::new(Rect {
            x: 0,
            y: 0,
            w: 128,
            h: 64,
        })
        .unwrap();
        let packet = "test test test test".as_bytes();

        assert_eq!(stout_buff.write(packet).unwrap(), packet.len())
    }

    #[test]
    fn test_write_read() {
        let rect = Rect {
            x: 0,
            y: 0,
            w: 128,
            h: 64,
        };
        let mut cursor = rect.get_origine();
        let mut stout_buff = StdoutBuffer::new(rect).unwrap();
        let packet = "test test test test".as_bytes();
        let result = format!("{}test test test test", termion::cursor::Goto(1, 1));
        let mut fun_res = [0; 4096];

        assert_eq!(stout_buff.write(packet).unwrap(), packet.len());
        let read_size = stout_buff.read(&mut fun_res, &mut cursor).unwrap();
        assert_eq!(read_size, result.len());
        assert_eq!(
            unsafe { str::from_utf8_unchecked(&fun_res[..read_size]) },
            result
        );
    }
}
