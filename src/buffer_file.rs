extern crate byte_slice_cast;

use std::io;
use std::ptr;
use std::ops;
use crate::size_utilis::*;

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

    fn deref(&self) -> &StdoutBuffer
    {
        &self.stdoutBuffer
    }
}

impl ops::DerefMut for StdoutBufferLock {
    fn deref_mut(&mut self) -> &mut StdoutBuffer
    {
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
        Ok(StdoutBuffer{
            line_list: Vec::new(),
            line_end: false,
            pane_rect,
            last_y: 0,
        })
    }
    /// will be a lot bigger because this need to test if the line is not to big
    /// and it need also to handle all the ainsi sequence
    pub fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut i = 0;
        let mut string_building = None;
        if self.line_end == false {
            string_building = self.line_list.pop();
        }
        let mut string_building = match string_building {
            Some(str)   => str,
            None        => String::new(),
        };
        for c in buf.iter() {
            i += 1;
            if *c as char == '\n' {
                //self.line_end = true;
                self.line_list.push(string_building);
                string_building = String::new();
                continue;
            }
            string_building.push(*c as char);
        }
        self.line_list.push(string_building);
        self.last_y = (self.line_list.len() - 1) as u16;
        Ok(i)
    }

    pub fn read(&self, buf: &mut [u8], cursor: &mut Coordinate) -> io::Result<usize> {
        if cursor.y > self.pane_rect.h || cursor.y > self.line_list.len() as u16 {
            return Ok(0);
        }
        let buff_slice = match self.line_list.get((cursor.y) as usize) {
            Some(line_str) => line_str,
            None => "",
        };
        //print!("{}", buff_slice);
        cursor.y += 1;
        if buff_slice.len() <= 0 {
            return Ok(0);
        }
        let mut cursor_tmp = cursor.clone();
        /*if self.line_end == false {
            cursor_tmp.x += buff_slice.len() as u16;
            cursor.x += buff_slice.len() as u16;
        }*/
        let mut line = cursor.goto_string();
        line.push_str(buff_slice);
        unsafe {
            let dst_ptr = buf.as_mut_ptr();
            let src_ptr = line.as_ptr();

            ptr::copy_nonoverlapping(src_ptr, dst_ptr, line.len());

        }
        Ok(line.len())
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use std::str;

    #[test]
    fn test_write() {
        let mut stout_buff = StdoutBuffer::new(Rect{x: 0, y: 0, w:  128, h: 64}).unwrap();
        let packet = "test test test test".as_bytes();

        assert_eq!(stout_buff.write(packet).unwrap(), packet.len())
    }

    #[test]
    fn test_write_read() {
        let rect = Rect{x: 0, y: 0, w:  128, h: 64};
        let mut cursor = rect.get_origine();
        let mut stout_buff = StdoutBuffer::new(rect).unwrap();
        let packet = "test test test test".as_bytes();
        let result = format!("{}test test test test", termion::cursor::Goto(1, 1));
        let mut fun_res = [0; 4096];

        assert_eq!(stout_buff.write(packet).unwrap(), packet.len());
        let read_size = stout_buff.read(&mut fun_res, &mut cursor).unwrap();
        assert_eq!(read_size, result.len());
        assert_eq!(unsafe {str::from_utf8_unchecked(&fun_res[..read_size])}, result);
    }
}
