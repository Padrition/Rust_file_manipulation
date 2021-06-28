#![allow(unused_variables)]

extern crate rand;
use rand::Rng;
use std::fmt;
use std::fmt::{Display};

fn one_in(n: u32) -> bool{
    rand::thread_rng().gen_weighted_bool(n)
}

#[derive(Debug, PartialEq)]
enum FileState{
    Open,
    Closed,
}

#[derive(Debug)]
struct File{
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match *self{
            FileState::Open => write!(f,"OPEN"),
            FileState::Closed => write!(f,"CLOSED"),
        }
    }
}

impl Display for File{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f,"<{}   ({})>", self.name, self.state)
    }
}

impl File{
    fn new(name: &str) -> File{
        File{
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) ->File{
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }
    
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading!"));
        }
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
    
}

fn open (mut f: File) -> Result<File, String>{
    f.state = FileState::Open;
    Ok(f)
}

fn close (mut f: File) -> Result<File, String>{
    f.state = FileState::Closed;
    Ok(f)
}

fn main() {
    let f6_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f6 = File::new_with_data("f6.txt", &f6_data);

    let mut buffer: Vec<u8> = vec![];

    f6 = open(f6).unwrap();
    let f6_length = f6.read(&mut buffer).unwrap();
    f6 = close(f6).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f6);
    println!("{}", f6);
    println!("{} is {} bytes long.", &f6.name, f6_length);
    println!("{}", text);
}
