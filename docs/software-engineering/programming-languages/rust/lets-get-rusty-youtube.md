---
title: "Let's Get Rusty"
nav_order: 2
parent: Rust
---

<!-- prettier-ignore-start -->
# Let's Get Rusty
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

- [link](https://www.youtube.com/watch?v=ZhedgZtd8gw) Ultimate VS Code setup for Rust development (2025)
- [link](https://www.youtube.com/watch?v=SWwTD2neodE) 5 deadly Rust anti-patterns to avoid
- [link](https://www.youtube.com/watch?v=q3VOsGzkM-M) The ultimate Rust performance guide
- [link](https://www.youtube.com/watch?v=53XYcpCgQWE) 21+ Rust Pro Tips (That Will Change How You Code)

---

## VS Code extensions

- [link](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) rust-analyzer (switch to pre-release version)
- [link](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) codelldb (debugger)
- [link](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml) even better toml
- [link](https://marketplace.visualstudio.com/items?itemName=fill-labs.dependi) dependi (check dependency in cargo file)
- [link](https://marketplace.visualstudio.com/items?itemName=Gruntfuggly.todo-tree) todo tree
    - Modify setting `vscode://settings/todo-tree.regex.regex` to `(//|#|<!--|;|/\*|^|^[ \t]*(-|\d+.))\s*($TAGS)|todo!`. This will include the `todo!` rust macro.

## Rust good patterns

### Error handling

- Do not use `unwrap`, `expect`, `panic!`. It will crash the program. Instead modify the program to handle the error and recover.
- Add these litner rules in `main.rs`.
    ```
    #![deny(clippy::unwrap_used)]
    #![deny(clippy::expect_used)]
    #![deny(clippy::panic)]
    #![deny(unused_must_use)]
    ```

### Default values

- Use `std::default` trait to provide default values like to `struct`.

    ```rust
    #[derive(Default)]
    struct Player {
      level: i8,
      items: Vec<i8>,
      special_power: Option<i8>
    }

    // Use the default values for the data type
    let p1 = Player::default();
    ```

    Alternatively to specify the default value use `impl`.

    ```rust
    impl Default for Player {
      fn default() -> Self {
        Player{level: 1, items: vec![2,3], special_power: None}
      }
    }
    ```

### Converion between types

- `std::convert::From` - conversions that are guranteed to succeed.
- `std::convert::TryFrom` - conversions that can fail.
- `std::convert::FromStr` - parse string (user input) to a value.

### Avoid cloning

- Avoid cloning whereever possible for performance reasons.
- Ways to avoid it
    - Take ownership of the values.
    - Use references.
    - Use smart pointers.

### No wildcard imports

- Do not use `use some_crate::module::*`.
- Use `use some_library::module as Name` if you need a shorter name to access the things in it.
- Three places where using star is acceptable
    - Prelude imports `use std::prelude::*`. These include trais and types choses by library authors.
    - Unit tests require access to everything in source `use super::*`.
    - Re-export modules to collect and reexport items `pub use crate::parser::*`.

## Performance tuning

- Measure
    - Memory usage (heap, stack).
    - CPU time and hot paths.
    - IO bottlenecks (disk, network).
    - Throughput (requests per second).
    - Latency (response time, tail latency).
- Isolate
- Optimize

### hyperfine

Use `hyperfine` to measure the time it takes to run the program.

```
cargo build --release
hyperfine './target/release/prog_name'
```

### flamegraph

Use `flamegraph` to isolate what parts are taking the most time.

In `Cargo.toml` add debug symbols to the release build (just for this tool).

```toml
[profile.release]
debug = true
```

```
cargo flamegraph --bin prog_name
```

### dhat

Use `dhat` for heat profiling and see memory allocations.

In `Cargo.toml` add

```toml
[features]
dhat-heap = []
```

In `main.rs` add

```rust
#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
}
```

```
cargo run --release --features dhat-heap
```

### Optimization techniques

- Generics over dynamic dispatch.
- Inline critical functions.
- Copy over write smart pointer.
- Use rayon for parallel iterator.
- Use dashmap for concurrent hashmap.

## Pro tips

- Use `dbg!(var_name)` instead of `println!("{:?}", var_name)`.
- Use `todo!(string)` instead of todo comment.
- With _trait_ rust decides the type at runtime, incurring a small runtime cost. Instead use generics for compile time gurantees and optimal performance (comes at memory cost, as all the possible types are part of the binary).
- Keep `main.rs` small.
    - Keep application logic in `lib.rs`.
    - `command.rs` for user actions.
    - `storage.rs` for file input and output.
    - `prelude.ts` for imports shared across files.
- Be mindful of public and private code. Having more public code, makes it harder to refactor. `pub(crate)` can be used to make the function public only in the given crate, while still hiding it from the outside world.
- Use parse contructors to define types on things. Instead of using `String` as the type for email, use type driven design to do validation when the type is constructed.

    ```rust
    struct Email(String);

    impl Email {
        fn parse(s: &str) -> Result<Self, String> {
            if s.contains('@') { Ok(Self(s.to_string())) } else { Err("Invalid email".to_owned()) }
        }
    }

    struct User {
        email: Email
    }
    ```

- Enforce invalid states through type driven design. For example, unverified user can't send email. So `send_email` function should not even include the that the user is verified or not. Instead split the user into verified and unverified users. (this assumes verified user cannot go back to unverified user).

    ```rust
    struct UnverifiedUser {
        email: Email
    }
    impl UnverifiedUser {
        fn verify(self) -> VerifiedUser {
            VerifiedUser { email: self.email }
        }
    }

    struct VerifiedUser {
        email: Email
    }
    ```

- Use `rust-toolchain.toml` file to define versions for rust compiler, clippy, formatter and to make builds reproducible.
- Configure CI/CD pipiline
    - `cargo audit` - security audit.
    - `cargo deny check` - dependency policy check and license check.
    - `cargo tarpaulin --fail-under 80` - test coverage.
    - Cache dependencies for faster ci/cd builds
        ```
        cargo check prepare --recipe-path recipe.json
        cargo chef cook --recipe-path recipe.json
        cargo build --release
        ```
