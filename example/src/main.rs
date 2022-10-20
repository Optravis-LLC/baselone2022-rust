use std::sync::{Mutex, MutexGuard};
use std::thread;

fn main() {

    let mutex = Mutex::new(String::from("Hello"));
    thread::scope(|scope| {
        scope.spawn(|| {
            let mut greeting: MutexGuard<String> = mutex.lock().unwrap();
            greeting.push_str(" world!");
            println!("{greeting}");
        });
        scope.spawn(|| {
            let greeting = mutex.lock().unwrap();
            println!("{greeting}");
        });
    });
}
