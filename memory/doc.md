# Memory Management

This section gives an overview of the way Rust manages memory and variables.

## Constants `const`

The key characteristics of `const` are:

* Immutable: Constants are always immutable, meaning their value cannot be changed after they are defined.
* Compile-time Evaluation: Constants are evaluated at compile time, so their values must be known and fixed when the program is compiled.
* Global Scope: Constants can be declared in any scope, including global scope, making them accessible throughout your code.
* Type Annotation Required: You must specify the type of a constant when you declare it.
* Uppercase Naming Convention: By convention, constant names are written in uppercase with underscores separating words.

```rust
const NAME: TYPE = value
```
Here is a typical use case for this type.

```rust
const PI: f64 = 3.141592653589793;
const RADIUS: f64 = 5.0;

fn main() {
    let circumference = 2.0 * PI * RADIUS;
    println!("The circumference of the circle is {}", circumference);
}
```

## `static`

The key characteristics of this type are:

* Value has a fixed memory address.
* Can be mutable with the `mut` keyword (`static mut`), though this is unsafe.
* Used for more complex data that might require a fixed memory location.
* Can be used for global variables.

Here is a use case.

```rust
static MESSAGE: &str = "Hello, world!";

fn main() {
    println!("{}", MESSAGE);
}
```

## Variables

If you tried to compile the following:

```rust
    fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6; // Reassigning the variable x with a different value
    println!("The value of x is: {x}");
}
```
This will result in compiler error.

```sh
Compiling playground v0.0.1 (/playground)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
3 |     println!("The value of x is: {x}");
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
```


## Ownership

Ownership is a set of rules that govern how a Rust program manages memory. Compliance to rules is done at compile time. If any of the rules are violated, the program won’t compile.

The rules are:

* Each value in Rust has an owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{s1}, world!");
```

This will result in the compiled error. `s1` is no longer valid as ownership has been moved.

`s1` is a reference to value `"hello"`, which is stored in heap.
The assignment to `s2` transfer ownership from `s1`. 

Please refer to the following for specific type ownerships:

* [String](../strings/doc.md)