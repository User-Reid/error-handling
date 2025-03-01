mod read_file;

use read_file::*;

fn main() {
    let file_result = read_file();

    match file_result {
        Ok(file) => println!("{file}"),
        Err(error) => eprintln!("You done fucked up cousin {error}"),
    };
}
