


#[derive(Debug)]
enum Result {
 Pass,
 Fail
}
# [derive(Debug)]
struct Student {
 name: String,
 result: Result
}
fn main() {
 let student1 = Student {
  name: String::from("John"),
  result: Result::Pass
 };
 let student2 = Student {
  name: String::from("Riya"),
  result: Result::Fail
 };
 println!("{:?}", student1);
 println!("{:?}", student2);
}