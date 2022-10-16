use std::{
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let x = Mutex::new(0);
    thread::scope(|scope| {
        scope.spawn(|| {
            let mut x = x.lock().unwrap();
            *x += 1;
            println!("{x}");
        });

        scope.spawn(|| {
            let x = x.lock().unwrap();
            println!("{x}");
        });
    });
}
