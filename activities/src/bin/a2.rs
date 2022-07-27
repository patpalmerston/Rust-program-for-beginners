// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

// fn sub(a: i32, b: i32) -> i32 {
//   a - b
// }

fn sum(a: i32, b: i32) -> i32 {
  a + b
}

fn display(result: i32) {
  println!("{:?}", result);
}


fn main() {
  let result = sum(1, 2);
  display(result);
}
