fn drop(s: String) {}

fn main() {
    let s = String::new();
    drop(s);
    println!("{s}");
}
