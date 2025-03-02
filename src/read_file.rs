use std::fs::File;
use std::io::{self, stdin, Read};

pub fn read_file() -> Result<String, io::Error> {
    let mut x: String = String::new();
    println!("Tell me the damn file you want to see!");
    stdin().read_line(&mut x)?;

    let mut file_contents: String = String::new();
    File::open(x.trim())?.read_to_string(&mut file_contents)?;

    Ok(file_contents)
}
