fn basics() {
  let answer = 42;
  println!("Hello {}", answer);
  assert_eq!(answer, 42);

  for i in 0..5 {
    if i %2 == 0 {
      println!("even {}", i);
    } else {
      println!("odd {}", i);
    }
  }

  for i in 0..5 {
    let even_odd = if i % 2 == 0 { "even "} else { "odd" };
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
  println!("square is {}" , res);

  fn abs1(x: f64) -> f64 {
    if x > 0.0 {
      x
    } else {
      -x
    }
  }
  fn abs2(x: f64) -> f64 {
    if x > 0.0 { x} else { -x }
  }
  let num1 = 10.1;
  let num2 = -11.1;
  println!("{}, {}", abs1(num1), abs2(num1));
  println!("{}, {}", abs1(num2), abs2(num2));
}

fn main() {
  basics();
}
