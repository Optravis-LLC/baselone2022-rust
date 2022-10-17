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

<v-clicks>

* Mainly Java from 2010 to 2016
* Mainly Kotlin since 2016
* Rust for hobby projects since 2020
  * benimator: Sprite animation
  * bhv-arena: Bounding-volume hierarchy for collision detection
  * impacted: 2d collision detection logic
  * beancount-parser: Parser for beancount files

</v-clicks>

<!--
## Optravis
* Build software for software operational transfer pricing
* We have booth
-->

---

## Agenda

* Tour of the language syntax
* Memory ownership system
* Fearless concurrency demo
* Error management
* The Ecosystem and tools

---

## Rust

> A language empowering **everyone** to build **reliable** and **efficient** software.

<v-clicks>

* Safe
  * Strong type system
  * Memory safe
  * Data-race free
* Fast
  * Compiled to machine code
  * No runtime
  * No garbage collector
* Productive
  * Type inference
  * High level abstraction
  * Excellent toolings

</v-clicks>

<!--
**The tradeoff: complexity**
-->

---

## Hello world

```rust
fn main() {
  println!("Hello world!");
}
```

<!-- 
Println! is a macro
-->

---

## Structs

```rust {1-4|6|8|all}
struct Vector {
  x: i32,
  y: i32,
}

struct WithUnamedFields(i32,i32);

struct WithoutField;
```

<!--
On the stack by default (like value-classes)

Zero cost abstraction

No inheritance
-->

---

## Enums and pattern matching

```rust {1,8|2|3|4-7|1-8|10-16}
enum Stuff {
  Unit,
  WithData(f32),
  WithStructuredData {
    x: i32,
    y: i32,
  }
}

fn foo(s: Stuff) -> String {
  match s {
    Stuff::Unit => String::from("unit"),
    Stuff::WithData(0) => String::from("zero"),
    Stuff::WithData(v) => v.to_string(),
    Stuff::WithStructuredData { x, y } => format!("({x}, {y})"),
  }
}
```

<!--
Enums can have data.
Like sealed classes.
Pattern matching!
-->

---

## Other types

<v-clicks>

* tuples (example: `(i32, String)`)
* type alias (example: `type Foo = Bar<i32>`)
* arrays (exmaple: [0; i32])

</v-clicks>

---

## Methods

```rust {1-4|6,14|7-9|11-13}
struct Vector {
  x: i32,
  y: i32,
}

impl Vector {
  fn new(x: i32, y: i32) -> Self {
    Self { x, y }
  }
  
  fn dot(self, other: Self) -> i32 {
    self.x * other.x + self.y * other.y
  }
}
```

<v-click>

```rust
let v1 = Vector::new(1, 2);
let v2 = Vector::new(3, 4);
let dot = v1.dot(v2);
```

</v-click>

<!-- Static dispatch -->

---

## Traits

```rust {1,4|2-3|6,13|7-12|all}
trait Len {
  fn new_empty() -> Self;
  fn len(&self) -> usize;
}

impl Len for String {
  fn new_empty() -> Self {
    String::new()
  }
  fn len(&self) -> usize {
    self.len()
  }
}
```

---

## Generics

```rust
fn greet<T : Display>(name: T) {
  println!("Hello {value}!");
}
```

<v-click>

### Monomorphisation

```rust {1|2|3|all}
greet(String::from("hello")); // greet_String(name: String)
greet(3);                     // greet_i32(name: i32)
greet(true);                  // greet_bool(name: bool)
```

</v-click>

<!--
Generic functions are monomorphised
-->

---

## iterators

```rust
fn main() {
  (0..100)
    .filter(|n| *n % 2 == 0)
    .map(|n| n + 1)
    .map(i32::to_string)
    .for_each(|v| {
      println!("{v}");
    })
}  
```

---

## Macros

<v-clicks>

* There is no reflection support
* Macros can read and generate code at compile time

</v-clicks>

<v-click>

```rust
#[derive(Serialize, Deserialize)]
struct Person { name: String }
```

</v-click>

---

## No GC, yet memory safe!

<v-clicks>

* no use after free
* no double free
* no dangling pointer
* no data race
* hard to create a memory leak

</v-clicks>

---

## Memory ownership model


* Ownership (example: `String`)
  * Each value as exactly one owner
  * When the owner's scope ends, the memory is free
* Read only borrow (example: `&String`)
  * There is an owner (the memory is not yet free)
  * There is no concurrent mutable borrow
  * There may be concurrent read-only borrows
* Mutable borrow (example: &mut String)
  * There is an owner (the memory is not yet free)
  * There is no concurrent borrow

---

## Automatic free


```rust {1,3|2|3}
fn foo() {
  let s1 = String::from("hello");
}
```

<!--
Enforced at compile time
-->

---

## Move semantic

Ownership may be transfered

<v-click>

```rust {1|1-4|7|1,2,3,4,8|9}
fn foo(s: String) -> String { 
  println!("{s}");
  s
}

fn main() {
  let s1 = String::from("hello world");
  let s2 = foo(s1);
}
```

</v-click>

<!--
Ownership can be transfered via:
* Function argument
* Return value
-->

---

## Use after free

```rust {1-3|6|7|8}
fn foo(s: String) {
  println!("{s}");
}

fn main() {
  let s = String::new();
  foo(s);
  println!("{s}");
}
```

<v-click>

![move error](/move_error.png)

</v-click>

<!--
"double free" is also prevented the same way, as it is a special case of "use after free"
-->

---

## The Copy trait

```rust {1-3|6,7|8|all}
fn foo(num: i32) {
  println!("{num}");
}

fn main() {
  let num = 42;
  foo(num); // copy
  foo(num); // copy again (no error)
}
```

---

## References

```rust {1|2-3|4-5|1-6|8|9-10|11-12|8-13}
fn take_mutable_ref(s: &mut String) {
  // The reference is guarandeed to live for at least as long as the scope
  println!("{s}");
  // This scope has exclusive access to the reference
  s.push_str("!");
}

fn take_read_only_ref(s: &String) {
  // The reference is guarandeed to live for at least as long as the scope
  println!("{s}");
  // There may be other scopes with concurrent access
  s.push_str("!"); // <-- Compile error
}
```

---

## Fearless concurrency demo

---

## Null?

```rust
enum Option<T> {
  Some(T),
  None,
}
```

<v-click>

```rust
let x: Option<i32> = None;
match x {
  Some(v) => println!("There is something: {v}"),
  None => println!("There is nothing"),
}
```

</v-click>

---

## Error management

Two kind of errors:

<v-clicks>

* Recoverable: (no network, file not found, invalid input, etc.)
* Unrecoverable: (out-of-memory, bug)

</v-clicks>

---

## Panic for unrecoverable

```rust
let array = [0, 1, 2];
let value = array[5]; // Panic!
```

<!--
A panic, is an immediate crash of the software with a stack trace
It cannot be recovered
-->

---

## Result type

```rust
enum Result<T, E> {
  Ok(T),
  Err(E),
}
```


<v-click>

```rust
struct MyError;

fn may_fail() -> Result<i32, MyError> {
  Err(MyError)
}
```

</v-click>

<v-click>

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

</v-click>

---

## Tooling

<v-clicks>

* toolchain manager (rustup)
* package manager (cargo)
* linter (cargo clippy)
* code formater (cargo fmt)
* documentation generator (cargo doc)
* your favorite IDE
  * IntelliJ, CLion
  * Any LSP compatible editor (VSCode, vim, neovim, etc.)
  
</v-clicks>

---

## Dependency management

cargo understands semver

```mermaid
flowchart
  A --> B
  A -v1.2.0-> C
  B -v1.0.0-> C
  C
```


---

## Documentation

````rust
/// Returns the double of the given value
///
/// # Examples
///
/// ```
/// let result = double(2);
/// assert_eq!(result, 4);
/// ```
fn double(a: i32) -> i32 {
  a * 2
}
````

<!-- 
Show docs.rs
-->

---

## Wrap up

<v-clicks>

* Fast and reliable
* Modern syntax (but hard to learn)
* Excellent tooling
* High-quality ecosystem

</v-clicks>

---

## Questions
