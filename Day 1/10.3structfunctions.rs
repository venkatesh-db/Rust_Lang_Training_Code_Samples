

struct Student {
    name: String,
    school: String,
    roll: u32,
    grade: char
}
fn main() {
    fn student_detail() - > Student {
        let value1 = Student {
            name: String::from("Fitas"),
            school: String::from("Chercher.tech"),
            roll: 01,
            grade: 'A'
        };
        return value1;
    }
    let order = student_detail();
    println!("Student: {} of {} school bearing roll {} has obtained {} grade", order.name, order.school, order.roll, order.grade);
    println!("the details in which roll is less is printed above");
}