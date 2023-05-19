
fn main( ){

 let   code = "rust".to_string();

 let ret =pass( code);

 pass(ret );   // always one refernce to a memory 

}

fn pass( code :String) -> String  {

     println!(" { } ", code );
     
     return code
}