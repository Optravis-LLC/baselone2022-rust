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

## Memory is automatically deallocated when out of scope

```rust
fn foo() {
  let s1 = String::from("hello "); // allocate a new string
  { 
    let s2 = String::from("world!");
  } // s2 is free
} // s1 is free

```

<!--
Introduce the concept of "scope"

Mention:
* the compiler automatically injects deallocation code.
* memory leaks are not so easy to introduce.
-->

---

## Memory safe

Rust fails *at compile time* if it detects:

* use after free
* double free
* dangling pointer
* data race

---

## Memory ownership model - References

```rust
fn take_mutable_ref(s: &mut String) {
  // The reference is guarandeed to live at least for as long as the scope.
  s.push_str("!");
  // This scope has exclusive access to the reference.
  println!("{s}");
}

fn take_shared_ref(s: &String) {
  // The reference is guarandeed to live at least for as long as the scope.
  println!("{s}");
  // There may be other scopes with concurrent access 
  s.push_str("!"); // <-- Compile error
}
```

---

## Memory ownership

```rust
fn take_ownership(mut s: String) -> String {
  // The memory will be deallocated when the owned value reaches the end of scope
  
  // It can create a share reference
  take_shared_ref(&s);
  
  // It can create a mutable reference
  take_mutable_ref(&mut s);

  return s // <-- to return is transfering ownership to the scope of the caller
}

fn main() {
  let s1 = String::from("Hello world!");
  take_ownership(s1);
  println!("{s1}"); // compile error (s1 has been moved to `take_ownership`)
}
```

---

## No use after free

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
