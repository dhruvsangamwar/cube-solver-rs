mod cube;

use std::io::{self, Write};

fn main() {
    // prompt the user for input fi
    println!("CUBE SOLVER");
    print!("please enter cube file path (json): ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let trimmed = input.trim();
    let path = std::path::Path::new(trimmed);
    let mut c = cube::Cube::from_file(path).expect("Failed to load cube");
    c.show();
    println!("--------------------");
    c = cube::Cube::new();
    c.show()
}
