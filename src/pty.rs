use std::fs::File;
use std::os::unix::io::{FromRawFd, RawFd};
use std::ptr;
use std::io::{self, Write, Read};
use std::process::{Command, Stdio};
use std::os::unix::process::CommandExt;
use std::ops;
use libc;

/*
 * handleling the pty (pseudoterminal) gestion here
 * pty is a pair of two virtual character device (master slave) connected
 * everithing send to master while be receive to the slave and vice versa
 * the master is on the side of the programme that start the process
 */
pub struct Pty {
    fd_master: RawFd,
    file: File,
}

#[derive(Debug)]
pub enum PtyError {
    PtyOpen,
    PtySpawn,
    Resize,
}

impl Pty {
    pub fn create(cmd: &str, size: &Size) -> Result<Pty, PtyError>
    {
        // open a pseudoterminal, and get the pair to "comunicate"
        let (master, slave) = openpty(size)?;
        // start a new process change it's std I/O and before spawning start a new session to take
        // the controlle on tty
        unsafe {
            Command::new(cmd)
                .stdin(Stdio::from_raw_fd(slave))
                .stdout(Stdio::from_raw_fd(slave))
                .stderr(Stdio::from_raw_fd(slave))
                .pre_exec(take_controlle)
                .spawn()
                .map_err(|_| PtyError::PtySpawn)
                .and_then(|_| {
                    let pty = Pty{
                        fd_master: master,
                        file: File::from_raw_fd(master),
                    };
                    pty.resize(&size)?;
                    Ok(pty)
                })
        }
    }
    pub fn resize(&self, size: &Size) -> Result<(), PtyError> {
        unsafe {
            libc::ioctl(self.fd_master, libc::TIOCSWINSZ, &size.to_c_size())
                .to_result()
                .map(|_| ())
                .map_err(|_| PtyError::Resize)
        }
    }
    pub fn clone(&self) -> Pty {
        Pty {
            fd_master: self.fd_master,
            file: self.file.try_clone().unwrap(),
        }
    }
}

impl Read for Pty {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>
    {
        self.file.read(buf)
    }
}

impl Write for Pty {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize>
    {
        self.file.write(buf)
    }
    fn flush(&mut self) ->io::Result<()>
    {
        self.file.flush()
    }
}

impl ops::Deref for Pty {
    type Target = File;

    fn deref(&self) -> &File
    {
        &self.file
    }
}

impl ops::DerefMut for Pty {
    fn deref_mut(&mut self) -> &mut File
    {
        &mut self.file
    }
}

fn take_controlle() -> io::Result<()>
{
  unsafe {
      //create a new sesion to have a new group of process
        libc::setsid()
            .to_result()
            .map_err(|_| io::Error::new(io::ErrorKind::Other, ""))?;
        //tell the process to take the control on tty
        libc::ioctl(0, libc::TIOCSCTTY, 1)
            .to_result()
            .map_err(|_| io::Error::new(io::ErrorKind::Other, ""))?;
    }
    Ok(())
}

fn openpty(size: &Size) -> Result<(RawFd, RawFd), PtyError>
{
    let mut masterfd = 0;
    let mut slavefd = 0;

    unsafe {
        match libc::openpty(
            &mut masterfd,
            &mut slavefd,
            ptr::null_mut(),
            ptr::null(),
            &size.to_c_size())
            .to_result() {
                Err(_) => return Err(PtyError::PtyOpen),
                _ => ()
            }
    }
    Ok((masterfd, slavefd))
}

pub struct Size {
    pub w: u16,
    pub h: u16,
}

impl Size {
    pub fn to_c_size(&self) -> libc::winsize
    {
        libc::winsize {
            ws_row: self.h,
            ws_col: self.w,
            ws_xpixel: 0,
            ws_ypixel: 0,
        }
    }
}

pub trait FromLibcResult: Sized {
    type Target;

    fn to_result(self) -> Result<Self::Target, ()>;
}

impl FromLibcResult for libc::c_int {
    type Target = libc::c_int;

    fn to_result(self) -> Result<libc::c_int, ()> {
        match self {
            -1  => Err(()),
            res => Ok(res),
        }
    }
}

impl FromLibcResult for *mut libc::passwd {
    type Target = libc::passwd;

    fn to_result(self) -> Result<libc::passwd, ()> {
        if self == ptr::null_mut() {
            return Err(())
        } else {
            unsafe { Ok(*self) }
        }
    }
}

pub fn pipe(input: &mut File, output: &mut File) -> io::Result<()>
{
    let mut packet = [0; 4096];

    let count = input.read(&mut packet)?;

    let read = &packet[..count];
    output.write_all(&read)?;
    output.flush()?;
    Ok(())
}
