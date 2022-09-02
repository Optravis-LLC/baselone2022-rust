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

## No garbage collector

rust code is compiled to machine code, and doesn't need any runtime to be executed

---

```rust {1,4|2|all}
let s = String::from("hello world"); // allocate a new string (on the heap)
drop(s); // deallocate
```

---

## Automatic, immediate memory free

```rust
fn foo() {
  let s = String::from("hello world");
} // out of scope -> deallocate

```

<!-- the compiler automatically injects deallocation -->

---

## No use after free

```rust {1,4|1,2,4|all}
let s = String::from("hello world");
drop(s);
println!("{s}"); // compile error!
```

---

## No double free

```rust {14|1,2,4|all}
let s = String::from("hello world"); 
drop(s);
drop(s); // compile error! 
```

---

## No dangling pointer

```rust {1,3|2}
fn foo() -> &String {
  &String::from("hello world") // compile error!
}
```

<!-- memory management mistakes are compile-time error -->
