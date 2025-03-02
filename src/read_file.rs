use std::fs;
use std::io::{self, stdin};

pub fn read_file() -> Result<String, io::Error> {
    let mut x: String = String::new();
    println!("Tell me the damn file you want to see!");
    stdin().read_line(&mut x)?;

    fs::read_to_string(x.trim())
}
