pub trait Shape {
    fn area(&self) -> f32;
}

pub struct Circle {
    pub radius: f32,
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        2 as f32 * 3.14 * self.radius * self.radius
    }
}

pub struct Rectangle {
    pub length: f32,
    pub width: f32,
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.length * self.width
    }
}

pub struct Square {
    side: f32,
}

impl Shape for Square {
    fn area(&self) -> f32 {
        self.side * self.side
    }
}

fn main()
{
    let circle = Circle { radius: 10.36 };
    println!("Area of circle is {}", circle.area());
    let rectangle = Rectangle { length: 10.59, width: 20.22 };
    println!("Area of Rectangle is {}", rectangle.area());
    let square = Square { side: 5.96 };
    println!("Area of Square is {}", square.area());
}