use std::fs::File;
use std::io::{stdin, Read};
use std::process::exit;

fn main() {
    let mut x: String = String::new();
    println!("Tell me the damn file you want to see!");
    let user_requested_file = stdin().read_line(&mut x);

    if let Err(error) = user_requested_file {
        eprint!("You done fucked up cousin: {error}");
        exit(1)
    }

    let mut opened_file = match File::open(x.trim()) {
        Ok(opened) => opened,
        Err(error) => {
            eprint!("Oopsie Poopsies: {error}");
            exit(1);
        }
    };

    let mut file_contents: String = String::new();
    let read_operation = opened_file.read_to_string(&mut file_contents);

    if let Err(error) = read_operation {
        eprint!("You done fucked up cousin on line 27: {error}");
        exit(1)
    }

    println!("{opened_file:#?}");
    println!("{file_contents:#?}");
}
