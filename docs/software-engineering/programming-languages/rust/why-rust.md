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

## Resources

- [link](https://www.oreilly.com/content/why-rust/) Why Rust? Trustworthy, concurrent systems programming by _Jim Blandy_ (60 pages) (Sep 2015)

---

Rust is a statically and strongly typed systems programming language.

- statically - all types are known at compile-time.
- strongly - types make it harder to write incorrect programs.
- systems - generate the best machine code with full control of memory use.

## Problems with C/C++

Systems programming languages (C, C++) have two main problems

- Difficult to write secure code (buffer overflow).
- Difficult to write multithreaded code.

These problems mostly stem from the standard (C/C++) not making the compiler responsible for detecting and handling odd behavior like running off the end of an array. Instead, the standard makes the programmer responsible for ensuring those conditions never arise in the first place.

## Goals of Rust

- (C++ also) Zero-overhead principle. What you don't use, you don't pay for. And what you do use, you couldn't hand code any better.
- Memory safety.
- Data-race-free concurrency.

## What is _type safety_?

- **Undefined behavior** - When program does something, for which standard has no requirement. For example, in C an element can be accessed outside an array. And the standard does not define what to do in this situation.
- **Well defined program** - If a program has been written so that no possible execution can exhibit undefined behavior, the the program is _well defined_.
- **Type safe** - If a language's type system ensures that every program is well defined, then the language is called _type safe_.

C/C++ are not type safe. \
Python, Java, JavaScript, Ruby, Haskell are type safe.

### unsafe

Rust provides `unsafe` block, where some of Rust's type rules are relaxed, allowing use of unrestricted pointers, using blocks of raw memory with any type, calling any C function, and using inline assembly language. It is the programmer's responsibility to avoid undefined behavior.

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

- No runtime cost in either case, but C++ takes longer to compile (check differences below for reason).
- In Rust, a copy of the generic function is created for each unique call to `min`. Compiler can further inline method calls, take advantage of other aspects of the type, and perform optimizations that depend on the types.

Differences

1. In Rust, you can define a type for `T` as well. Here `Ord` means, `T` should support comparison operator.
2.  - In C++, on each call of `min`, the types are substituted in the template and checked if the result is meaningful.
    - In Rust, `min`'s definition is only checked once, and calls to `min` can be checked solely based on the function's stated type. This allows Rust to produce error messages that locate problems more precisely, since the call to `min` is reported in the error stack.

`Ord` is a **trait** (collection of functionality that a type can implement).

#### Traits

Usually used as bounds for type parameters.

Can also be used to mimic C++ virtual member function i.e. refer to values whose specific type isn't determined until runtime, and then use dynamic dispatch to find the trait's implementation, retrieving the relevant method definition from a table at runtime.

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

In C++, the equivalent is _tagged union_ (enum + union), to ensure type safety. `std::variant` introduced in C++17 as a shorthand.

```c++
std::variant<int, double, std::string> v1 = 10;
std::variant<int, double, std::string> v2 = 3.14;
std::variant<int, double, std::string> v3 = "Hello";
```

## Memory safety

Key promises Rust makes about every program that passes compile-time checks (these form the foundations for memory safety and trustworthy concurrency)

- _No null pointer dereferences_. Program will not crash, if you try to dereference a null pointer.
- _No dangling pointers_. Every value will live as long as it must. Your program will never use a heap-allocated values after it has been freed.
- _No buffer overruns_. Your program will never access elements beyond the end or before the start of the array.

### No Null Pointer Dereferences

Solution

- Never allow null pointers to be created.
- Require each variable to be initialized before using.
- Use `Option<P>`, whenever a `None` value is required. And the only way to extract value from `Option`, is to use `match` statement and find `Some(p)` (here `p` is guaranteed not to be null).
- For situations, when an error needs to be returned from a function use `type Result<T> = std::result::Result<T, std::io::Error>`.
    ```rust
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    ```

After compilation, Rust does produce null pointers in code similar to C++.

### No Dangling Pointers

To ensure heap-allocated values are not accessed after been freed, languages use garbage collection or reference counting. But this increases runtime cost, and garbage collection is also a source of non-deterministic behavior.

Rust has three rules, to specify when each value is freed, and ensure all pointers to it are gone by that point. This is all compile time, and at runtime regular pointers are used.

1. Every value has a single owner at any given time. You can move a value from one owner to another, but when a value's owner goes away, the value is freed along with it.

    Every heap-allocated value has a single pointer that owns it; when its owning pointer is dropped, the value is dropped along with it.

    ```rust
    {
        let s = "kushaj".to_string();
    } // s goes out of scope here; text is freed
    ```

    Assignment moves the value: the destination takes ownership, and the source is no longer considered initialized. The reason being, in most of the cases the source of the assignment isn't going to be used anymore.

    ```rust
    {
        let s = "kushaj".to_string();
        let t1 = s; // t1 takes ownership from 's'
        let t2 = s; // compile-time error: use of moved value: 's'
    }
    ```

    Passing arguments to functions and returning values from a function, are also handled like assignment i.e. the values are moved, leaving the source unusable.

    ```rust
    let s = "kushaj".to_string();
    f(s) // value of 's' moved to 'f'
    s // compile-time error: use of moved value: 's'
    ```

    For simple (primitive) types, the values are copied instead of moved on assignment. Internally, this is done by having a `Copy` trait. For custom types, you can implement `Copy` (need to meet the rules for it though), otherwise you can use `Clone` trait.

    ```rust
    let pi = 3.14;
    let one_eighty = pi;
    ```

    Both `Copy` and `Clone` can be automatically created by the compiler as well

    ```rust
    #[derive(Copy, Clone)]
    Struct Color { r: u8, g: u8, b: u8 }
    ```

2. You can borrow a reference to a value, so long as the reference doesn't outlive the value (or equivalently, its owner). Borrowed references are temporary pointers; they allow you to operate on values you don't own.

    Rust restricts the use of references to ensure that they all disappear before the value they refer to is dropped or moved, so references are never dangling pointers.

    Example of function without reference borrowing

    ```rust
    fn append_to_string(mut t: String) -> String {
        t.push('!');
        t
    }

    let mut s = "Hello, world".to_string();
    s = append_to_string(s);
    println!("{}", s)
    ```

    Example with borrowing

    ```rust
    fn append_to_string(t: &mut String) {
        t.push('!');
        t
    } // 't' goes out of scope, so the borrow has ended

    let mut s = "Hello, world".to_string();
    append_to_string(&mut s); // share a mutable reference to 's'
    println!("{}", s);
    ```

    In the above example
    - `s` always had ownership.
    - `append_to_string` borrowed ownership.

    For cases, when we don't want to modify a value use `&` instead of `&mut`.

    ```rust
    fn print_string(t: &String) {
        println!("{}", t);
    }

    let mut s = "Hello, world".to_string();
    print_string(&s);
    ```

    References cannot outlive the value they point to

    ```rust
    let x = String::new();
    let borrow = &x;
    let y = x; // error: cannot move out of 'x' because it is borrowed
    ```

    The borrowed value must not outlive the owner i.e. a variable must not go out of scope while it's borrowed.

    ```rust
    let borrow;
    let x = String::new();
    borrow = &x; // error: 'x' does not live long enough (since `x` declared after 'borrow')
    ```

    The above is equivalent to

    ```rust
    {
        let borrow;
        {
            let x = String::new();
            borrow = &x; // error
        }
    }
    ```

    If at least one value in a struct is being borrowed, assignment on the whole struct is forbidden.

    ```rust
    let mut v = vec!["hemlock"];

    let borrow = &v[0]; // borrow first element
    v = vec!["wormwood"]; // error: cannot assign to 'v' because it is borrowed
    ```

3. You can only modify a value when you have exclusive access to it.
    - While you borrow a shared reference to a value, nothing can modify it or cause it to be dropped.
        ```rust
        let x: i32 = 128;
        function(&x, &x); // you can borrow as many times as you want, since
                          // shared borrow does not modify the original value.
        ```
    - While you borrow a mutable reference to a value, that reference is the only way to access that value at all.
        ```rust
        let mut x = 128;
        let b1 = &mut x;
        x; // error: cannot use 'x' because it was mutable borrowed
        x += 1; // error: cannot assign to 'x' because it is borrowed
        ```

    Borrow checking on data structures, locks the entire structure.

    ```rust
    let mut v = Vec::new();
    v.push(vec![' ', 'o', 'x']);
    v.push(vec![' ', 'x', 'x']);
    v.push(vec!['o', ' ', ' ']);

    // It does chain of borrows: first v, then v[1], then v[1][0]
    let b1 = &v[2][2];

    v[1][0]; // reads are fine, since the borrow is shared
    v[1][0] = 'o'; // error: cannot borrow 'v' as mutable because it is also borrowed as shared
    ```

### Lifetimes

Functions can return reference to one of its arguments, or some part of the argument. The reason being the caller was able to pass in a reference, the the original things must be alive for the duration of the call.

```rust
fn first(v: &Vec<i32>) -> i32 {
    return &v[0];
}
```

When returning multiple references, it is not clear what reference points to which arguments from the function type. `(&i32, &i32)` does not give any info.

```rust
fn first(x: &Vec<i32>, y: &Vec<i32>) -> (&i32, &i32) {
    return (&x[0], &y[0]);
}
```

In situations like these, define _explicit lifetimes_ on the references to spell out the relationships.

```rust
// 'a - define lifetime name
// 'b - define lifetime name
fn firsts<'a, 'b>(x: &'a Vec<i32>,
                  y: &'b Vec<i32>)
                  ->
                  (&'a i32, &'b i32) {
    return (&x[0], &y[0]);
}
```

References always have lifetimes associated with them, Rust just allows us to omit them when the situation is unambiguous.

### Buffer overruns

In C++, you don't actually index arrays, you index pointers, which carry no information about the start and end of the array or object they point into.

In Rust, you index arrays and slices, both of which have definite bounds.

- Doing `a[i]`, first checks that `i` falls within the array's size `n`. Sometimes the compiler recognizes that this check can be safely omitted, but when it can't, Rust generates code to check the array's index at runtime.
- Slice is a pointer to the first element included in the slice, along with the number of elements in it. `&a[i..j]` is a slice referring to the i-th through j-1th elements of `a`. Bounds are checked when creating the slice, and also when indexing into the slice.

## Multithreaded programming

Concurrency without data races.

```rust
let thread1 = std::thread::spawn(|| {
    println!("Alphonse");
    return 137;
})

assert_eq(!(try!(thread1.join()), 137));
```

Use `scoped` instead of `spawn`, when you want to access local variables. A thread started with `scoped` never outlives its `JoinGuard`.

```rust
// This fails as we are violating rule 3. You can only modify variables, when you have
// exclusive access to it
let mut x = 1;
let thread1 = std::thread::scoped(|| { x += 8; });
let thread2 = std::thread::scoped(|| { x += 27; });
```

### Mutex

In C/C++, the relationship between data and the data it protects is entirely implicit in the structure of the program. And the developer has to write comments that explain which threads can touch which data structures, and what mutexes must be help while doing so.

In Rust, `std::sync::Mutex` uses borrowing rules to ensure that threads never use a data structure without holding the mutex that protects it. Each mutex own the data it protects, and threads can borrow a reference to the data only by locking the mutex.

```rust
let x = std::sync::Mutex::new(1);
let thread1 = std::thread::scoped(|| {
    *x.lock().unwrap() += 8;
});
let thread2 = std::thread::scoped(|| {
    *x.lock().unwrap() += 27;
});

thread1.join();
thread2.join();

assert_eq!(*x.lock().unwrap(), 36);
```

### Channels

Threads exchange messages with each other representing requests, replies. Do not communicate by sharing memory; instead share memory by communicating.

Use `std::sync::mpsc::channel<T>() -> (Sender<T>, Receiver<T>)` (works like queue). This is MPSC (Multiple Sender, Single Consumer). The `Sender` end of channel can be cloned and used by multiple threads, while `Receiver` is not allowed to clone.
