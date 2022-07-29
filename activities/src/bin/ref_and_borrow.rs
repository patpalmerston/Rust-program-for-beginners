fn main() {
  // let mut s = String::from("hello");
  let mut s = String::from("hello");

  // double immutable ref is okay but needs to be dropped before mutable reference to same value is processed.
  let r3 = &s; // no problem
  let r4 = &s; // no problem
  println!("{}, {}", r3, r4);

  let r1 = &mut s;
  println!("{}", r1);// need to drop reference after single use to be used again. Can not create double references to a single value to be used simultaneously. 

  let r2 = &mut s;
  println!("{}", r2);
}

// fn change(some_string: &mut String) {
//   some_string.push_str(", world");
// }
