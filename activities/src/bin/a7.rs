// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

// * Use an enum with color names as variants
enum ColorNames {
  Red,
  Green,
  Blue,
  Yellow,
}

// * Use a function to print the color name
// * The function must use the enum as a parameter
fn print_color(my_color: ColorNames){
  match my_color {
    ColorNames::Red => println!("Red"),
    ColorNames::Green => println!("Green"),
    ColorNames::Blue => println!("Blue"),
    ColorNames::Yellow => println!("Yellow"),
  }
}

fn main() {
  print_color(ColorNames::Red);
}
