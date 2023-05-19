

#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn new(name: &str, age: i32) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }
}

fn main() {
    let people = vec![
        Person::new("John", 26),
        Person::new("Kyle", 22),
        Person::new("Tommy", 17),
    ];
    
    println!("{:#?}", people);
}