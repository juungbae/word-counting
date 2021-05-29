use std::fs;
use std::env;

fn main() {
    let file_name = env::args().last()
        .expect("You must call app with target file"); // First

    let contents = fs::read_to_string(file_name)
        .expect("Cannot find file");

    println!("With text:\n{}", contents);
}
