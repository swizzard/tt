use std::fs::read_to_string;
use std::io;

extern crate tt;
use tt::tokenizer::WSP;

fn main() {
    let mut tokenizer = WSP::from_file("pg4955.txt");
    println!("{:?}", tokenizer.next());
    println!("{:?}", tokenizer.next());
    println!("{:?}", tokenizer.next());
    println!("{:?}", tokenizer.next());
    println!("{:?}", tokenizer.next());
}
