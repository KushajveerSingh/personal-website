---
title: Book - The Rust Programming Language
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

-   [link](https://doc.rust-lang.org/nightly/book/index.html) The Rust Programming Langauge book

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
-   **String literals**
    -   Immutable and hardcoded in the binary executable.
    -   These are string slices as the data type is `&str`.

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

### Heap

-   **String**
    ```rust
    let s = String::from("hello");
    ```

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

-   Ownership is a set of rules for memory management, which memory safety without needing garbage collector.
-   Compile time rules that don't affect runtime performance.
-   **Stack**
    -   Stores values in the order it gets and removes the values in the opposite order.
    -   All data must have a fixed known size at compile time.
-   **Heap**
    -   For data with unknown size at compile time or a size that might change.
    -   _Allocating on the heap_
        -   You request certain amount of space on the heap.
        -   Memory allocator finds an empty spot in the heap that is big enough, marks it being used and returns a pointer to that location.
        -   The pointer to the heap is stored on the stack.
    -   Accessing data on the heap is slower than stack.

### Rules

-   Each value has an _owner_.
-   There can only be one owner at a time.
-   When the owner gets out of scope, the value will be dropped.
    -   Internally, `drop` function is used to return the memory.
    -   Rust calls `drop` automatically at the closing curly bracket.
    -   This pattern is called _Resource Acquisition Is Initialization (RAII)_.

### move

-   _Move_ is used when referring to copying a variable that points to heap to another variable.
    -   Passing a variable to a function follows the rule of _move_ as well.
    -   Returning values also follows the rules of _move_.
-   In the below example, we say `s1` was moved into `s2`. Now at the end of the scope, we only need to _drop_ `s2`.
    ```rust
    let s1 = String::from("hello"); // s1 is on the stack, with a pointer to the heap
    let s2 = s1; // contents of s1 stack copied to s2 stack, and s1 is dropped
    ```
-   When assigning a new value, the previous memory is freed immediately.
    ```rust
    let s1 = String::from("hello");
    // Run `drop` on `s1`
    s1 = String::from("new");
    ```
-   For deep copying (copy both stack and heap) use `clone` method.
    ```rust
    let s1 = String::from("hello");
    let s2 = s1.clone();
    ```
-   **Copy trait** - The logic will be used instead of _move_. And the original variable will still be valid after assignment to another variable.
    -   `Copy` trait is not allowed if the type or any of its parts implement `Drop` trait.
    -   Integer, Boolean, Floating, Character, Tuple (should not contain a type with Drop trait) implement `Copy` trait.

```rust
fn main() {
    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value "moves" into the function

    let x = 5; // x comes into scope
    makes_copy(x); // Because i32 implements "Copy" trait,
                   // x does not move into the function,
                   // and thus x can be used afterward.

    {
        let s1 = gives_ownership(); // gives_ownership moves its return value into s1
        let s2 = String::from("hello"); // s2 comes into scope
        let s3 = takes_and_gives_back(s2) // s2 is moved into takes_and_gives_back,
                                          // takes_and_gives back moves its return value
                                          // into s3
    } // s3 goes out of scope and is dropped.
      // s2 was moved, so nothing special happens.
      // s1 goes out of scope and is dropped.

} // x goes out of scope, then s (but s was already moved, so nothing special happens)

fn takes_ownership(some_string: String) { // some_string comes into scope
} // some_string goes out of scope and `drop` is called.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
} // some_integer goes out of scope. Nothing special happens.

fn takes_and_gives_back(string: String) -> String { // string comes into scope
    string // string is returned and moves out to the calling function
}
```

## Borrowing

-   Pass a reference to variable that is owned by some other variable.
-   **Referencing** - Use `&` to create a reference that refers to the value of a variable, but the ownership does not change. By default, references are immutable.
-   **Mutable references** - Use `mut &`.
    -   If you have a mutable reference, you are not allowed to have any other reference, including immutable reference.
    -   This helps prevent _data races_.

```rust
fn main() {
    // s1 has stack values (ptr to heap, len, capacity)
    // s1 has heap values (the actual hello data)
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
}

// s has stack value (ptr to the s1 stack ptr, which then points to the heap)
fn calculate_length(s: &String) -> usize {
    s.len();
}
```

Reference's scope starts from where it is introduced to the last point where it is used. So this code will work, even though we have both immutable and mutable references.

```rust
let mut s = String::from("hello");

let r1 = &s; // scope of r1 starts
println!("{r1}"); // scope of r1 ends, since it is not used after

let r3 = &mut s;
```

### Returning a reference

-   _Dangling pointer_ - Pointer that references a region of memory that is assigned to some other variable.
-   Rust gurantees no dangling pointers.
-   For example, the below code fails

    ```rust
    fn dangle() -> &String { // Return a reference to string
        let s = String::from("hello");  // s comes into scope

        &s // return reference to s
    } // s goes out of scope and is dropped, so its memory is freed.
      // This causes &s to be dangling pointer, which is an error.
    ```

To resolve the issue use lifetime, or simply return the original string instead.

### Slices

-   Reference to a contigous sequence of elements in a collection.
-   Used when we want to keep a collection like string in sync with a variable like index of first space character.
-   Consider the problem of finding the first word in string
    ```rust
    let mut s = String::from("hello world");
    let word_end_index: usize = first_word(&s); // Let's say this get a value of 10
    s.clear(); // the string is empty now
    // word_end_index still has the value 10, even though the string is empty
    ```
-   Slices solve this issue of keeping variables in sync with the collection.
    ```rust
    let s = String::from("hello world");
    let hello: &str = &s[0..5]; // reference to a portion of String
    let world = &s[6..11];
    ```
-   **Slice creation**
    -   `[starting_index..ending_index]`. Internally stored as a value of the stack with two properties `ptr` to the starting position and `len` specifying the length of the slice.
    -   `[..ending_index]` - same as doing `[0..ending_index]`.
    -   `[starting_index..]` - same as doing `[starting_index..len]` where `len = s.len()`.
    -   `[..]` - same as `[0..len]` where `len = s.len()`.
    -   Note that the indexes should be at valid UTF-8 character boundaries, otherwise the program will exit with an error.
-   **Use `&str` as parameter instead of `&String`** - This allows the function to work for both `String` and `str` string literals.

    ```rust
    fn first_word(s: &str) -> &str {
    }

    fn main() {
        let s = String::from("hello world");
        first_word(&s[0..6]);
        first_word(&s);

        let literal = "hello";
        first_word(&literal[0..6]);
        first_word(literal);
    }
    ```

## Struct

-   Essentially named tuple.
-   If you want to later change any field, the entire instance has to be made mutable.
-   To use references inside the struct use lifetimes.

```rust
struct User {
    active: bool,
    username: String,
    email: String,
}

let user1 = User {
    active: true,
    username: String::from("hello"),
    email, // same as writing "email: email,"
};

user1.active
user1.username

let user2 = User {
    email: String::from("hello@example.com"),
    ...user1 // Use the same values from user1
}; // This also moves user1 to user2, leaving user1 unusable (because we used
   // username from user1 which is a heap value)
```

### Tuple struct

-   Same as tuple, but just provide a name/meaning to the tuple.
-   Also, when destructuring you need to provide the struct name as well.

```rust
struct Color(i32, i32, i32);

let black = Color(0, 0, 0);
let Color(r, g, b) = black;
```

### Unit struct

-   Similar to unit tuple. It is a struct that has no data.
-   Used when you need to implement a trait on some type but don't have any data that you want to store in the type itself.

```rust
struct AlwaysEqual;

let subject = AlwaysEqual;
```

## Crate

-   Collection of Rust source code files.
-   Binary crate - Where the aim is to create an executable.
-   Library crate - Contains code that is intended to be used in other programs and can't be executed on it own.

https://doc.rust-lang.org/nightly/book/ch05-02-example-structs.html
