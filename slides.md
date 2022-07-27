---
marp: true
title: Build trust with rust
author: Jonathan Cornaz
---

# Build trust with rust

---

# Jonathan Cornaz

![](fab fa-github) [jcornaz](https://github.com/jcornaz)

![](https://static.wixstatic.com/media/3f78ef_8c02390aed384d6982039a6bac5628eb~mv2.png/v1/fill/w_219,h_52,al_c,q_85,usm_0.66_1.00_0.01,enc_auto/logo_optravis_RGB.png)

---

# Rust

> A language empowering everyone
to build reliable and efficient software.

* Fast
  <!-- compiled to machine code -->
  <!-- no runtime, no garbage collector -->
  <!-- as fast as c/c++ -->
* Safe
  <!-- very strict compiler -->
  <!-- "if it compiles it works" philosophy -->
* Productive
  <!-- not as much as Kotlin, but still quite good -->
  <!-- much better than c/c++ -->

---

# Tour of the language - Hello world

```rust
fn main() {
  println!("Hello Basel One!");
}
```

---

# Tour of the language - Types

* struct
* tuple
* array
 
---

# Tour of the language - Type inference

```rust
let x: String = String::new();
```

```rust
let x = String::new();
```

```rust
fn new_vec<Vec<T>>() -> T { Vec::new() }

fn foo(v: Vec<String>) {}

fn main() {
  let v = new_vec(); // v is inferred to `Vec<String>`
  foo(v);
}
```

---

# Tour of the language - Functions and methods

```rust
fn a_top_level_function(s: String) -> usize { ... }

impl MyStruct {
  fn a_static_function(x: bool) -> Self { ... }
  fn new() -> Self { ... }
  fn a_method(self, a: f32, b: String) { ... }
}

fn main() {
  let x = MyStruct::new();
  x.a_method();
}
```

---

# Tour of the language - Traits

Trust does not have implementation inheritance.
But it does have traits and generics.

A trait is like an interface, with some differences:

```rust
trait Container {
  fn empty() -> Self;
  fn len(&self) -> usize;
}
```

```rust
impl Container for String {
  fn empty() -> Self { String::new() }
  fn len(&self) -> usize { String::len(self) }
}
```

---

# Why is it fast?

* Compiled to machine code
* No runtime
* No garbage collector
* Static dispatch by default
* Zero cost abstractions
* A syntax designed for performance
  * Low level access to memory management
  * Costly operation must be explicit and stand out when reading the code
  * Great benchmarking tools
  * Performance focused ecosystem and community

---

# Memory ownership - scope

When a an instance reaches the end of a scope, the memory is immediately freed.

```rust
fn foo() {
  let s1 = String::from("Hello");
  {
    let s2 = String::new("world");
  } // s2 is de-allocated
} // s1 is de-allocated
```

---

# Memory ownership - move semantic

```rust
fn foo(s: String) {
  println!("s: ${s}");
} // s is de-allocated
```

```rust
let s = String::from("Hello");
foo(s); // s1 is moved to the scope of f
println!("s: ${s}"); // compilation error
```

---

# Memory ownership - shared reference

```rust
fn foo(s: &String) {
  println!("s: ${s}");
} // s is de-allocated
```

```rust
let s = String::from("Hello");
foo(&s); // a reference to s1 is shared
println!("s: ${s}"); // works fine
```

---

# Memory ownership - mutable reference

```rust
fn foo(s: &String) {
  s.push_str(" world!"); // compilation error
} // s is de-allocated
```

```rust
let mut s = String::from("Hello");
foo(&mut s); // a mutable reference to s1 is shared
println!("s: ${s}"); // works fine
```

---

# Idioms - Option instead of null

---

# Idioms - Result instead of exception

---

# Idioms - Iterators

---

# Macros

---

# Tools

---

# What are the tradeoffs

* Steep learning curve
* More syntax friction

---

# Learning resources

* The rust book
* Rustlings

---

# Questions

<!-- TODO anticipate the most likely questions -->

---

# Thank you

<!-- We are hiring -->

![bg 25%](assets/slides_qrcode.png)
