use std::io::{self, Write};

fn main() {
    println!("Hello, world!");
    io::stdout().flush().unwrap();
}
