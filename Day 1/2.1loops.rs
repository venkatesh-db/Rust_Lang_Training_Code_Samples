


fn main()
{
let array: [i32; 5] = [0, 1, 2, 3, 4];

println!("The first element of the array is: {}", array[0]);

let mut counter = 0;
for x in array.iter(){
    println!("The element at index {} is {}", counter, x);
    counter += 1;
}

let data:[i32;3]=[1,2,3];

let mut c = data[0];
println!("{}",data[0]);

}


