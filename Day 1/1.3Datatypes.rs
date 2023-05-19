


#[derive(Debug)]
enum Fruits {
 Apple,
 Mango
}
fn main() {
 let apple = Fruits::Apple;
 let mango = Fruits::Mango;

 println!("{:?}", apple);
 println!("{:?}", mango);
}