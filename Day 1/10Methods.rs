
pub struct Calculator {}

impl Calculator {
    // Define methods here
}

impl Calculator {
    pub fn add(&self, val1: i32, val2:i32) -> i32 {
        val1 + val2
    }

    pub fn subtract(&self, val1: i32, val2:i32) -> i32 {
        val1 - val2
    }

    pub fn multiple(&self, val1: i32, val2:i32) -> i32 {
        val1 * val2
    }

    pub fn divide(&self, val1: i32, val2:i32) -> f64 {
        val1 as f64 / val2 as f64
    }
}

fn main() {
    let cal = Calculator{};
    println!("{}", cal.add(1,2));
    println!("{}", cal.subtract(1,2));
    println!("{}", cal.multiple(1,2));
    println!("{}", cal.divide(1,2));
}