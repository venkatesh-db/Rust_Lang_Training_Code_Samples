-fn my_function(value: i32, f: &Fn(i32) -> i32) -> i32 {
    println!("{}", f(value));
    value
}
fn multiply(value: i32) -> i32 {
    value * value
}

:=|  | {

}

fn main() {
    my_function(5, &multiply);
}