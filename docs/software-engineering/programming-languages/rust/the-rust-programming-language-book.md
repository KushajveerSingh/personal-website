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

## Basic CLI Commands

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
        -   This will use `Cargo.lock` file for getting dependency versions. In case dependencies have changed/added/removed/updated, only those dependencies will be recompiled.
    -   Compile and run - `cargo run`.
    -   Check program compiles - `cargo check`. Does not produce executable, which is must faster.
    -   Release build - `cargo build --release`.
    -   Update dependencies - `cargo update`.

## Filename

-   Use underscore instead of hyphen for filenames.
-   If using hyphen, Rust will internally convert to underscore. And this will be refelcted in the binary as well (which just causes confusion).

## Prelude

-   Eveyr package can specify a set of imports that should always be added, in a prelude file.
-   For `std` the prelude contains things like `Option`, `Result`. So that you don't have to import them like normal.

## Crate

-   Collection of Rust source code files.
-   Binary crate - Where the aim is to create an executable.
-   Library crate - Contains code that is intended to be used in other programs and can't be executed on it own.

## Statements and Expressions

-   _Statement_ - Perform an action and do not return a value.
    -   This is not valid `let x = (let y = 6);` since statements do not return a value.
-   _Expression_ - Evaluate to a resultant value. Do not include semicolon in end.
    -   Calling a function.
    -   Calling a macro.
    -   A new block scope created with curly brackets.
        ```rust
        let y = {
            let x = 3;
            x + 1
        };
        ```

## Variables

-   Use _snake case_ for variable names.
-   **Immutable variable**. Default. Once a value is assigned to a variable, the variable cannot change value.
    -   `let x`.
-   **Mutable variable**. Make variables mutable using `mut`.
    -   `let mut x`.
-   **Constant variable**. Define compile time constants using `const`. Use uppercase and underscore for variable name.
    -   `const SCREAMING_SNAKE_CASE: u32 = 10;`.
-   **Shadowing**. Declar new variable with the same name as a previous variable.
    -   This allows you to apply transformations to a variable, while still keeping it immutable.

## Types

-   All the listed types are allocated on the stack.

### Scalar

-   **Integer** (default `i32`)
    -   8-bit - `i8`, `u8`.
    -   16-bit - `i16`, `u16`.
    -   32-bit - `i32`, `u32`.
    -   64-bit - `i64`, `u64`.
    -   128-bit - `i128`, `u128`.
    -   Architecture dependent - `isize`, `usize`. Used when indexing some sort of collection.
-   **Integer literals**
    -   Decimal - `98_222`.
    -   Hex - `0xff`.
    -   Octal - `0o77`.
    -   Binary - `0b1111_0000`.
    -   Byte (`u8` only) - `b'A'`.
    -   Prefix _Integer_ type in end of literal to specify type
        -   `1_000u32`.
-   **Overflow**
    -   Dubug build - Overflow checks are added. In case of overflow the program will panic.
    -   Release build - Overflow checks are not included. In case of overflow, wrapping will be used.
-   **Floating point** (default `f64`)
    -   32-bit - `f32`.
    -   64-bit - `f64`.
-   **Boolean** (1-byte in size)
    -   `bool` - Values `true`, `false`.
-   **Character** (4-bytes in size)
    -   `char` - `let z: char = 'Z';`.

### Compound

-   **Tuple**
    -   Group together multiple values of different types.
    -   Fixed length.
        ```rust
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let (x, y, z) = tup;
        let x = tup.0;
        ```
    -   **Unit** - Tuple with no values.
        -   `let unit: () = ();`
        -   Represents empty value or empty return type.
        -   Expressions return this implicitly if no return value is specified.
-   **Array**
    -   Same as type but all values have the same type.
    -   Fixed length.
        ```rust
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        let arr = [3; 5]; // Length = 5, All values = 3
        let first_val = arr[0];
        ```
    -   Use `usize` to specify the type for index if needed.
    -   For invalid index values, the program will panic.

## Functions

-   Use _snake case_ for function names.
-   `fn some_function(x: i32) -> i64 {}`.
-   Return the last expression implicitly.

### Control flow

-   **if**
    -   Expression.
        ```rust
        if bool_expression {
        } else if bool_expression {
        } else {
        }
        let x = if condition { 5 } else { 6 };
        ```
-   **loop**
    -   Same as `while true`.
        ```rust
        loop {
        }
        ```
    -   `continue`, `break`.
    -   Use `break return_val` to break from loop and return a value.
        ```rust
        let result = loop {
            break 20;
        };
        ```
    -   Provide labels to loop (begins with single quote) and use `continue label`, `break label` when working with nested loops.
        ```rust
        'outer_loop: loop {
            loop {
                break;
                break 'outer_loop;
                continue;
                continue 'outer_loop;
            }
        }
        ```
-   **while**
    ```rust
    while condition {
    }
    ```
-   **for**
    -   To loop over a collection.
        ```rust
        for element in a {
        }
        ```

## Ownership

-   Provides memory safety in Rust without needing garbage collector.

https://doc.rust-lang.org/nightly/book/ch04-01-what-is-ownership.html
