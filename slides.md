---
marp: true
title: Build trust with rust
author: Jonathan Cornaz
theme: uncover
backgroundColor: gaia
---

<style>
@import 'https://maxcdn.bootstrapcdn.com/font-awesome/4.7.0/css/font-awesome.min.css';
</style>

# Build trust with rust

---

![width:500](https://static.wixstatic.com/media/3f78ef_8c02390aed384d6982039a6bac5628eb~mv2.png)


**Jonathan Cornaz**

<i class="fa fa-github" aria-hidden="false"></i> [jcornaz](https://github.com/jcornaz)


---

![width:250](assets/rust-logo.svg)

> A language empowering everyone
to build reliable and efficient software.

---

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

# Tour of the language

---

```rust
fn main() {
  println!("Hello Basel One!");
}
```

---

* struct
* tuple
* array
 
---

<!-- Type inference -->

```rust
let x: String = String::new();
```

```rust
let x = String::new();
```

---

<!-- Type inference, continued -->

```rust
fn new_vec<Vec<T>>() -> T { Vec::new() }
```

```rust
fn foo(v: Vec<String>) {}
```

```rust
fn main() {
  let v = new_vec(); // v is inferred to `Vec<String>`
  foo(v);
}
```

---

# Functions and methods

```rust
fn a_top_level_function(s: String) -> usize { ... }
```

```rust
impl MyStruct {
  fn a_static_function(x: bool) -> Self { ... }
  fn new() -> Self { ... }
  fn a_method(self, a: f32, b: String) { ... }
}
```

```rust
fn main() {
  let x = MyStruct::new();
  x.a_method();
}
```

---

# Traits

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

# Absent by design

* No inheritance -> use traits and composition
* No reflection -> use macros
* No null -> use `Option` instead
* No exception
  * `Result` for recoverable errors
    <!-- Examples: Network unavailable, Incorrect user-input, File not found, etc. -->
  * panic for unrecoverable errors
    <!-- Programming mistakes, like index out of bounds -->
    <!-- Out of memory -->

---

# Why is it fast?

* Compiled to machine code
* No runtime
* No garbage collector
* Static dispatch by default
* Zero cost abstractions

---

# A syntax designed for performance
  * Low level access to memory management
  * Costly operation must be explicit and stand out when reading the code
  * Great benchmarking tools
  * Performance focused ecosystem and community

---

# Scope

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

# Move semantic

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

# Shared reference

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

# Mutable reference

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

# Iterators

---

# Pattern matching

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

![width:300](assets/slides_qrcode.png)

*slides*

