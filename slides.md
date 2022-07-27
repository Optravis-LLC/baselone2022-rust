---
title: Build trust with rust
pdf: build-trust-with-rust.pdf
slideNumber: true
controls: true
---

# Build trust with rust


# About me

![](fab fa-github) [jcornaz](https://github.com/jcornaz)

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

# Tour of the language - Types

* struct
* tuple
* array

# Tour of the language - Functions and methods

# Tour of the language - Traits

Trust does not have implementation inheritance.
But it does have traits and generics.

A trait is like an interface, with some differences:

* Can define methods and *functions*
* Can use `Self`
* May be implemented on third party types

# Tour of the language - Static dispatch

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

# Memory ownership - move semantic

```rust
fn foo(s: String) {
  println!("s: ${s}");
} // s is de-allocated
```

```rust
let s = String::from("Hello");
foo(s); // s1 is moved to the scope of bar
println!("s: ${s}"); // compilation error
```

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

# Idioms - Option instead of null

# Idioms - Result instead of exception

# Idioms - Iterators

# Macros

# Tools

# What are the tradeoffs

* Steep learning curve
* More syntax friction
