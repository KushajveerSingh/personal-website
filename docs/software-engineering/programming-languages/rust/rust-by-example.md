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

-   [link](https://rustbyexample.io) Rust by Example

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

```rust
let arr_inferred = [1, 2, 3];
let arr_explicit: [i32; 3] = [1, 2, 3];

let arr_same = [3; 5];
let arr_2d = [[1, 2], [3, 4]];

let arr_index = 3;
let val = arr_same[arr_index];

let arr = [0, 1, 2, 3, 4, 5];
let slice = &arr[1..3];
let rest = &arr[1..];
let all = &arr[..];

for element in arr.iter() {
}

for (i, element) in arr.iter().enumerate() {
}

for i in 0..arr.len() {
}
```

## Vector

```rust
let v: Vec<i32> = vec![1,2,3];

let mut v = Vec::<i32>::new();
v.push(1);
v.push(2);

let mut v = Vec::with_capacity(3);
v.push("one");
v.push("two");

println!("Length {}", v.len());
println!("Capacity {}", v.capacity());

// Use collect to convert iterator to vector
let v_from_iter: Vec<i32> = (0..5).collect();
println!("{:?}", v_from_iter);

let third_ele: &i32 = &v_from_iter[3];
let third_ele: i32 = v_from_iter[3];

// Use get to get Option<&T>
let val = v.get(100);
match val {
    Some(val) => print!("42nd element is {}", val),
    None => println!("No 42nd element"),
}

// Both [] and .get() do bounds check. The only difference being
// [] will panic. The performance is the same.
// If performance is a concern use iterator like below
for val in v.iter() {
}

// Use Enum to store different types in vector
enum Variant<'a> {
    Int(i32),
    Float(f64),
    Text(&'a str),
}
let mut v = vec![
    Variant::Int(3),
    Variant::Float(3.14),
    Variant::Text("hello, world"),
];
for val in v.iter() {
    match val {
        Variant::Int(val) => println!("Integer: {}", val),
        Variant::Float(val) => println!("Float: {}", val),
        Variant::Text(val) => println!("Text: {}", val),
    }
}

let slice = &v[0..3];

let last_val = v.pop();
v.insert(2, Variant::Int(10));
let removd = v.remove(1);

v.clear();
```

## HashMap

```rust
use std::collection::HashMap;

let map: HashMap<&str, i64> = HashMap::new();
let mut map = HashMap::from([
    ("Alice", 123),
    ("Bob", 456),
]);

map.insert("Charlie", 36);
map.entry("Alice").or_insert(28); // insert only if key not present

if let Some(age) = map.get("Alice") {
    println!("Alice's age is {}", age);
} else {
    println!("Alice's age is not available");
}

match map.get("Alice") {
    Some(age) => println!("Alice's age is {}", age),
    None => println!("Alice's age is not available"),
}


map.remove("Alice");
if let Some(age) = map.remove("Bob") {
}

if (map.contains_key("Alice")) {
    let age = map.get("Alice").unwrap();
}

for (name, age) in map.iter() {
    println!("{}: {:?}", name, age);
}

map.len();
map.is_empty();
map.clear();
```

## Set

```rust
use std::collections::HashSet;

// Implemented as a wrapper around HashMap<T, ()>
let mut set: HashSet<i32> = HashSet::new();
set.insert(1);
set.insert(2);

set.remove(&2);

let present = set.contains(&2);

// Use collect() to convert iter to set
let b: HashSet<i32> = [3,4,5].iter().cloned().collect();

println!("Union: {:?}", set.union(&b).collect::<HashSet<&i32>>());
println!("Difference: {:?}", set.difference(&b).collect::<HashSet<&i32>>());
println!("Intersection: {:?}", set.intersection(&b).collect::<HashSet<&i32>>());
```

## match

```rust
let value = 1;
match value {
    1 => println!("Match with single pattern"),
    2 | 3 => println!("Match with multiple patterns"),
    2..=5 => println!("MAtch with a range"),
    x if x < 5 => println!("Match with additional condition that must be true"),
    x @ 12..=20 => println!("@ used to create a binding for the value that matched the pattern {}", x),
    _ => println!("Wildcard"),
}
```

## Loop

```rust
// Execute block until you tell to stop
loop {
    break;
}

// Execute block and return a value
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    }
};
println!("Result of counter {}", result);

// while loop
let mut number = 3;
while number != 0 {
    number -= 1;
}

// for-loop to iterate for a collection of items or a range
let arr = [1,2,3];
for ele in arr {
}

// 0..5 = [0,5)
// 0..=5 = [0,5]
for i in 0..=5 {
}
```

## Function

-   Functions with no return value return `()` known as _unit_ type.
-   To return multiple values use tuple `return (x, y)`.

## Closures

-   Same as inline/lambda functions.

```rust
let add_1 = |a: i32, b: i32| -> i32 { return a + b; };
let add_2 = |a: i32, b: i32| -> i32 { a + b };
let add_3  = |a: i32, b: i32| a + b;

// Pass closure to another function as argument
fn func_1<T: Fn(i32, i32) -> i32>(closure: T) -> i32 {
    closure(2,3)
}

fn func_2<T>(closure: T) -> i32
where
    T: Fn(i32, i32) -> i32,
{
    closure(2,3)
}
```

## Strings

-   `&str` - String slice is an immutable reference to a UTF-8 encoded string in memory that is not owned by the program.
-   String literal are stored in the binary as immutable data. These are also considered string slices `&str`.
-   `String` - Mutable UTF-8 string allocated on the heap. Used to store text that is unknown at compile time.

```rust
let s: &str = "hello, world";
let first_word: &str = &s[0..5];

let str_from: String = String::from("hello, world!");


let mut str: String = String::new();
str.push_str("hello, world!");

let world: &str = "world";
let str_format: String = format!("hello, {}", world);

let str_to_str: String = "hello, world".to_string();
```

## Structs

-   Used to group related data together.
-   Use `impl` to define functions on structs.
-   Structs are private and only visible within the module theyu are declared. Use `pub Struct name {}` to make them public.
    -   Similarly you need to make the fields and function within the struct public as well using `pub`, if you want other modules to access them.
-   Multiple `impl` blocks can be defined for the same struct.
-   Use `Default` trait to set default values for the fields.

    ```rust
    #[derive(Default)]
    struct Point {
        x: i32,
        y: i32,
    }
    let point = Point::default();
    let other_point = Point { x: 10, ..Point::default() };
    ```

    -   Define `Default` trait

        ```rust
        impl Default for Point {
            fn default() -> Point {
                Point { width: 10, height: 20}
            }
        }
        ```

-   Three type of structs

    -   **Unit-like** - Have no fields are useful for implementing trait on a type without storing any data in the type itself.
        ```rust
        struct UnitLike;
        ```
    -   **Tuple-like** - Essentially a tuple.
        ```rust
        struct Color(u8, u8, u8);
        let black = Color(0, 0, 0);
        ```
    -   **Named-Field** - Named tuple.

        ```rust
        struct Rectangle {
            width: u32,
            height: u32,
        }
        let rect = Rectangle { width: 10, height: 20 };

        // Use snake_case for associated function names
        impl Rectangle {
            // Type-associated functions - Function that don't take a struct instance as a parameter
            //                             (used as construction or utility functions).
            fn new(width: u32, height: u32) -> Rectangle {
                Rectangle { width, height }
            }

            // Methods - Functions that take "self" as a parameter. self method takes ownership of the
            //           instance.
            fn area(self) -> u32 {
                self.width * self.height
            }

            // Borrow instance. Use this when a method only needs to read, not modify the instance.
            fn can_hold(&self, other: &Rectangle) -> bool {
                self.width > other.width && self.height > other.height
            }

            // Borrow instance and modify.
            fn increase_size(&mut self, width: u32, height: u32) {
                self.width += width;
                self.height += height;
            }

            // Return self to chain method calls
            fn set_width(mut self, width: u32) -> Self {
                self.width += width;
                self
            }
        }
        let rect = Rectangle::new(30, 50);
        let rect2 = Rectangle { width: 30, ..rect };
        ```

## Traits

-   Used to define shared behavior between types (similar to interfaces or abstract base classes).
-   Implement the functions required by the trait, for the type to be a member of that trait.

```rust
trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}
impl Circle {
    fn new(radius: f64) -> Circle {
        Circle { radius }
    }
}
impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}

let circle = Circle::new(5.0);
let area = circle.area();
dbg!(area);

// Function that references a `Shape` trait object and calls it area method
fn print_area(shape: &dyn Shape) {
    println!("Area {}", shape.area());
}
print_area(&circle);
```

## Generics

-   Generics allow you to define functions, structs, enums and trais that can operate on any type.
-   Naming convention is to use single-upper case letter.

    ```rust
    fn first_element<T>(slice: &[T]) -> Option<&T> {
        if slice.is_empty() {
            return None;
        } else {
            return Some(&slice[0]);
        }
    }

    fn combine<T, U>(x: T, y: U) -> (T, U) {
        (x, y)
    }
    ```

-   Use _trait bounds_ to restrict the type that can be used with a generic function.

    ```rust
    // 'largest' function can only be used with types that implement the 'PartialOrd' trait.
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
    }

    // Multiple Triat bounds can be defined as well
    fn max<T: Display + PartialOrd>(x: T, y: T) -> T {
    }
    ```

-   Example of generic struct

    ```rust
    struct GenericStruct<T> {
        field: T,
    }

    impl<T> GenericStruct<T> {
        fn get_field(&self) -> &T {
            &self.field
        }
    }

    // Define impl with a type to restrict to a specific type
    impl GenericStruct<f32> {
        fn get_float_field(&self) -> f32 {
            self.field
        }
    }
    ```

-   Example of enum
    ```rust
    enum HTTPResp<T> {
        Success(T),
        Error(T),
    }
    let resp1: HTTPResp<i32> = HTTPResp::Success(200);
    ```

## Error handling

-   `Result<T, E>` - `Ok<T>`, `Err<E>`.
    ```rust
    match ... {
        Ok(result) => ...,
        Err(error) => ...,
    }
    ```
-   `Option<T>` - `Some<T>`, `None`
    ```rust
    match ... {
        Some(result) => ...,
        None => ...,
    }
    ```
-   Use `?` to propagate errors up the call stack. In which case the caller has to handle the error.
    ```rust
    let result = divide(10, 2)?;
    ```
    -   Typically used when chaining.
    ```rust
    fn sum(a: &str, b: &str) -> Result<i32, ParseIntError> {
        let result = a.parse::<i32>()? + b.parse::<i32>()?;
        Ok(result)
    }
    ```
-   Use `()` when you do not want to return a success value
    ```rust
    fn do_something() -> Result<(), io::Error> {
        Ok(())
    }
    ```
-   `unwrap()` - used to extract the `Ok` value. In case of error, the program will _panic_.
-   `expect()` - similar to `unwrap()` but allows to provide a custom error message in case of err variant.
-   `unwrap_or()` - will extract contents of `Ok`. In case of error, will return the default value provided as an argument.
