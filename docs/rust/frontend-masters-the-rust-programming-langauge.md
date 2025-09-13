---
title: "Frontend Masters: The Rust Programming Language"
nav_order: 2
parent: Rust
---

<!-- prettier-ignore-start -->
# Frontend Masters: The Rust Programming Language
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

-   [link](https://frontendmasters.com/courses/rust/) The Rust Programming Language by _Richard Feldman_ (4 hours 42 minutes) (May 11, 2021)
-   [link](https://rtfeldman-rust-workshop.netlify.app/) Course website
-   [link](https://github.com/rtfeldman/rust-1.51-workshop) Course GitHub repository
-   [link](https://docs.google.com/presentation/d/1kkTsCrMIVtxYef9T7SV-MWS-nQlnmTniAGaTl6L9Fe4) Course slides

---

## Primitives

-   `let` - similar to `const` in js
-   `let mut` - similar to `let` in js
-   `as` - for type casting
-   `!` - functions ending with `!` are macros
-   `panic!(str)` - similar to `throw` in js
-   last expression in function (after last semicolon) used as default return value

```rust
// String
let str = "Hello";
println!("{}", str);
let same_as_print = format!("{}", str);

// Number
let num = 1.1;
let mut num2 = 1.2;
let num3: f64 = 1.3;

// Function
fn func(x: f64, y: f64) -> f64 {
    return x * y;
}
// The above is same as
fn func(x: f64, y: f64) -> f64 {
    x * y
}

// as
let x: f64 = 1.1;
let y: f32 = 1.2;
let z = x * y as f64;

// Integer
let int1: i8 = 1;
let unsigned_int2: u8 = 2;

// Boolean - At runtime type changed to u8
if cats > 1 {
} else if 1 == 2{
} else {
}

let ternary_operator = if something {
    "val1"
} else if something2 {
    "val2"
} else {
    "val3"
};
```

## Collections

-   `unit` - similar to `void`
-   tuple
-   struct - Syntax sugar only. Similar to named tuples.
-   array - length is fixed.
-   at runtime tuple, struct, array have same memory layout and no additional overhead. Meaning they have the same performance. The read/write operations compile to the same machine code.

```rust
// Tuple
let point: (i64, i64, i64) = (0, 0, 0);
fn get_x(my_point: (i64, i64, i64)) -> i64 {
    my_point.0
}
fn set_x(mut my_point: (i64, i64, i64), x: i64) {
    my_point.0 = x;
}
fn destructure((x, y, z): (i64, i64, i64)) {}
fn destructure ((x, _, _): (i64, i64, i64)) {}

// Unit - Tuple with zero elements (used to return "nothing" from functions)
let unit: () = ();

// Struct
struct Point {
    x: i64,
    y: i64,
    z: i64
}
fn point(x: i64, y: i64, z: i64) -> Point {
    Point { x: x, y: y, z: z }
}
fn get_x(point: Point) -> i64 {
    point.x
}
fn destructure(Point { x, y }) -> i64 {
    x + y
}
fn destrcture(Point { x, .. }) -> i64 {
    x
}

// Array
let arr: [i32, 3] = [2000, 2001, 2002];
for year in arr.iter() {
}
arr[0] = 1998;
let [year1, year2, year3] = arr;
```

## Pattern matching

-   Enums define one of several distinct alternative variants at runtime.
-   At runtime, the variants are converted to `u8`. If number of varaints are more then `u16` is used. By default, the numbering starts from 0. You can assign a start value or values in general to any varaint using `Yellow = 3`.
-   `match` similar to switch, except _break_ is not needed. For default you can use `_ => {}`, but generally avoid that, as you should be handling all variants manually. Plus if you add a variant in the future, you need to know the places it affects.
-   For payloads, the first memory represents the `u8` number of enum, and the additional bytes represent the payload.
-   Also, the size of the largest payload determines the size of each variant in enum. Since `Custom` in the example below takes 4 bytes, it means `Green` also takes 4 bytes.
-   `impl` used for namespacing functions.
    -   `self` takes the type of the thing that comes after `impl`.

```rust
enum Color {
    Green,
    Yellow,
    Red,
    Custom {red: u8, gren: u8, blue: u8 } // Payload
}

let green: Color = Color::Green;
let blue: Color = Color::Custom {red: 0, green: 0, blue: 255 };
println!("In memory Yellow is {}", Color::Yellow as u8);

let color_str = match current_color {
    Color::Green => "green",
    Color::Yellow => "yellow",
    Color::Red => "red",
    Color::Custom {red: 0, green, blue } => format!("custom color with no red (RGB 0, {}, {})", green, blue),
    Color::Custom { red, green, blue } => format!("custom (RGB {}, {}, {})", red, green, blue),
};

impl Color {
    fn is_red(color: Color) -> bool {
        match color {
            Color::Red => true,
            _ => false,
        }
    }

    fn is_yellow(self) -> bool {
        match self {
            Color::Yellow => true,
            _ => false,
        }
    }
}

let is_color_red = Color::is_red(Color::Yellow);
let is_color_yellow = Color::is_yellow(Color::Yellow);
let is_color_yellow = Color::Yellow.is_yellow(); // self allows us to uss method-calling syntax
```

### Type parameters

-   Use `Option<T>` to mimic null, undefined.
    -   `Option` is available globally, so no need to do `Option.Some`, `Option.None`.
-   Use `Result` to return from a function, where the first argument is for success and the second argument is for error.
    -   Available globally.

```rust
enum Option<T> {
    None,
    Some(T),
}

let some_i64: Option<i64> = Some(41); // No need to do Option.Some
let some_i64: Option<i64> = None;
```

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

let failure: Result<i64, String> = Err("failure reason");
let success: Result<i64, String> = Ok(42);
```

## Vectors

-   Internally `vec` stores _memory_index_of_first_element_, _length_, _capacity_. Capacity is the starting size of `vec`.

```rust
let mut years: Vec<i32> = vec![2000, 2001, 2002];
years.push(2003);

let pop_val: Option<i32> = match years.pop() {
    Some(val) => { val },
    None => -1,
};

// vec macro shorthand for
let mut years: Vec<i32> = Vec::capacity(1);
```

## Ownership

-   Only applies to things in the heap.
-   Goal of ownership is to add `dealloc` in code to free the memory. In C/C++ we have to manually add these and do memeory management. In gargabe collected languages, the garbage collector does this. In Rust this is automatic. You can use `unsafe` to do manual memory management.
-   _Ownership_ - Every value is "owned" by a particular scope. At first it's owned by the scope where it was originally created, but ownership can be passed to other scope later on.
-   _moving_ - Transferring ownership of a value is called "moving" that value. Use `return` to transfer ownership.
-   Deallocation happens whern there is no longer any scope owning a value.

**use-after-free** bug

```rust
fn() -> i64 {
    let heap_allocated_thing = vec![1,2,3,4]; // alloc
    dealloc(heap_allocated_thing);

    for item in heap_allocated_thing.iter() { ... }; // use-after-free bug
}
```

**double free** bug

```rust
fn() -> i64 {
    let heap_allocated_thing = vec![1,2,3,4]; // alloc
    dealloc(heap_allocated_thing);
    ...
    dealloc(heap_allocated_thing); // double free bug

}
```

Solution: Rust adds dealloc when a variable goes out of scope. So at the end of function

```rust
fn() -> i64 {
    let heap_allocated_thing = vec![1,2,3,4]; // alloc
    ...

    // dealloc(heap_allocated_thing) added by rust
    return something;
}
```

You can also control when to free memory by creating custom scopes using `{}`

```rust
fn() -> i64 {
    {
        let heap_allocated_thing = vec![1,2,3,4]; // alloc
    } // dealloc(heap_allocated_thing) added by rust

    ...
    return something;
}
```

Problem with the current approach

```rust
fn get_years() -> Vec<i32> {
    let years = vec![2001, 2002, 2003]; // alloc
    return years; // dealloc(years)
}

fn main() {
    let years = get_years(); // use-after-free bug, since `years` already deallocated
}
```

To resolve the above problem rust created the concept of **Ownership**

```rust
fn get_years() -> Vec<i32> {
    let years = vec![2001, 2002, 2003]; // alloc (this scope "owns" years)
    return years; // transfer ownership to main
}

fn main() {
    let years = get_years(); // take ownership
} // dealloc(years) because it went out of scope, without being moved elsewhere
```

Limitation of Ownership.

```rust
fn print_years(years: Vec<i32>) {
    for year in years.iter() {
        println!("Year: {}", year);
    }
} // dealloc(years)

fn main() {
    let years = vec![2000, 2001];

    print_years(years);
    print_years(years); // user-after-free bug, since years already deallocated
}
```

Possible solutions to the above problem

-   Return `years` from the `print_years` function, as that will transfer ownership back to `main`.
-   Or use `.clone()` to pass a new copy to the function. This will hurt performance.
-   If the function takes immutable value, then you can convert it to mutable value inside the function.
-   Recommended: Use Borrowing

```rust
fn main() {
    let mut years = vec![2001, 2002];

    years = print_years(years);
    print_years(years); // this works
    print_years(years.clone()); // this works as well
}
```

## Borrowing

-   To solve limitations of ownership.
-   _Borrowing_ - Obtain a reference from an owned value. Also, the thing borrowing can't move or mutate it. Once the function returns, that reference if no longer in any scope, and there are no longer any _active borrows_ on the original owned value.
-   _Borrow checker_ - Rust's compilier errors around ownership and borrowing are collectively called "the borrow chekcer".
    -   Borrow checker cannot be turned off, even in unsafe code.
-   For immutable references, multiple functions can borrow the variable.
-   For mutables references, only one function can borrow at a time, to prevent race-conditions.
-   References are deallocated when the original variable is deallocated. Also, references never cause anything to get deallocated.
-   Prefer reference over ownership, if a function can accomplish its goals.
    -   It makes the function more restricted in what it can do.
    -   But it makes the caller less restricted. They don't have to clone or modify the function to return the value.

```rust
fn print_years(years: &Vec<i32>) {
    for year in years.iter() {
        println!("Year: {}", year);
    }
} // dealloc(years)

fn main() {
    let years = vec![2000, 2001];

    print_years(&years); // temporarily give print_years access to years (borrow)
    print_years(&years);
}
```

Mutable references have a limitation that as long as you have a mutable borrow active on a value, you are not allowed to have any other borrows (mutable or immutable) active on that value. This helps prevent _data races_.

```rust
let mut years: Vec<i32> = vec![2001, 2002];

let years2: &mut Vec<i32> = &mut years;
let years3: &mut Vec<i32> = &mut years; // error
```

```rust
let mut years: Vec<i32> = vec![2001, 2002];

let years2: &mut Vec<i32> = &years;
let years3: &mut Vec<i32> = &mut years; // error (also the order does not matter, it will
                                        // still error if &mut defined before &years)
```

### Slices

```rust

```

## Lifetimes
