# Package Management and Build Tools

This section discuss aspects of organising Rust project and managing libraries.

## Cargo

You organise your source code around a Cargo workspace (also known as a package) or a directory containing a `Cargo.toml` and an `src` folder. A Rust project is a root directory containing multuple workspaces, a `Cargo.toml` and `Cargo.lock`.

A command line application named `cargo` is an artifact to help you manage workspaces, build crates or perform other related operations such as run tests.

Using this project as an example, we have a root level directory named `learn-rust` and a series of workspaces. Here is how the project was created:

* **STEP 1**: Create a `Cargo.toml` file under the directory `learn-rust`. Add this to the file:
```toml
[workspace]

members = [ 
]
```
* **STEP 2**: Run the command `cargo new floats --bin` to create an executable workspace to demonstrate operations related to float data types.

* **STEP 3**: To execute floats workspace, run the command `cargo run -p floats`.

## Crate

A crate is either a library or an executable. 

## References

* [The Cargo Book](https://doc.rust-lang.org/cargo/index.html)