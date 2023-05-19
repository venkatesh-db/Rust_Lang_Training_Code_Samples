


fn add_one(i: i32) -> i32 {
    i + 1
}

fn main()
{
let f: fn(i32) -> i32 = add_one;
//let _f = add_one;
let value = f(5);
println!("{}", value);
}
