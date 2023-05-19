
/*

The deref trait is a method which is inbuilt in the Rust standard library, which is used to implement 
the deref() method. This method deref() borrows a self and returns a reference to an inner data.

A program to show the use of the Deref Trait.

In the below program, we have created a structure called MyMemory which has a generic return type T. 
Inside the structure, we have a variable called value1 which is of T Type

*/


struct MyMemory < T > {
  value1: T,
}
use::std::ops::Deref;
impl < T > Deref
for MyMemory < T > {
  type Target = T;
  fn deref( & self) - > & T {
    &self.value1
  }
}
fn main() {
  let value2 = MyMemory {
    value1: 10
  };
  println!("{}", *(value2.deref()));
}

/*
Now, the trait deref is implemented on the structure MyMemory of MyMemory type.
The reference of the variable value1 is returned using the deref() method.

Now, in the function main() when we create an instance called value2 of the structure 
MyMemory, we assign the variable value1 with 10. Now, when we use the method deref() 
with the variable value2. It will automatically dereference the instance. 
That is, it will print the data which is pointed by value2, which is value1, i.e 10.
*/