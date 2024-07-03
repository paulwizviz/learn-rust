# Strings and Characters

Rust has two types of string: `str` and `String`. They represents sequence of characters. A character is a type known as `char`.

## `char` Type and Literals

A char literal is written with single quotes `'`.

The characteristics of `char` are:

* **Size**: A char in Rust is four bytes (32 bits) in size. This is because it needs to be able to store any Unicode code point, which can be up to 21 bits.
* **Unicode**: The char type can represent any Unicode character, including those outside the ASCII range.
* **Single Character**: It represents a single character, not a string of characters.
* **char vs u8**: While both can represent characters, `u8` is used for raw bytes, and `char` is used for actual Unicode characters. A `char` can represent more than just ASCII characters.
* **Encoding**: Since `char` represents a Unicode scalar value, it can handle a wide range of characters from different languages and symbols.
* **Conversion**: You can convert `char` to and from other types, like `u32`, if you need the numeric representation of the character.

See [working examples](./src/characters.rs).

## String Literals

A string literal is enclosed with `"` with the following characteristics:

* **Memory Location**: String literals are stored in the read-only section of the binary, which is part of the program's data segment. This memory is typically considered part of the program's text or data segment, and it is immutable.
* **Type**: String literals have the type &'static str. This means they are string slices (&str) with a 'static lifetime, indicating they are valid for the entire duration of the program.

For example:

```rust
fn main() {
    let s: &str = "Hello";  // s is a string slice with type &'static str
    println!("{}", s);
}
```

`"Hello"` is stored in the read-only memory, and `s` is a reference to this string slice.

The prefix `r` before a string literal is used to create a string literal. Raw string literals allow you to include characters in the string without needing to escape them. This is particularly useful for strings that contain many backslashes (such as regular expressions or file paths) or other characters that would normally require escaping in a regular string literal.

In this example, we use it to represent `\` without escaping.

```rust
let raw_path = r"C:\Users\name\Documents\file.txt";
println!("{}", raw_path);
```

You can also use additional `#` characters to create raw strings that contain double quotes.

```rust
let raw_string_with_quotes = r#"He said, "Hello, world!""#;
println!("{}", raw_string_with_quotes);
```

## `str` 

This is a slice of strings. However, `str` type cannot by itself be directly annotate variables because it is an unsize type. You can only annotate with `&str`, which is a reference to a string slice (or literals). 

`&str` points to some UTF-8 encoded string data, but it does not own the data. It can point to string literals or slices of `String` types.

See [working example](./src/strings.rs).

## `String` (String Type)

The following are characteristics of a `String` type:

* **Memory Location**: When you create a String from a string literal or any other string data, the String is heap-allocated. The `String` type in Rust is a mutable, growable, and allocated in heap.
* **Ownership**: The `String` type owns the string data and manages the memory for you.
