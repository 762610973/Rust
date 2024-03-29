#![allow(unused)]

use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

fn main() {
    // panic!("crash and burn");
    // let v = vec![1, 1, 33, 12];
    // v[10];
    let f = std::fs::File::open("hello.txt");
    let f = std::fs::File::open("hello.txt").unwrap();
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file:{:?}", error)
        }
    };
}

fn open_file() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file:{:?}", e),
            },
            other_error => panic!("problem opening the file:{:?}", other_error),
        },
    };
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
