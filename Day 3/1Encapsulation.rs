
pub struct AverageMarks {
    list: Vec<i32>,
    average: f64,
}

impl AverageMarks {

    pub fn add(&mut self, marks: i32) {
        self.list.push(marks);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(marks) => {    // Some(value), a tuple struct that wraps a value with type T.
                self.update_average();
                Some(marks)
            }
            none => none,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

fn main(){
    let mut list: Vec<i32> =vec![];
    let mut averagemarks =AverageMarks{ list: vec![], average: 0.0 };
    averagemarks.add(25);
    averagemarks.add(16);
    averagemarks.add(85);
    println!("average marks of student {}",averagemarks.average());
    averagemarks.remove();
    println!("average marks of student {}",averagemarks.average());
}