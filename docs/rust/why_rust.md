---
title: Why Rust
nav_order: 1
parent: Rust
---

<!-- prettier-ignore-start -->
# Why Rust
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

## Problems with C/C++

Systems programming languages (C, C++) have two main problems.

-   Difficult to write secure code (buffer overflow).
-   Difficult to write multithreaded code, which is essential for modern machines.

These problems mostly stem from the standard (C, C++) not making the compiler responsible for detecting and handling odd behavior like running off the end of an array. Instead, the standard make the programmer responsible for ensuring those conditions never arise in the first place.

## Goals of Rust

-   (C++ also) Zero-overhead principle. What you don't use, you don't pay for. And what you do use, you couldn't hand code any better.
-   Memory safety.
-   Datta-race-free concurrency.

## What is _type safety_?

-   **Undefined behavior** - When program does something, for which standard has no requirement. For example, in C an element can be accessed outside an array. And the standard does not define what to do in this situation.
-   **Well defined program** - If a program has been written so that no possible execution can exhibit undefined behavior, the the program is _well defined_.
-   **Type safe** - If a language's type system ensures that every program is well defined, then the language is called _type safe_.

C/C++ are not type safe. \
Python, Java, JavaScript, Ruby, Haskell are type safe.

### unsafe

In an unsafe block, some of Rust's type rules are relaxed, allowing use of unrestricted pointers, using blocks of raw memory with any type, calling any C function, using inline assembly langauge.

It is the programmer's responsibility to avoid undefined behavior.

## Rust vs C/C++

### Generics

In Rust

```rust
fn min<T: Ord>(a: T, b: T) -> T {
    if a <= b { a } else { b }
}
```

In C++

```c++
T min(T a, T b) {
    return a <= b ? a : b;
}
```

Similarities

-   No runtime cost in either case, but C++ takes longer to compile.
-   In Rust, a copy of the generic function is created for each unique call to `min`. Compiler can further inline method calls, take advantage of other aspects of the type, and perform other optimizations that depend on the types.

Differences

1. In Rust, you can define a type for `T` as well. Here `Ord` means, `T` should support comparion operator.
2.  - In C++, on each call of `min`, the types are substituted in the template and checked if the result is meaningful.
    - In Rust, `min`'s definition is only checked once, and calls to `min` can be checked solely based on the function's stated type. This allows Rust to produce error messages that locate problems more precisely, since the call to `min` is reported in the error stack.

`Ord` is a **trait** (collection of functionality that a type can implement).

#### Traits

Usually used as bounds for type parameters.

Can also be used to mimic C++ virtual member function i.e. refer to values whose specific type isn't determined until runtime, and then use dynamic dispatch to find the trait's implementation, retreiving the relevant method definition from a table at runtime.

### Enumerations

In Rust

```rust
enum Option<T> {
    None,
    Some(T)
}

fn safe_divide(n: i32, d: i32) -> Option<i32> {
    if d == 0 {
        return None;
    }
    return Some(n / d);
}

match safe_divide(num, denom) {
    None => println!("No quotient."),
    Some(v) => println!("quotient is {}", v)
}
```

In C++, the equivalent is _tagged union_ (enum + union), to ensure type safety. `std::variant` introducted in C++17 as a shorthand.

```c++
std::variant<int, double, std::string> v1 = 10;
std::variant<int, double, std::string> v2 = 3.14;
std::variant<int, double, std::string> v3 = "Hello";
```

## Memory safety

Key promises Rust makes about every program that passes compile-time checks (these form the foundations for memory safety and trustworthy concurrency)

-   _No null pointer dereferences_. Program will not crash, if you try to dereference a null pointer.
-   _No dangling pointers_. Every value will live as long as it must. Your program will never use a heap-allocated values after it has been freed.
-   _No buffer overruns_. Your program will never access elements beyond the end or before the start of the array.

### No Null Pointer Dereferences

Solution

-   Never allow null pointers to be created.
-   Require each variable to be intialized before using.
-   Use `Option<P>`, whenever a `None` value is required. And the only way to extract value from `Option`, is to use `match` statement and find `Some(p)` (here `p` is guranteed not to be null).
-   For situations, when an error needs to be returned from a function use `type Result<T> = std::result::Result<T, std::io::Error>`.
    ```rust
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    ```

After compilation, Rust does produce null pointers in code similar to C++.

### No Dangling Pointers

To ensure heal-allocated values are not accessed after been freed, langauges use garbage collection or reference counting. But this increases runtime cost, and garbage collection is also a source of non-deterministic behavior.

Rust has three rules, to specify when each value is freed, and ensure all pointers to it are gone by that point. This is all compile time, and at runtime regular pointers are used.

1. Every value has a single owner at any given time. You can move a value from one owner to another, but when a value's owner goes away, the value is freed along with it.
2. You can borrow a reference to a value, so long as the reference dosen't outlive the vlaue (or equivalenty, its owner). Borrowed references are temporary pointers; they allow you to operate on values you don't own.
3. You can only modify a value when you have exclusive access to it.
