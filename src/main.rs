use std::env;
use std::fs;

fn main() {
    // --snip--
    let filename = "example-code/hello_world.rbg";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
