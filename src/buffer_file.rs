extern crate rand;

use rand::Rng;
use rand::distributions::Alphanumeric;
use std::io;
use std::fs::{self, File};
use std::fs::OpenOptions;
use std::ops;
use std::iter::Iterator;

pub struct BufferFile {
    file: File,
    path: String,
}

impl BufferFile {
    pub fn new() -> Result<BufferFile, io::Result<()>>
    {
        let file_name = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(10)
                .collect::<Vec<u8>>();
        let file_name = String::from_utf8(file_name).unwrap();
        let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create_new(true)
                .open(format!("/tmp/{}", file_name)).unwrap();
        Ok(BufferFile{
            file,
            path: format!("/tmp/{}", file_name),
        })
    }

    pub fn to_string(&self) -> String
    {
        fs::read_to_string(self.path.clone()).unwrap()
        /*let mut res = String::new();
        let mut out = self.file.try_clone().unwrap();

        out.read_to_string(&mut res).unwrap();
        res*/
    }
}

impl ops::Deref for BufferFile {
    type Target = File;

    fn deref(&self) -> &File
    {
        &self.file
    }
}

impl ops::DerefMut for BufferFile {
    fn deref_mut(&mut self) -> &mut File
    {
        &mut self.file
    }
}
