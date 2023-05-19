
struct Vec2 {
    data: Box<[isize]>,
    length: usize,
    capacity: usize
}

impl Vec2 {
    fn new() -> &Vec2 {
        let v = Vec2 {
            data: Box::new([]),
            length: 0,
            capacity: 0
        };
        return &v;
    }
}

fn main () {}