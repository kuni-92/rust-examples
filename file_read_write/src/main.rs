use std::{fs::File, io::{Read, Write}, path::Path};

fn main() {
    let path = Path::new("test.txt");
    let display = path.display();

    let mut file = File::create(path).expect("File create error.");
    file.write("Hello, World".as_bytes()).expect("File write error.");

    let mut file = File::open(path).expect("File open error.");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("File read error.");

    println!("{} contains text is {}", display, buf);
}
