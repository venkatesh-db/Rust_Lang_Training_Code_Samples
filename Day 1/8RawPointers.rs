

#![allow(unused)]
fn main() {

let my_num: Box<i32> = Box::new(10);
let my_num_ptr: *const i32 = & *my_num;


let mut my_speed: Box<i32> = Box::new(88);

let my_speed_ptr: *mut i32 = &mut *my_speed;

println!("{:?}",my_speed_ptr);

unsafe {
    
    println!("{:?}",*my_speed_ptr );
 
     *my_speed_ptr =34;

     println!("{:?}",*my_speed_ptr );
    
}

my_speed=Box::new(55);

println!("{}",my_speed);

 let x = 25;
 let r = &x;



// println!("{:?}",r );

//println!("{:}",my_num );
println!("{:?}",my_num_ptr );

unsafe {
    
    println!("{:?}",*my_speed_ptr );
}

}