use std::collections::HashMap;
use std::io;

fn main() {
    let mut data = HashMap::new();
    let mut book = String::new();
    let mut author = String::new();
    println!("enter the title of the book");
    io::stdin().read_line(&mut book).expect("Failed to read line");
    println!("enter the author of the book");
    io::stdin().read_line(&mut author).expect("Failed to read line");
    data.insert(book.to_string(), author.to_string());
    println!("{:?}", data)
}
