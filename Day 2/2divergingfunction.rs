
/*

Diverging Functions in Rust are used for crashing the current execution of a thread. 

panic! is one of the diverging functions which can crash the current thread, and it prints a given text to the console.

Since panic!() function will crash the current execution of the thread, therefore, it will not return anything.

*/

fn main()
{
	diverge_function();
}

fn diverge_function() -> ! {
   panic!("This function does not return anything, but crashes the current thread execution");
}