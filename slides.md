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

## About me

> **Jonathan Cornaz**
> 
> Senior software developer at Optravis

* Mainly Java from 2010 to 2016
* Mainly Kotlin since 2016
* Rust for hobby projects since 2022
  * benimator: A sprite animation library
  * beancount-parser: A parser for beancount files

---

## Rust

> A *language* empowering everyone to build *reliable* and *efficient* software.

* safe
* fast
* productive

<!--
* Compiled to machine code
* No runtime
* No garbage collector
* Borrow checker
-->

## No GC, yet memory safe!

* no use after free
* no double free
* no dangling pointer
* no data race

---

## Memory ownership rules

* Each value has an exactly one owner.
* When the owner goes out of scope, the value will be dropped.

```rust {1|1,2|3,4|3-5|1,2,6|all}
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
fn foo(s: String) -> String { 
  println!(s);
  s
}
```

```rust
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
```
```rust {monaco}
fn main() {
  let s = String::new();
  foo(s); // transfer ownership to `foo`
  foo(s); // compile error! (use after free)
}
```

<!--
"double free" is also prevented the same way, as it is a special case of "use after free"
-->

![move error](/move_error.png)

---

## The Copy trait

```rust
fn foo(num: i32) {
  println!("{num}");
}
```
```rust
fn main() {
  let num = 42;
  foo(num); // copy
  foo(num); // copy (no error)
}
```

---

## References

```rust
fn take_read_only_ref(s: &String) {
  // The reference is guarandeed to live for at least as long as the scope.
  println!("{s}");
  // There may be other scopes with concurrent access. 
  s.push_str("!"); // <-- Compile error
}

fn take_mutable_ref(s: &mut String) {
  // The reference is guarandeed to live for at least as long as the scope.
  s.push_str("!");
  // This scope has exclusive access to the reference.
  println!("{s}");
}
```


## Methods

```rust
impl String {
  fn add(self, other: &str) -> Self {... }
  fn len(&self) -> usize { ... }
  fn push_str(&mut self) { ... }
}
```

---

--- 

## Create references

```rust
let s1 = String::new();
take_read_only_ref(&s1);

let mut s2 = String::new();
take_mutable_ref(&mut s2);
```

```rust
// is implicit for methods
let s3 = s1.add(&s2);
s3.len(); // equivalent to `String::len(&s2)`
s3.push_str("hello"); // equivalent to `String::push_str(&mut s2)`
```

---

## No concurrent access on mutable data

```rust
let mut owner = String::new();
let read_only = &owner; // read-only ref
owner.push_str("Hello"); // mutable ref
read_only.len(); // Compile error!
```

![concurrent borrow errow](/concurrent_borrow_error.png)

<!-- 
Note the use of `str` instead of `String`
-->

---

## Result

```rust
enum Result<T, E> {
  Ok(T),
  Err(E),
}
```

```rust
struct MyError;

fn may_fail() -> Result<i32, MyError> {
  Err(MyError)
}
```

```rust
match may_fail() {
  Ok(v) => println!("The value is: {v}"),
  Err(err) => println!("Error! {err}"),
}
```

```rust
fn foo() -> Result<String, MyError> {
  let v: i32 = may_fail()?;
  Ok(format!("{v}"))
}
```

---
