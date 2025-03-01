use std::fs::File;
use std::io::stdin;
use std::process::exit;

fn main() {
    let mut x: String = String::new();
    println!("Tell me the damn file you want to see!");
    let user_requested_file = stdin().read_line(&mut x);

    if let Err(error) = user_requested_file {
        eprint!("You done fucked up cousin: {error}");
        exit(1)
    }

    let opened_file = match File::open(x.trim()) {
        Ok(opened) => opened,
        Err(error) => {
            eprint!("Oopsie Poopsies: {error}");
            exit(1);
        }
    };

    println!("{opened_file:#?}");
}
