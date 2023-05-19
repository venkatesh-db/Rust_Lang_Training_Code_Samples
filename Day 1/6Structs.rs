
// This debug attribute implements fmt::Debug which will allow us
// to print the struct using {:?}

#[derive(Debug)]

//Note that we’re using the derive attribute, which we’ll cover in detail later, to automate the implementation of
// certain traits on our struct. Since we derive the debug trait, we can print the entire struct using {:?}:

// declaring a struct

struct Cat {
 // name property typed as a String type
 name: String,
 // age typed as unsigned 8 bit integer
 age: u8
}

fn main() {

 // create string object with cat's name
 let catname = String::from("Scratchy");
 // Create a struct instance and save in a variable
 let scratchy = Cat{ name: catname, age: 4 };
 
 // using {:?} to print the entire struct
 println!("{:?}", scratchy);

 // using individual properties in a String
 println!("{} is {} years old!", scratchy.name, scratchy.age);

}

//[#derive(hash)]: converts the struct into a hash
//[#derive(clone)]: adds a clone method to duplicate the struct
//[#derive(eq)]: implements the eq trait, setting equality as all properties having the same value
