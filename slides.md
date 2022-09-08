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

No runtime

* Compiled to machine code
* No garbage collector
* Borrow checker

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
  let s1 = String::from("hello ");
  { 
    let s2 = String::from("world!");
  } // s2 is droped
} // s1 is droped
```

<!--
* Quick intro to the presented rust syntax
* Type inference
* Mention why we I use string for the examples
* Introduce the concept of "scope"
* The compiler automatically injects deallocation code.
-->

---

## Move semantic

Ownership may be transfered

```rust
fn foo() {
  let s1 = String::from("hello world!");
  let s3 = { 
    let s2 = s1; // move s1 to s2
    s2 // move s2 to s3
  };
} // s3 is droped
```

---

## Move between functions

```rust
fn foo(s: String) { 
  println!(s);
  s
}

fn main() {
  let s1 = String::from("hello world");
  // s1 is moved to `foo`
  // the result of `foo` is moved to s2
  let s2 = foo(s1);
} // s2 is droped
```
<!--
Ownership can be transfered via:
* Function argument
* Return value
-->

---

## Use after free

```rust
fn foo(s: String) {
  println!("{s}");
} // s is droped

fn main() {
  let s = String::new();
  foo(s); // transfer ownership to `foo`
  foo(s); // compile error!
}
```

---

## Use after free error

![move error](/move_error.png)

---

## A note about `Copy`

```rust
fn foo(num: i32) {
  println!("{s}");
}

fn main() {
  let num = 42;
  foo(num); // copy
  foo(num); // copy
}
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

## Create reference

```rust
let s1 = String::new();
take_read_only_ref(&s1);

let mut s2 = String::new();
take_mutable_ref(&mut s2);

// is implicit for methods
s2.len(); // equivalent to `String::len(&s2)`
s2.push_str("hello"); // equivalent to `String::push_str(&mut s2)`
```

---

## No concurrent access on mutable data

```rust
let mut owner = String::new();
let read_only = &owner; // read-only ref
owner.push_str("Hello"); // mutable ref
read_only.len(); // Compile error!
```

---

## Concurrent borrow error

![borrow error](/borrow_error.png)

---

## No dangling pointer

```rust
fn foo(s: String) -> &String {
  &s // compile error! (s does not live long enough)
} // s is droped
```

