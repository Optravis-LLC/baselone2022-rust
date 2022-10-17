use std::thread;

fn main() {
    let name = String::new();
    drop(name);
    println!("Hello {name}!");
}
