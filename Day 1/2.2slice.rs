


fn main()
{
let array: [i32; 5] = [0, 1, 2, 3, 4];

let slice = &array[0..3];

for x in slice {
	println!("x is {}", x);
}
}