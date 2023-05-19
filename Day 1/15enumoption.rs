
/*
In Rust, Option is an in-built enum.
This enum can have only two variants Some and None.
A function can use the Option enum in such a way that, if the function returns 
a value then the Some variant with the data will be displayed. 
Otherwise, if the function does not return any value, Null, will be displayed.

enum Option<data-type> {
   Some(data),        //used to return a value.
   None                  // indicate no return of values.
   }
*/


fn main() {
 let result = check(30);
 println!("{:?}", result);
}
fn check(number: i32) -> Option < bool > {
 if number > 5 {
  Some(true)
 } else {

  None
 }
}