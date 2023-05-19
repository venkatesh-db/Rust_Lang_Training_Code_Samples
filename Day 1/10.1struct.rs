

struct Student {
  name: String,
  school: String,
  roll: u32,
  grade: char

}
fn main() {
  let value = Student {
    name: String::from("Fitas"),
    school: String::from("Chercher.tech"),
    roll: 01,
    grade: 'A'

  };
  println!("Student: {} of {} school bearing roll {} has obtained {} grade", value.name, value.school, value.roll, value.grade);
}