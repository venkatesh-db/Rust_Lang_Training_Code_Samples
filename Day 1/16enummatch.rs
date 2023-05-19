

enum FruitName {
 Apple,
 Orange,
 Strawberry
}
fn fruitdetails(fruit: FruitName) {
 match fruit {
  FruitName::Apple => {
    println!("Apple is a red-orange colour fruit and its really tasty");
   },
   FruitName::Orange => {
    println!("Orange is a orange colour fruit, and its tasts sour and sweet too. ");
   },
   FruitName::Strawberry => {
    println!("Strawberry is my favourite fruit");
   }
 }
}
fn main() {
 fruitdetails(FruitName::Apple);
 fruitdetails(FruitName::Orange);
 fruitdetails(FruitName::Strawberry);
}