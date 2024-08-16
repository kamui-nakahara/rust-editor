use crate::Settings;
use std::error::Error;
use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, Stdin, Stdout, Write};
use std::path::PathBuf;
use termion::cursor::{Hide, Show};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::terminal_size;

pub struct Data {
    pub settings: Settings,
    pub path: String,
    pub content: Vec<String>,
    pub width: u16,
    pub height: u16,
    pub x: u16,
    pub y: u16,
    pub command_x: usize,
    pub x_max: bool,
    pub top: usize,
    pub mode: char,
    pub command: String,
    pub stdin: Stdin,
    pub stdout: RawTerminal<Stdout>,
    pub is_dir: bool,
    pub listdir: Vec<String>,
}

impl Data {
    pub fn new() -> Self {
        let (width, height) = terminal_size().unwrap();
        Self {
            settings: Settings::new(),
            path: String::new(),
            content: Vec::<String>::new(),
            width,
            height,
            x: 1,
            y: 1,
            command_x: 0,
            x_max: false,
            top: 0,
            mode: 'n',
            command: String::new(),
            stdin: stdin(),
            stdout: stdout().into_raw_mode().unwrap(),
            is_dir: false,
            listdir: Vec::<String>::new(),
        }
    }
    pub fn load(&mut self, path: String) -> Result<(), Box<dyn Error>> {
        self.x = 1;
        self.y = 1;
        self.top = 0;
        self.x_max = false;
        self.path = path;
        self.content = Vec::<String>::new();
        let target = PathBuf::from(self.path.clone());
        self.is_dir = target.is_dir();
        if self.is_dir {
            self.listdir = Vec::<String>::new();
            let mut listdir1 = Vec::<String>::new();
            let mut listdir2 = Vec::<String>::new();
            for file in target.read_dir().expect("") {
                let filename = file.unwrap().file_name().to_str().unwrap().to_string();
                if PathBuf::from(self.path.clone())
                    .join(filename.clone())
                    .is_dir()
                {
                    listdir1.push(filename);
                } else {
                    listdir2.push(filename);
                }
            }
            listdir1.sort();
            listdir2.sort();
            self.listdir.push("..".to_string());
            self.listdir.push(".".to_string());
            self.listdir.append(&mut listdir1);
            self.listdir.append(&mut listdir2);
            self.mode = 'e';
            write!(self.stdout, "{}", Hide).unwrap();
        } else {
            if self.path.len() == 0 {
                self.content.push(String::new());
            } else {
                for buffer in BufReader::new(File::open(self.path.clone())?).lines() {
                    let line = buffer.unwrap();
                    self.content.push(line);
                }
            }
            self.mode = 'n';
            write!(self.stdout, "{}", Show).unwrap();
        }
        Ok(())
    }
}
