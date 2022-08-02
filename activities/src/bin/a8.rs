// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

//....1st example......//
// struct ShippingBox {
//   depth: i32,
//   width: i32,
//   height: i32
// };

// let my_box = ShippingBox {
//   depth: 3,
//   width: 2,
//   height: 5,
// };

//......2nd example......//
// struct GroceryItem {
//   stock: i32,
//   price: f64, //floating point used for decimals
// }

// let cereal = GroceryItem {
//   stock: 10,
//   price: 2.99,
// };

// println!("stock: {:?}", cereal.stock);
// println!("stock: {:?}", cereal.price);

// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//

  // Notes:
  // * Use an enum to create different flavors of drinks
enum DrinkFlavor {
  Chocolate,
  Vanilla,
  Strawberry,
}
// * Use a struct to store drink flavor and fluid ounce information
struct DrinkInfo {
  flavor: DrinkFlavor,
  ounces: f64,
}

// * Use a function to print out the drink flavor and ounces
fn print_drink(drink: DrinkInfo) {
  // * Use a match expression to print the drink flavor
  match drink.flavor {
    DrinkFlavor::Chocolate => println!("chocolate"),
    DrinkFlavor::Vanilla => println!("vanilla"),
    DrinkFlavor::Strawberry => println!("strawberry"),
  }
  println!("{:?}", drink.ounces);
}

fn main() {
  let yummy = DrinkInfo {
    flavor: DrinkFlavor::Chocolate,
    ounces: 2.2,
  };
  print_drink(yummy);

}
