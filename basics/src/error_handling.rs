use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

pub fn error_handling() -> Result<String, io::Error> {
    // panic!("crash and burn");

    // let mut f = File::open("hello.txt");
    // handling error using match
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(error) => panic!("Problem creating the file: {:?}", error),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file : {:?}", other_error);
    //         }
    //     },
    // };

    //unwrap
    // let mut f = File::open("hello.txt").unwrap();

    //expect to return custom error
    // let mut f = File::open("Hello.txt").expect("Cannot open file");

    //Error propagation -> return error to the caller (use question mark operator)
    // let mut s = String::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s);
    // or
    return fs::read_to_string("hello.txt");
}
