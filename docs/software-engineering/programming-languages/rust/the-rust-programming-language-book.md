---
title: The Rust Programming Language
nav_order: 5
parent: Rust
---

<!-- prettier-ignore-start -->
# The Rust Programming Language
{: .no_toc }

<details open markdown="block">
  <summary>
    Table of contents
  </summary>
  {: .text-delta }
1. TOC
{:toc}
</details>

<!-- prettier-ignore-end -->

## Resources

-   [link](https://doc.rust-lang.org/nightly/) The Rust Programming Langauge book

---

## Install

-   `rustup` - CLI to manage rust versions.
    -   Install Rust compiler `curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`.
    -   Linker - Install C compiler (gcc, clang) to get linker.
-   Rust version - `rustc --version`.
-   Update Rust - `rustup udpate`.
-   Uninstall Rust and rustup - `rustup self uninstall`.
-   Offline documentation - `rustup doc`.
-   Cargo - Build system and package manager.
    -   Version - `cargo version`.
    -   New project - `cargo new project_name`.
    -   Create `cargo.toml` for project started without Cargo - `cargo init`.
    -   Build project (default build is debug) - `cargo build` will build the project and name the executable `target/debug/project_name`.
    -   Compile and run - `cargo run`.
    -   Check program compiles - `cargo check`. Does not produce executable, which is must faster.
    -   Release build - `cargo build --release`.

## Filename

-   Use underscore instead of hyphen for filenames.
-   If using hyphen, Rust will internally convert to underscore. And this will be refelcted in the binary as well (which just causes confusion).

https://doc.rust-lang.org/nightly/book/ch02-00-guessing-game-tutorial.html
