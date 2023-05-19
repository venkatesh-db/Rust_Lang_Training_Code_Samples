
fn main(){

    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;
    
    // Error! The type of a variable can't be changed.
  //  mutable = true;
    
    // Variables can be overwritten with shadowing.
    let mutable = true;

}