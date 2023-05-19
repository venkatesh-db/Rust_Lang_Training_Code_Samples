

fn main() {
 
   let my_tuple:(i32,f64,u8,char,&str, bool) = (-35,100.2, 35,'a', "hello", false);
   println!("{:?}",my_tuple);
   

 let tuple = (-3, 4.9, 'a');
 println!("The unsigned integer is :{:?}", tuple .0);
 println!("The floating point number is :{:?}", tuple .1);
 println!("The character is  :{:?}", tuple .2);


    let original_tuple: (char, i32) = ('A', 30);
    println!("before destructuring");
    println!("{:?}", original_tuple);
    let (grade, pass) = original_tuple;
    println!("after destructuring");
    println!("Got a Grade {} , and it is a pass mark: {} ", grade, pass);


}


