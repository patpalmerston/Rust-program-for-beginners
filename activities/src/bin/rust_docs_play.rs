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
  let s1 = String::from("hello");
  let s2 = s1.clone();

  println!("s1 = {}, s2 = {}", s1, s2) // s2 is a clone of s1


} // s is now out of scope
