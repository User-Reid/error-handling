use std::fs::File;
use std::io::{self, stdin, Read};

pub fn read_file() -> Result<String, io::Error> {
    let mut x: String = String::new();
    println!("Tell me the damn file you want to see!");
    let user_requested_file = stdin().read_line(&mut x);

    if let Err(error) = user_requested_file {
        return Err(error);
    }

    let mut opened_file = match File::open(x.trim()) {
        Ok(opened) => opened,
        Err(error) => {
            return Err(error);
        }
    };

    let mut file_contents: String = String::new();
    let read_operation = opened_file.read_to_string(&mut file_contents);

    if let Err(error) = read_operation {
        return Err(error);
    }

    Ok(file_contents)
}
