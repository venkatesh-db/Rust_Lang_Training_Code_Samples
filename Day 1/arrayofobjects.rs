

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Person {
    pub name: &'static str,
    pub age: u32
}

const PEOPLE: &'static [Person] = &[
  Person {
    name: "Jhon",
    age: 26,
  },
  Person {
    name: "Kyle",
    age: 22,
  },
  Person {
    name: "Tommy",
    age: 17,
  },
];


fn main(){
    
}