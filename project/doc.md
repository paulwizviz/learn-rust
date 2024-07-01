# Project Management

In this section we discuss aspects of Rust toolchains, organising source codes and packaging built artifacts.

* [rustup](#rustup)
* [cargo](#cargo)

## `rustup`

This is a command line tool to manage rust compilers, `rustc`, package management tool, `cargo`, and other associated tools.

For details on the installation of `rustup` please refer to [Rust Forge](https://forge.rust-lang.org/infra/other-installation-methods.html).

## `cargo`

This is a command line package management and build tool.

A cargo workspace is a directory that contains a `Cargo.toml`, a `Cargo.lock` files and a series of projects. A Cargo project is a directory contains a `Cargo.toml` file and source files organised around a `src` folder.
