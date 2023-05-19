

fn main()
{

const VALUE: u16= 500;

let tuple = ("hello world", 42, "world", [3,6,9]);

println!("First element is {}", tuple.0);
println!("Second element is {}", tuple.1);
println!("Third element is {}", tuple.2);
let mut counter = 0;
for x in &tuple.3 {
    println!("Element {} of the fourth element is {}", counter, x);
    counter += 1;
}
}