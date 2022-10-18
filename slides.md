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

![Optravis](optravis_logo.png)

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
* We have a booth
* Quiz!
-->

---

## Rust

> A language empowering **everyone** to build **reliable** and **efficient** software.

<v-clicks>

* Fast
  * Compiled to machine code
  * No runtime
  * No garbage collector
* Safe
  * Strong type system
  * Memory safe
  * Data-race free
* Productive
  * Type inference
  * High level abstraction
  * Excellent toolings

</v-clicks>

<!--
**But complex and hard to learn**
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

## Structs and enums

```rust {1-4|6-9|all}
struct Vector {
  x: i32,
  y: i32,
}

enum State {
  Idle,
  Moving { velocity: Vector },
}

struct Person {
  location: Vector,
  state: State,
}
```

<!--
* Zero cost abstraction
* enums are like sealed classes in Java
-->

---

## Methods

```rust {1-4|6,14|7-9|11-13|all}
struct Vector {
  x: i32,
  y: i32,
}

impl Vector {
  fn dot(self, other: Vector) -> i32 {
    self.x * other.x + self.y * other.y
  }

  fn new(x: i32, y: i32) -> Vector {
    Vector { x, y }
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

<!--
* static dispatch
-->

---

## Traits

```rust {1-4|6-13|all}
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

<!--
* Can define 'static' method available on type
* Can refer to the implementation type via `Self`.
* Can be implemented on third party types!
-->

---

## Generics

```rust
fn greet<T : Display>(name: T) {
  let name_string = name.to_string();
  println!("Hello {name_string}!");
}
```

<v-click>

```rust {1|2|3|all}
greet(String::from("hello")); // greet_String(name: String)
greet(3);                     // greet_i32(name: i32)
greet(true);                  // greet_bool(name: bool)
```

</v-click>

<!--
* static dispatch
* no auto-boxing
* each variant can be optimized independently
* bloat, and `dyn` keyword
-->

---

## Iterators

```rust
fn main() {
  (0..100)
    .filter(|n| *n % 2 == 0)
    .map(|n| n + 1)
    .map(|i| i.to_string())
    .for_each(|v| {
        println!("{v}");
    })
}
```

<!--
* zero cost abstractions
* readable idiomatic code is fast
-->

---

## Macros

<v-click>

```rust {1-2|4-7|all}
#[derive(Serialize, Deserialize)]
struct Person { name: String }

fn main() {
  let name = "BaselOne";
  println!("Hello {name}");
}
```

</v-click>

<!--
* No support for reflection 
* Macros can read and generate code at compile time
-->

---

## Memory ownership model

<v-clicks>

* Ownership (example: `String`)
  * Each value as exactly one owner
  * When the owner's scope ends, the memory is free
* Read only borrow (example: `&String`)
  * There is an owner (the memory is not yet free)
  * There is no concurrent mutable borrow
  * There may be concurrent read-only borrows
* Mutable borrow (example: `&mut String`)
  * There is an owner (the memory is not yet free)
  * There is no concurrent borrow

</v-clicks>

<!--
* no use after free
* no double free
* no dangling pointer
* no data race
* hard to create a memory leak
-->

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

```rust {1|4|5|6|all}
fn drop<T>(s: T) {}

fn main() {
  let s = String::new();
  drop(s);
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

## References

```rust {1-6|8-13|all}
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

## The billion dollar mistake

<v-click>

```rust
enum Option<T> {
  Some(T),
  None,
}
```

</v-click>

<v-click>

```rust
let x: Option<i32> = None;
match x {
  Some(v) => println!("There is something: {v}"),
  None => println!("There is nothing"),
}
```

</v-click>

<!--
- Tony Hoare

-->

---

## Error management

Two kind of errors:

<v-clicks>

* Recoverable: (no network, file not found, invalid input, etc.)
* Unrecoverable: (out-of-memory, bug)

</v-clicks>

---

## Panic

```rust
let array = [0, 1, 2];
let value = array[5]; // Panic!
```

<!--
A panic, is an immediate crash of the software with a stack trace
It cannot be recovered
-->

---

## Result

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

## Wrap up

<v-clicks>

* Fast and reliable
* Modern syntax (but hard to learn)
* Excellent tooling
* High-quality ecosystem

</v-clicks>

---

## Questions

---

## Dependency management

```mermaid
flowchart LR
  X[My App]
  X --> A
  X --> B
  A -- v1.2.0 ---> C
  B -- v1.0.0 ---> C
  C[C ?]
```

---

## Dependency management

```mermaid
flowchart LR
  X[My App]
  X --> A
  X --> B
  A -- v1.2.0 ---> C
  B -- v1.0.0 ---> C
  C[C v1.2.3]
```

---

## Dependency management

```mermaid
flowchart LR
  X[My App]
  X --> A
  X --> B
  A -- v1.2.0 ---> C
  B -- v2.0.0 ---> C
  C[C ?]
```

---

## Dependency management

```mermaid
flowchart LR
  X[My App]
  X --> A
  X --> B
  A -- v1.2.0 ---> C1
  B -- v2.0.0 ---> C2
  C1[C v1.2.1]
  C2[C v2.0.0]
```

