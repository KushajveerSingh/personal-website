fn basics() {
    let answer = 42;
    println!("Hello {}", answer);
    assert_eq!(answer, 42);

    for i in 0..5 {
        if i % 2 == 0 {
            println!("even {}", i);
        } else {
            println!("odd {}", i);
        }
    }

    for i in 0..5 {
        let even_odd = if i % 2 == 0 { "even " } else { "odd" };
        println!("{} {}", even_odd, i);
    }

    let mut sum = 0.0;
    for i in 0..5 {
        sum += i as f64;
    }
    println!("sum is {}", sum);

    fn sqr(x: f64) -> f64 {
        x * x
    }
    let res = sqr(2.0);
    println!("square is {}", res);

    fn abs1(x: f64) -> f64 {
        if x > 0.0 { x } else { -x }
    }
    fn abs2(x: f64) -> f64 {
        if x > 0.0 { x } else { -x }
    }
    let num1 = 10.1;
    let num2 = -11.1;
    println!("{}, {}", abs1(num1), abs2(num1));
    println!("{}, {}", abs1(num2), abs2(num2));

    // Pass value by reference
    fn by_ref(x: &i32) -> i32 {
        *x + 1
    }
    let by_ref_val = 10;
    let res1 = by_ref(&by_ref_val);
    let res2 = by_ref(&41);
    println!("{}, {}", res1, res2);

    // Mutable references to modify argument passed
    fn mutable_ref(x: &mut f64) {
        *x = 10 as f64;
    }
    let mut mutable_ref_val: f64 = 0 as f64;
    println!("{}", mutable_ref_val);
    mutable_ref(&mut mutable_ref_val);
    println!("res is {}", mutable_ref_val);

    let arr = [10, 20, 30, 40, 50];
    let first = arr[0];
    println!("first is {}", first);

    for i in 0..arr.len() {
        println!("[{}] = {}", i, arr[i]);
    }
    println!("length {}", arr.len());

    fn slice_sum(values: &[i32]) -> i32 {
        let mut res = 0;
        for i in 0..values.len() {
            res += values[i];
        }
        res
    }
    let slice_sum_res = slice_sum(&arr);
    println!("sum {}", slice_sum_res);
    println!("sum of slice {}", slice_sum(&arr[0..2]));
    println!("sum of remaining slice {}", slice_sum(&arr[2..]));
    println!("{:?}", arr);

    // Trick to know type of any variable, declar it empty and check the compile error
    // let var: () = [1.1, 1.2];

    // On slices use "get" to get the value at index, which is safe. It returns an Option
    println!("first {:?}", arr.get(0));
    println!("last {:?}", arr.get(5));

    let first = arr.get(0);
    let last = arr.get(5);
    println!("first {} {}", first.is_some(), first.is_none());
    println!("last {} {}", last.is_some(), last.is_none());
    println!("first value {}", first.unwrap());

    // last.unwrap() will panic at run-time, since it is out-of-bounds
    let last1 = if last.is_some() { *last.unwrap() } else { -1 };
    let last2 = *arr.get(5).unwrap_or(&-1);
    println!("last1 {}", last1);
    println!("last2 {}", last2);

    let mut vec = Vec::new();
    vec.push(10);
    vec.push(20);
    vec.push(30);
    let first = vec[0]; // will panic if out-of-range
    let maybe_first = vec.get(0);
    println!("first {}", first);
    println!("maybe_first {:?}", maybe_first);

    // Allocate data in 2 spaces
    // 1. Stack - fast to allocate data, order of megabytes
    // 2. Heap - allocating is expensive, order of gigabytes, must be freed later
    // Rust panics are safe in the sense, you get to know of the problem, rather
    // than allowing illegal access to memory.

    // Iterators
    let mut iter = 0..3;
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());

    // Prefer this over using 0..arr.len()
    for i in arr.iter() {
        println!("{}", i);
    }

    let mut v1 = vec![10, 20, 30, 40];
    v1.pop();

    let mut v2 = Vec::new();
    v2.push(10);
    v2.push(20);
    v2.push(30);

    assert_eq!(v1, v2);
    v2.extend(0..2);

    // Strings - allocated or static (which might be on ROM)
    let text = "hello"; // &str slice
    let s = text.to_string(); // it's now an allocated string
    println!("{}", &s);
    // Strings are internally Vec<u8> and &str is &[u8]

    // The f-string equivalent is format!
    let res = format!("hello {:?}", arr);
    println!("{}", res);

    // std::env::args() - to get command line arguments
    for arg in std::env::args() {
        println!("'{}'", arg);
    }
}

fn main() {
    basics();
}
