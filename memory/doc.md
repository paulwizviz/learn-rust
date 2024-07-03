# Memory Management

This section gives an overview of the way Rust manages memory and variables.

## `const`

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


## Ownership and Borrowing

Rust is similar to C and C++ that requires you to think about memory when you code. In particular, there are two types of memories you need to have in mind when you code: stack and heap. Unless you explicity create allocate a memory in heap, data is stored in the stack.

Ownership is a set of rules that govern how a variable owns data value. Compliance to rules is done at compile time. If any of the rules are violated, the program won’t compile.

A simple way of thinking about ownership is as follows:
```rust
let i = 1; // i owns the value 1, which is stored in stack memory
let s = "Hello"; // s owns the value "Hello", which is stored in stack
let st = String::from("hello"); // st owns the value "Hello",  which is stored in heap.
```

The rules are:

* Each value in Rust has an owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

Here is an example demonstrating violation of the ownership rules

```rust
let s1 = String::from("hello"); // "hello" is created in heap
let s2 = s1;

println!("{s1}, world!");
```
This will result in the compiled error. The compiler has detected that `s1` is no longer valid as ownership has moved to `s2`.

The solution for `String` object to avoid ownership violation are as follows:
```rust
let x = String::from("hello");
let y = &x; // y "borrows" x by having y store the address of x
```
Alternatively, we could clone the value of `x` and let `y` own the value reference by `x`.
```rust
let x = String::from("hello");
let y = x.clone();
```

However, when it comes of ownership of stack based data, it **not** result in compiler error, for example:
```rust
let x = 1;
let y = x;
```
In this case, a new value of `x` (in stack data) is created and the new value is now owned by `y`. It is cheaper to create new memory in stack than heap.

The following are rules that automatically clone are as follows:
* All the integer types, such as u32.
* The Boolean type, bool, with values true and false.
* All the floating-point types, such as f64.
* The character type, char.
* Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

For detailed explanation, please refer to [What Is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

## References

* [Visualizing memory layout of Rust's data types [See description/first comment]](https://www.youtube.com/watch?v=rDoqT-a6UFg)
* [The Rust Borrow Checker - A Deep Dive - Nell Shamrell-Harrington, Microsoft](https://www.youtube.com/watch?v=HG1fppexRMA)
* [Rust and RAII Memory Management - Computerphile](https://www.youtube.com/watch?v=pTMvh6VzDls)