use std::fs::File;
use std::io::ErrorKind;
use std::io::prelude::*;

fn main() {
    let fp = File::open("hello.txt");
    let mut f = match fp {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };
    f.write_all(b"Hello, world!");
    f.sync_all();
}
