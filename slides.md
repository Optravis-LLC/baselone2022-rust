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

## Rust

> A language empowering everyone to build reliable and efficient software.

---

## The core pillars

* fast
* safe
* productive

---

## Core design choices

* Compiled to machine code
* No garbage collector
* Borrow checker

<!--
No runtime -> Fast
Borrow checker -> Safe
-->

---

## Stack vs heap

```rust {1|1,2,3,8|4|6-7}
struct Vector { x: i32, y: i32, z: i32 }

fn main() {
  let on_stack = Vector { x: 0, y: 0, z: 0 };
  
  // Anything that has negative performance impact must be explicit
  let on_heap = Box::new(Vector { x: 0, y: 0, z: 0 });
}
```

--

## Static dispatch (and monomorphisation)

```rust
fn say_hello<T : Display>(name: T) {
  println!("Hello {name}!");
}

fn main() {
  say_hello("world"); // call say_hello_str(name: &str)
  say_hello(5); // call say_hello_i32(name: i32)
}
```

---

## No GC, yet memory safe!

* no use after free
* no double free
* no dangling pointer
* no data race

---

## Memory ownership rules

* Each value has an owner.
* There can only be one owner at a time.
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
fn foo() {
  let s1 = String::from("hello world!");
  let s3 = { 
    let s2 = s1; // move s1 to s2
    s2 // move s2 to s3
  };
} // s3 is droped
```

```rust
fn foo(s: String) { 
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
  println!("{s}");
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
fn take_mutable_ref(s: &mut String) {
  // The reference is guarandeed to live for at least as long as the scope.
  s.push_str("!");
  // This scope has exclusive access to the reference.
  println!("{s}");
}

fn take_read_only_ref(s: &String) {
  // The reference is guarandeed to live for at least as long as the scope.
  println!("{s}");
  // There may be other scopes with concurrent access. 
  s.push_str("!"); // <-- Compile error
}
```

--- 

## Create references

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

![concurrent borrow errow](/concurrent_borrow_error.png)

---

## Use after free on reference

```rust
let owner = String::new();
let reference = &owner;
drop(owner); // move ownership
reference.len(); // Compile error!
```

![borrow after move error](/borrow_after_move_error.png)

---

## Dangling pointer

```rust
fn foo() -> &String {
  let s = String::new();
  &s
}
```

![](/dangling_ref_error.png)

---

## Lifetimes

```rust
fn trim<'a>(s: &'a str) -> &'a str {
  s.trim()
}
```

```rust
fn trim(s: &str) -> &str {
  s.trim()
}
```

<!-- 
Note the use of `str` instead of `String`
-->

---

## Option

```rust
enum Option<T> {
  Some(T),
  None,
}
```
```rust
fn may_return_something() -> Option<i32> {
  Some(32)
}
```
```rust
match may_return_something() {
  Some(v) => println!("The value is: {v}"),
  None => println!("There is no value"),
}
```

---

## Result

```rust
enum Result<T, E> {
  Ok(T),
  Err(E),
}
```

```rust
fn may_fail() -> Result<i32, &str> {
  Err("Oops...")
}
```

```rust
match may_fail() {
  Ok(v) => println!("The value is: {v}"),
  Err(err) => println!("Error! {err}"),
}
```

<!-- 
I ues `&str` for the examble, in practice we define error types.
-->

---
