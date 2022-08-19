---
theme: default
background: https://source.unsplash.com/1600x900/?rust
class: 'text-center'
highlighter: shiki
---

<style>
@import 'https://maxcdn.bootstrapcdn.com/font-awesome/4.7.0/css/font-awesome.min.css';
</style>

# Build trust with rust

---

# About ne

---
class: 'text-center'
---

![width:250](assets/rust-logo.svg)

> A language empowering everyone
to build reliable and efficient software.

---

* Fast
* Safe
* Productive

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

<!-- TODO add link to slides -->