

/*
open() : This is a static method and opens a file in read-only mode.
create() : This is a static method that creates a new file or opens the existing file in write-only mode if the file_name already exists but destroys the earlier content of the file.
remove_file() : This method removes the file from the file system, however, it may not be deleted exactly from the storage device.
append() : This method sets the option for the append mode of a file.
write_all() :
read_to_string :
*/

use std::io::Write;
fn main() {
    let mut my_file = std::fs::File::create("my_document.txt").expect("creation failed");
    my_file.write_all("Hello Chercher.tech".as_bytes()).expect("write failed");
    my_file.write_all("	 Learning is Fun".as_bytes()).expect("write failed");
    println!("The tag line of Chercher.tech has been added, open the file to see :)");
}