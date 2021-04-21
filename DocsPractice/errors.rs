use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    let mut f = easy_open("sss.txt");
}

fn easy_open(filename: &str) -> Result<File, Error> {
    let f = File::open(filename)?;
    return Ok(f)
}