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

- [link](https://doc.rust-lang.org/nightly/book/index.html) The Rust Programming Langauge book

---

## Basic CLI Commands

- `rustup` - CLI to manage rust versions.
    - Install Rust compiler `curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`.
    - Linker - Install C compiler (gcc, clang) to get linker.
- Rust version - `rustc --version`.
- Update Rust - `rustup udpate`.
- Uninstall Rust and rustup - `rustup self uninstall`.
- Offline documentation - `rustup doc`.
- Cargo - Build system and package manager.
    - Version - `cargo version`.
    - New project - `cargo new project_name`.
    - Create `cargo.toml` for project started without Cargo - `cargo init`.
    - Build project (default build is debug) - `cargo build` will build the project and name the executable `target/debug/project_name`.
        - This will use `Cargo.lock` file for getting dependency versions. In case dependencies have changed/added/removed/updated, only those dependencies will be recompiled.
    - Compile and run - `cargo run`.
    - Check program compiles - `cargo check`. Does not produce executable, which is must faster.
    - Release build - `cargo build --release`.
    - Update dependencies - `cargo update`.

## Filename

- Use underscore instead of hyphen for filenames.
- If using hyphen, Rust will internally convert to underscore. And this will be refelcted in the binary as well (which just causes confusion).

## Prelude

- Eveyr package can specify a set of imports that should always be added, in a prelude file.
- For `std` the prelude contains things like `Option`, `Result`. So that you don't have to import them like normal.

## Statements and Expressions

- _Statement_ - Perform an action and do not return a value.
    - This is not valid `let x = (let y = 6);` since statements do not return a value.
- _Expression_ - Evaluate to a resultant value. Do not include semicolon in end.
    - Calling a function.
    - Calling a macro.
    - A new block scope created with curly brackets.
        ```rust
        let y = {
            let x = 3;
            x + 1
        };
        ```

## Variables

- Use _snake case_ for variable names.
- **Immutable variable**. Default. Once a value is assigned to a variable, the variable cannot change value.
    - `let x`.
- **Mutable variable**. Make variables mutable using `mut`.
    - `let mut x`.
- **Constant variable**. Define compile time constants using `const`. Use uppercase and underscore for variable name.
    - `const SCREAMING_SNAKE_CASE: u32 = 10;`.
- **Shadowing**. Declar new variable with the same name as a previous variable.
    - This allows you to apply transformations to a variable, while still keeping it immutable.

## Types

- All the listed types are allocated on the stack.

### Scalar

- **Integer** (default `i32`)
    - 8-bit - `i8`, `u8`.
    - 16-bit - `i16`, `u16`.
    - 32-bit - `i32`, `u32`.
    - 64-bit - `i64`, `u64`.
    - 128-bit - `i128`, `u128`.
    - Architecture dependent - `isize`, `usize`. Used when indexing some sort of collection.
- **Integer literals**
    - Decimal - `98_222`.
    - Hex - `0xff`.
    - Octal - `0o77`.
    - Binary - `0b1111_0000`.
    - Byte (`u8` only) - `b'A'`.
    - Prefix _Integer_ type in end of literal to specify type
        - `1_000u32`.
- **Overflow**
    - Dubug build - Overflow checks are added. In case of overflow the program will panic.
    - Release build - Overflow checks are not included. In case of overflow, wrapping will be used.
- **Floating point** (default `f64`)
    - 32-bit - `f32`.
    - 64-bit - `f64`.
- **Boolean** (1-byte in size)
    - `bool` - Values `true`, `false`.
- **Character** (4-bytes in size)
    - `char` - `let z: char = 'Z';`.
- **String literals**
    - Immutable and hardcoded in the binary executable.
    - These are string slices as the data type is `&str`.
    - UTF-8 encoded.

### Compound

- **Tuple**
    - Group together multiple values of different types.
    - Fixed length.
        ```rust
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let (x, y, z) = tup;
        let x = tup.0;
        ```
    - **Unit** - Tuple with no values.
        - `let unit: () = ();`
        - Represents empty value or empty return type.
        - Expressions return this implicitly if no return value is specified.
- **Array**
    - Same as type but all values have the same type.
    - Fixed length.
        ```rust
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        let arr = [3; 5]; // Length = 5, All values = 3
        let first_val = arr[0];
        ```
    - Use `usize` to specify the type for index if needed.
    - For invalid index values, the program will panic.

### Heap

- **String**

    ```rust
    let s = String::from("hello");
    let s = String::new();
    let s = "hello".to_string();

    for c in s.chars() {
        // This returns individual characters, not grapheme clusters.
    }
    ```

    - Implemented as a wrapper around `Vec<u8>`.
    - The compiler can coerce `&String` to `&str` when needed.

## Functions

- Use _snake case_ for function names.
- `fn some_function(x: i32) -> i64 {}`.
- Return the last expression implicitly.

## Control flow

### if

Expression.
`rust
    if bool_expression {
    } else if bool_expression {
    } else {
    }
    let x = if condition { 5 } else { 6 };
    `

### loop

- Same as `while true`.
    ```rust
    loop {
    }
    ```
- `continue`, `break`.
- Use `break return_val` to break from loop and return a value.
    ```rust
    let result = loop {
        break 20;
    };
    ```
- Provide labels to loop (begins with single quote) and use `continue label`, `break label` when working with nested loops.

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

### while

```rust
while condition {
}
```

### for

To loop over a collection.

```rust
for element in a {
}
```

### match

- Compare value against a series of patterns and the resultant value of the matched pattern expression is returned.
- Compiler ensures that all possible cases are handled.
- The comparison stops at the first match.

```rust
#[derive(Debug)]
enum USState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        } // no need for comma after curly
    }
}

println!("{}", value_in_cents(Coin::Quarter(USState::Alabama)));
```

**Catch-all**

- Use `_ => something` as a catch all pattern.
- If you need value use `other => something(other)`.

### if let

- Similar to `match` but use when you only want to handle one pattern.

```rust
let var = Some(3);
if let Some(max) = var {
    println!("{max}");
} else {
    println!("something");
}

// The above is the same as `match`
let var = Some(3);
match var {
    Some(max) => println!("{max}"),
    _ => println!("something"),
}
```

### let else

- Use to get the value from the mateched pattern and write logic to handle all the other cases.
- In `if let` we wrote logic for both the matched pattern and catch all case.

```rust
let Coin::Quarter(state) = coin else {
    return None;
};
println!("{state:?}"); // The value from the pattern is bound to the outer scope,
                       // i.e. why you can "state" here.

// The above is same as "if let"
let state = if let Coin::Quarter(state) = coin {
    state
} else {
    return None;
};
```

## Ownership

- Ownership is a set of rules for memory management, which memory safety without needing garbage collector.
- Compile time rules that don't affect runtime performance.
- **Stack**
    - Stores values in the order it gets and removes the values in the opposite order.
    - All data must have a fixed known size at compile time.
- **Heap**
    - For data with unknown size at compile time or a size that might change.
    - _Allocating on the heap_
        - You request certain amount of space on the heap.
        - Memory allocator finds an empty spot in the heap that is big enough, marks it being used and returns a pointer to that location.
        - The pointer to the heap is stored on the stack.
    - Accessing data on the heap is slower than stack.

### Rules

- Each value has an _owner_.
- There can only be one owner at a time.
- When the owner gets out of scope, the value will be dropped.
    - Internally, `drop` function is used to return the memory.
    - Rust calls `drop` automatically at the closing curly bracket.
    - This pattern is called _Resource Acquisition Is Initialization (RAII)_.

### move

- _Move_ is used when referring to copying a variable that points to heap to another variable.
    - Passing a variable to a function follows the rule of _move_ as well.
    - Returning values also follows the rules of _move_.
- In the below example, we say `s1` was moved into `s2`. Now at the end of the scope, we only need to _drop_ `s2`.
    ```rust
    let s1 = String::from("hello"); // s1 is on the stack, with a pointer to the heap
    let s2 = s1; // contents of s1 stack copied to s2 stack, and s1 is dropped
    ```
- When assigning a new value, the previous memory is freed immediately.
    ```rust
    let s1 = String::from("hello");
    // Run `drop` on `s1`
    s1 = String::from("new");
    ```
- For deep copying (copy both stack and heap) use `clone` method.
    ```rust
    let s1 = String::from("hello");
    let s2 = s1.clone();
    ```
- **Copy trait** - The logic will be used instead of _move_. And the original variable will still be valid after assignment to another variable.
    - `Copy` trait is not allowed if the type or any of its parts implement `Drop` trait.
    - Integer, Boolean, Floating, Character, Tuple (should not contain a type with Drop trait) implement `Copy` trait.

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

- Pass a reference to variable that is owned by some other variable.
- **Referencing** - Use `&` to create a reference that refers to the value of a variable, but the ownership does not change. By default, references are immutable.
- **Mutable references** - Use `mut &`.
    - If you have a mutable reference, you are not allowed to have any other reference, including immutable reference.
    - This helps prevent _data races_.

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

- _Dangling pointer_ - Pointer that references a region of memory that is assigned to some other variable.
- Rust gurantees no dangling pointers.
- For example, the below code fails

    ```rust
    fn dangle() -> &String { // Return a reference to string
        let s = String::from("hello");  // s comes into scope

        &s // return reference to s
    } // s goes out of scope and is dropped, so its memory is freed.
      // This causes &s to be dangling pointer, which is an error.
    ```

To resolve the issue use lifetime, or simply return the original string instead.

### Slices

- Reference to a contigous sequence of elements in a collection.
- Used when we want to keep a collection like string in sync with a variable like index of first space character.
- Consider the problem of finding the first word in string
    ```rust
    let mut s = String::from("hello world");
    let word_end_index: usize = first_word(&s); // Let's say this get a value of 10
    s.clear(); // the string is empty now
    // word_end_index still has the value 10, even though the string is empty
    ```
- Slices solve this issue of keeping variables in sync with the collection.
    ```rust
    let s = String::from("hello world");
    let hello: &str = &s[0..5]; // reference to a portion of String
    let world = &s[6..11];
    ```
- **Slice creation**
    - `[starting_index..ending_index]`. Internally stored as a value of the stack with two properties `ptr` to the starting position and `len` specifying the length of the slice.
    - `[..ending_index]` - same as doing `[0..ending_index]`.
    - `[starting_index..]` - same as doing `[starting_index..len]` where `len = s.len()`.
    - `[..]` - same as `[0..len]` where `len = s.len()`.
    - Note that the indexes should be at valid UTF-8 character boundaries, otherwise the program will exit with an error.
- **Use `&str` as parameter instead of `&String`** - This allows the function to work for both `String` and `str` string literals.

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

- Essentially named tuple.
- If you want to later change any field, the entire instance has to be made mutable.
- To use references inside the struct use lifetimes.
- Usage
    - Group similar data together and convey the meaning of our data in our code.
    - If a function computes area of rectange, then passing width and height as parameters is bad. Since the two parameters are not related Better approach is to use struct to group both width and height into a single object.

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

Struct pattern is also useful when you want to constrain a value, and _panic_ if constraints are not met. This helps keep the validation logic in one place.

```rust
pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.")
        }
        Guess{ value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```

### Tuple struct

- Same as tuple, but just provide a name/meaning to the tuple.
- Also, when destructuring you need to provide the struct name as well.

```rust
struct Color(i32, i32, i32);

let black = Color(0, 0, 0);
let Color(r, g, b) = black;
```

### Unit struct

- Similar to unit tuple. It is a struct that has no data.
- Used when you need to implement a trait on some type but don't have any data that you want to store in the type itself.

```rust
struct AlwaysEqual;

let subject = AlwaysEqual;
```

## Enum

- The value can be one of the possible set of values.
- Example IP address is better stored as enum, as it can be version four or six (it cannot be both at the same time). Thus we can enumerate all possible variants.

```rust
enum IpAddrKind {
    V4(String),
    V4_number(u8, u8, u8, u8),
    V6(String),
}

// We get constructor method for free
let four: IpAddrKind = IpAddrKind::V4(String::from("127.0.0.1"));
let four_number: IpAddrKind = IpAddrKind::V4_number(127, 0, 0, 1);
let six: IpAddrKind = IpAddrKind::V6(String::from("::1"));
```

### Option

- Enum to encode that value could be something or it could be nothing.

```rust
enum Option<T> {
    None,
    Some(T),
}

let some_number: Option<i32> = Some(5);
let absent_number: Option<i32> = None;
```

- Use `match` to compare variants of `Option<T>`.

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x + 1),
    }
}

let five = Some(5);
let six = plus_one(five); // `Some(5)` matches the pattern `Some(x)`
let none = plus_one(None); // `None` matches the pattern `None`
```

- When using `Option` inside a function use `?` to propagate the `None` value above the function (assuming the function also returns an `Option`).
    ```rust
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }
    ```

## Methods

- Similar to functions but defined for struct, enum, trait object.
- First parameter is always `self`. You can also use `&self` which is shorthand for `self: &Self`.
- The main purpose of `impl` is to group related code together.
- Calling `obj.something()` rust will do _automatic refercing and dereferncing_, where it will add `&`, `&mut`, `*` so that object matches the signature.
- Multiple `impl` blocks can be defined for the same type.

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

### Associated functions

- Functions defined in `impl` are called _associated functions_.
- The convention is to use `new`.
- `Self` alias for the type that comes after `impl`.
- Use `::` to call the associated function.

```rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

let square = Rectangle::square(10);
```

## Debug trait

- Add `#[derive(Debug)]` for types that don't implement `Debug` trait.
- Usage `dbg!(variable)`.
- Output's to `stderr` and takes ownership of value and returns it as well.
    - `println!` outputs to `stdout` and borrows values instead.

```rust
#[derive(Debug)]
struct Rectangle {}

let rect = Rectangle {
    width: dbg!(30 * scale); // Since dbg returns value, width will get the result
                             // of the expression
}

dbg!(&rect); // Use reference to not give ownership to dbg
```

## Module system

- Manage code organization, public/private details, what names are in each scope. These features are collectively called _module system_.
    - **Paths** - Way of naming an item, such as struct, function or module.
    - **Modules and use** - Let you control the organization, scope, and privacy of paths.
    - **Crates** - Tree of modules that produce a library or executable.
    - **Packages** - Cargo feature to build, test, and share crates.
    - **Workspaces** - Cargo feature for large projects comprising of interrelated packages that evolve together.

### Crate

- Smallest amount of code that the Rust compiler considers at a time. Collection of Rust source code files.
- Crate can contain modules, and the modules may be defined in multiple files.
- Two types
    - **Binary crate** - Where the aim is to create an executable. `main` function is the entry point.
    - **Library crate** - Does not produce an executable. Contains code that is intended to be shared with multiple projects and can't be executed on it own. Does not have `main` function.
        - This is what people generally refer to _crate_ as well.
- _Crate root_ - Source file that the Rust compiler starts from and makes up the root module of your crate.
    - `src/main.rs` - Root for binary crate with same name as package.
    - `src/lib.rs` - Root for library crate with same name as package.
    - For multiple binary crates, place them in `src/bin`.

### Package

- Bundle of one or more crates that provides a set of functionality.
- Contains `Cargo.toml` file that describes how to build those crates.
- Cargo is a package that contains a binary crate for command line usage. It also contains a library crate that the binary crate depends on.
- Package can contain any number of binary crates.
- Package can contain at most one library crate.

For packages with both binary and library crate

- Define the module tree in library crate.
- The binary crate essentially acts like a user of the library (similar to other users will use your library).
- The public items can be used in the binary crate by starting the path with the package name.

### Module

- Group related definitions together and name why they are related.
- Making module public, means the parents can access the module. It does not mean, the parents can access the inner components of the module as well.
- To make the inner components available, you need to make them public as well.

### Paths

- To show Rust where to find an item in a module tree.
- Two types
    - **Absolute path** - Full path starting from a crate root. Rust prefers this.
        - For current crate use the literal `crate` to start the path.
        - For external crate use the crate name to start the path.
    - **Relative path** - Start from current module. Use `self`, `super` (similar to `..` in command line to go one directory up) or identifier to start the path.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

### Compiler Working

- **Start from the crate root**. Compiler checks these files for code to compile
    - `src/main.rs` for binary crate.
    - `src/lib.rs` for library crate.
- **Declaring modules**. In above files you can define modules using `mod module_name;` and compiler will look for the module's code in these places
    - Inline, within curly brackets (`mod module_name { code }`).
    - `src/module_name.rs`.
    - `src/module_name/mod.rs`.
- **Declaring submodules**. If submodules are defined `mod submodule_name;`, the compiler will check these places
    - Inline, withing curly brackets `mod submodule_name { code }`.
    - `src/module_name/submodule_name.rs`. Although Rust docs recommend this, prefer `mod.rs`, as it keeps the module code and submodule code in the same directory.
    - `src/module_name/submodule_name/mod.rs`.
- **Paths to code in modules**. Once a module is part of crate, it can be referred from anywhere else in that same crate. `crate::module_name::submodule_name::function_name`.
- **Private vs public**. Code within module is private from the parent by default.
    - To make the entire module public use `pub mod module_name`.
    - To make specific items public, use `pub` before item declaration.
    - Private items are internal implementation details.
- **The use keyword**. Within a scope, the `use` keyword creates shortcut to items to reduce repetition of long paths. `use crate::module_name::submodule_name::function_name;`. Now you can call `function_name` directly without the long path.
    - The shortcuts are created only in the scope `use` occurs.
        - Use `pub use` to allow code outside scope to refer to that name.
        - `pub use` is also useful when you want to expose a different structure to your program, instead of the client coming up with his own.
    - When bringing in functions specify only the module name in `use`. This ensures the parent module needs to be specified when calling the function, and makes things clearer.
    - When bringing structs, enums, and other items specify the full path.
    - Alias can be provided using `as`. `use std::io::Result as IOResult;`.
    - Use nesting to bring multiple imports in one line
        - `use std::{cmp::Ordering, io};`.
        - `use std::io::{self, Write}` to import `std::io` and `std::io::Write`.
        - `use std::collections::*;` to bring all public items. Use when writing tests and with prelude pattern only.

## Error handling

Two categories

- _Recoverable errors_ - `Result<T, E>` such as file not found. Report the problem to the user and retry the operation.
- _Unrecoverable errors_ - `panic!` such as index out of bounds. Symptom of bugs, so we want to immediately stop the program.

### panic!

The macros does the following

- Print a failure message.
- Unwind. Backtrack the call stack, and clean data from each function.
    - This takes up a significant chunk in the binary.
    - Instead you can abort immediately and have the operating system do the clean up.
        ```toml
        [profile.release]
        panic = 'abort'
        ```
- Clean the stack.
- Quit.

Use `RUST_BACKTRACE=1 cargo run` to view the backtrace of the panic.

- Read from top and find the file where the error occurs.

### Result

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

- Closures providea cleaner way to handler `Result` than `match`.
- `unwrap()` return `Ok` otherwise calls `panic`.
- `unwrap_or_else()` return `Ok` otherwise run the function passed as argument.
- `expect()` - same as `unwrap` but you can provide an error message.
- To propage the errors up the function call use `?` (also works with `Option`).

    This also requires the `From` trait to be implemented. When `?` is used the error is sent to the `from` function to convert it to the return type of the function.

    ```rust
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;

        // The equivalent of above to match
        let username_file_result = File::open("hello.txt");
        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
    }
    ```

### main function

- `main` function can return `Result<(), Box<dyn Error>>`, where `Box<dyn Error>` means "any kind of error".
- This is useful when you want the executable to return an exit code.
    - Exit code of 0 means success. The returned value of `main` is `Ok(())`.
    - Exit code with a number means failure. The Error should have `std::process::Termination` trait which has `report` function to return an `ExitCode`.

## Generics

- Replace specific types with placeholder that represents multiple types to remove code duplication.
- Use with functions, structs, enums, methods to replace conrete data types.
- You also restrict the generic to what trait the types should support, for it to valid.
- No runtime performance. The compiler looks at all the places where generic code is called and generates code for that concrete types the generic code is called with.

```rust
// The function "largest" is generic over some type "T"
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
}

// "Point<T>" struct is generic over some type "T"
struct Point<T> {
    x: T,
    y: T,
}
// "T" required after "impl", so that we can use Point<T>
impl<T> Point<T> {
    fn x(&self) -> &T {
    }
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## Traits

- To define functionality of a particular type has.
- Trait bounds are used to restrict generics to types that have certain behavior.
- Each type implementing the trait must provide its own custom behavior for the body of the method `summarize`.

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```
