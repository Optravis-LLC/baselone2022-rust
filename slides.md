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

<!-- TODO presentation slide -->

---

## Rust

> A language empowering everyone to build reliable and efficient software.

---

## The core pillars

* fast
* safe
* productive

---

## Core design choices

* Compiled to machine code (no runtime)
* No garbage collector
* Yet safe and productive

---

## Memory safe

Rust fails *at compile time* if it detects:

* use after free
* double free
* dangling pointer
* data race

---

## Memory ownership rules

* Each value has an owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

```rust
fn foo() {
  let s1 = String::from("hello "); // allocate a new string
  { 
    let s2 = String::from("world!");
  } // s2 is droped
} // s1 is droped
```

<!--
* Quick intro to the presented rust syntax
* Introduce the concept of "scope"
* The compiler automatically injects deallocation code.
-->

---

## Move semantic

Ownership may be transfered

```rust
fn foo() {
  let s1 = String::from("hello world!"); // allocate a new string
  let s3 = { 
    let s2 = s1; // move s1 to s2
    s2 // move s2 to s3
  };
} // s3 is droped
```
---

## Memory references

```rust
fn take_mutable_ref(s: &mut String) {
  // The reference is guarandeed to live at least for as long as the scope.
  s.push_str("!");
  // This scope has exclusive access to the reference.
  println!("{s}");
}

fn take_read_only_ref(s: &String) {
  // The reference is guarandeed to live at least for as long as the scope.
  println!("{s}");
  // There may be other scopes with concurrent access. 
  s.push_str("!"); // <-- Compile error
}
```

---

## Use after free

```rust
let s = String::from("hello world");
drop(s);
println!("{s}"); // compile error!
```

---

## No double free

```rust
let s = String::from("hello world"); 
drop(s);
drop(s); // compile error! 
```

---

## No dangling pointer

```rust
fn foo() -> &String {
  &String::from("hello world") // compile error!
}
```

<!-- memory management mistakes are compile-time error -->
