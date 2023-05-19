#[derive(Debug)]
struct Db{
  empid:i32,    
}

fn main(){

   let code :String = "rust".to_string();

   owner(&code);

   owner(&code);   

   let sai = Db{empid:1235};

   let b = Box::new(&sai);

   println!("{:?}",b);

}

fn owner( c :&String){

    println!("{}",c);

}