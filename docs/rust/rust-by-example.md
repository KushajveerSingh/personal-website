---
title: "Rust by Example"
nav_order: 3
parent: Rust
---

<!-- prettier-ignore-start -->
# Rust by Example
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

-   [link](https://rustbyexample.io/constants) Rust by Example

---

## Variable

```rust
let x = 5;
let mut x = 10;
```

## Constant

-   Can be declared in any scope, including global.
-   At compile time the constant is replaced with the actual value.
-   Must define type in source.
-   Cannot be defined multiple times (like `let`), even if you use different data type.

```rust
const SOMETHING: i32 = 10_000;
```

## Enum

-   Define a type that can be of different variants.
-   To define methods on enums use `impl`.

```rust
enum Animal {
    Dog(String),
    Cat { name: String, age: u8 },
    Bird,
}

let dog = Animal::Dog(String::from("Sam"));
let cat = Animal::Cat {
    name: String::from("Tommy"),
    age: 15,
};
let bird = Animal::Bird;

impl Animal {
    fn name(&self) -> String {
        match self {
            Animal::Dog(name) => String::from(name),
            Animal::Cat { name, .. } => name.clone(),
            Animal::Bird => String::from("Bird"),
        }
    }

    fn age(&self) -> u8 {
        match self {
            Animal::Cat { age, .. } => *age,
            _ => 0,
        }
    }
}

cat.name();
```

## Tuple

-   Fixeed length.

```rust
let tuple: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tuple;
let x = tuple.0;
```

## Arrays

continue https://rustbyexample.io/arrays
