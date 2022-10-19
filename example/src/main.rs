use std::sync::Mutex;
use std::thread;

fn main() {
    let mutex = Mutex::new(String::from("Hello"));
}
