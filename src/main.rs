use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let query = &args[1];

    let contents = fs::read_to_string("sample.log")
        .expect("could not read the file");

    for (number, line) in contents.lines().enumerate() {
        if line.contains(query) {
            println!("{}: {}", number + 1, line);
        }
    }
}