

/*
new() : This function create and new and empty vector. No memory will be allocated by this vector until and unless any element is inserted into the vector.
push() : This function will append an element at the back of a collection(E.g: vector)
remove() : This function retrieves and removes the element in the mentioned index and then shift all the elements to the left.
contains() : This function returns a true if a specific value is matched or found.
len() : This function returns the number of elements in the vector, which is also the length of the vector
*/




fn main() {

  // let mut my_vector = Vec::new();

    let  mut my_vector = vec![1,2,3,4,5];

   my_vector.push(100);
   my_vector.push(200);
   my_vector.push(300);
   my_vector.push(400);
   my_vector.push(500);
   println!("The element in the index 0 is {:?}",my_vector[0]);
   println!("The element in the index 3 is {:?}",my_vector[3]);
   my_vector.remove(1);
    if my_vector.contains(& 5) {
      println!(" 5 is pressent in the vector");
   }
 println!("size of vector is :{}",my_vector.len());
}

