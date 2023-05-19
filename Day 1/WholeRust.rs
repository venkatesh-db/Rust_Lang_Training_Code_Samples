Data types
let happiness = "job"

i8
i16
i32
i64
i128
isize
:&str 
:String

println!("result value is {}",result);
println!("address {:p}",&a);

// Data segment
// Scope // lifetime 
// pass by value address


fn (){
    
    // Scope of the storage 
   // stack 
   //  heap 

   // Scope // lifetime 
  // pass by value address

}

struct {
// life time
}

//  stack 
//  heap 
// Scope of the storage 
// Scope // lifetime 
// pass by value address


fn  (){

    let b = Box::new(var_i32); 
    //heap
    println!("b = {}", b);

    let values = Box::new([0; size])

    // Scope // lifetime 
    // pass by value address

    let mut vector = Vec::new();

    vector.push(20);
    vector.insert(0, 10); // insert at the beginning


}
// 1. heap  2. stack  3. assign value to heap
//  stack to stack 
//  heap to stack 
//  data to stack

// DS HEap 

use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// Scope // lifetime 
// pass by value address

// oops
// struct
// method
// composition
// trait
// generic struct , traits 

// Library code

// Error handling

// condiitional compilation

// Smart Pointers

// Iterators
 
// OS 

// testing

fn refer(  n:&str , c:&str){

    println!( "{} {} ", n,c);
     
 }
 
 fn heap() ->String{
     
     "dinneis ritchie".to_string()
 }
 
 fn stat() ->&'static str {
 
     "rust"
 
     
 }
 
 
 struct gang {
     
     name :&'static str ,
     friends: String ,
     
 }
 
 
 
 fn main() {
 
     println!("Hello, world!");
     
     let name ="venkatesh" ;   // read only memory 
     
     let coderrange:String = "babu".to_string(); // heap memory
     
     refer(name,&coderrange );
     
     println!("{}",coderrange);
     
     let ret = heap();
     
     let pointer:&str = &ret;
     
     println!("{} {}",ret ,pointer);
     
     let data = stat();    
     
     println!("{}",data );
     
     let hero= gang{ name:"blockchain", friends: "ronit faraz dheeraj umang".to_string()};
 }






use std::collections::HashMap;

static house:[i32;3]= [1,2,3];

static global:&str = "me";



fn stack()->i32{
    
    let a:i32= 5;
     println!("address {:p}",&a);
    a
  
}


fn main(){
    
println!("coderrange");
    
let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);

scores.insert(String::from("Yellow"), 50);

let name :&str = "coderrange";

let name1 :&str = name; 

println!("{} {}",name ,name1);

let ret :i32 = stack();

println!("{}",ret);

let mut values = Box::new([0; 10]);

values[0]=1;

println!("{:?}",values);
    
}