


fn main() {
    println!("message: {}", hello_string(""));
}
fn hello_string(_: &str) -> &str {  // _: is used to signify, that it is a private pointer variable.
    return "hello jvt";
}