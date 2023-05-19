
pub struct Calculator {}

impl Calculator {
    pub fn add(val1: i32, val2:i32) -> i32 {
        val1 + val2
    }

    pub fn subtract(val1: i32, val2:i32) -> i32 {
        val1 - val2
    }

    pub fn multiple(val1: i32, val2:i32) -> i32 {
        val1 * val2
    }

    pub fn divide(val1: i32, val2:i32) -> f64 {
        val1 as f64 / val2 as f64
    }
}

fn main() {
    println!("{}", Calculator::add(1,2));
    println!("{}", Calculator::subtract(1,2));
    println!("{}", Calculator::multiple(1,2));
    println!("{}", Calculator::divide(1,2));
}