


use std::collections::HashMap; // importing the libraries

fn main() {

    let mut department = HashMap::new(); // creating the instance
    department.insert("CSE", "Computer Science And Engineering"); // first initialization
    department.insert("IT", "Information Technology"); //second initialization
    department.insert("ME", "Mechanical engineering"); //third initialization
    println!("{:?}", department); //printing the hashmap instance

}

