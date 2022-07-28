fn main() {
  //.......ownership............//
  // s is not valid as it has not been declared
  // let s = "hello"; // s is valid from this point on
  // println!("{:?}", s)

  //..... moves current s1 to s2 variable, s1 is dropped at transition....///
  // let s1 = String::from("hello");
  // let s2 = s1;
  // println!("{}, world!", s2)

  //..... cloning is memory expensive but allows for both values to exist...//
  // let s1 = String::from("hello");
  // let s2 = s1.clone();

  // println!("s1 = {}, s2 = {}", s1, s2) // s2 is a clone of s1

  //.... ownership rules .... //
  let s = String::from("hello"); // s comes into scope

  takes_ownership(s);           // s's value moves into the function...
                                // ... and so is no longer valid here

  // println!("{}", s); // creates error value borrowed after move

  let x = 5;                    // x comes into scope

  makes_copy(x);                // x would move into the function,
                                // but i32 is Copy, so it's okay to still
                                // use x afterward
  makes_copy(x) // adds another copy

} // s is now out of scope
  // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
  println!("{}", some_string);
} // Here, some_string goers out of scope and 'drop' is called. The backing
  // memory is freed

fn makes_copy(some_integer: i32) { // some_integer comes into scope
  println!("{}", some_integer)
} // Here, some_integer goes out of scope. Nothing special happens.

// makes_copy(x); //out of scope and stalls whole terminal run
