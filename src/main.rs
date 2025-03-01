use std::fs::File;
use std::process::exit;

fn main() {
    let x = match File::open("story.txt") {
        Ok(file) => file,
        Err(error) => {
            eprintln!("You got a fuckin error dog: {error:#?}");
            exit(1)
        }
    };

    println!("{x:#?}");
}
