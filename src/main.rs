use std::fs::File;
use std::io::Read;

pub mod rope;

fn read_text_file(file_name: &str) {
    let mut file = File::open(file_name).expect("Can't open file");
    let mut text = String::new();
    file.read_to_string(&mut text).expect("Can't read file");
    println!("{}", text);
}

fn main() {
    read_text_file("./examples/test.txt");

    let rope = rope::Rope::new();

    println!("Hello, world!");
}
