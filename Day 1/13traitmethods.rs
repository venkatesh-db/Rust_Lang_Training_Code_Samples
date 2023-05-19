

// Create structs
#[derive(Debug)]
struct Cat { name: String, age: u8 }
#[derive(Debug)]
struct Dog { name: String, age: u8 }
// Declare the struct
trait Pet {
 // This new function acts as a constructor
 // allowing us to add additional logic to instantiating a struct
 // This particular method belongs to the trait
 fn new (name: String) -> Self;
// Signature of other functions that belong to this trait
 // we include a mutable version of the struct in birthday
 fn birthday(&mut self);
 fn sound (&self);
}

// We implement the trait for cat
// we define the methods whose signatures were in the trait
impl Pet for Cat {

 fn new (name: String) -> Cat {
   return Cat {name, age: 0};
 }

 fn birthday (&mut self) {
   self.age += 1;
   println!("Happy Birthday {}, you are now {}", self.name, self.age);
 }

 fn sound(&self){
   println!("{} goes meow!", self.name);
 }
}

// We implement the trait for dog
// we only define sound. Birthday and name are already defined
impl Pet for Dog {

 fn new (name: String) -> Dog {
   return Dog {name, age: 0};
 }

 fn birthday (&mut self) {
   self.age += 1;
   println!("Happy Birthday {}, you are now {}", self.name, self.age);
 }

 fn sound(&self){
   println!("{} goes ruff!", self.name);
 }
}

// Notice we define a new method that acts like a constructor. Instead of creating a new Cat like we did in our previous snippet,
//we can just type our new variable!

// When we invoke the constructor, it will use the new implementation of that particular type of struct. 
//Therefore, both Dog and Cat will be able to use the Birthday and Sound functions:

fn main() {
 // Create structs using the Pet new function
 // using the variable typing to determine which
 // implementation to use
 let mut scratchy: Cat = Pet::new(String::from("Scratchy"));
 let mut spot: Dog = Pet::new(String::from("Spot"));

 // using the birthday method
 scratchy.birthday();
 spot.birthday();

 // using the sound method
 scratchy.sound();
 spot.sound();
}