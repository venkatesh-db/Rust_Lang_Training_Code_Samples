
use 


use  std::io ;

fn main(){

println!("enter name");

let mut name = String::new();

io:: stdin().read_line(&mut name);
   
println!("{}", name);

}