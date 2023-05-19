
struct vector {
    len:i32,
    cap:i32
}

fn set(v:vector ) -> vector{
  //println!("{} {}",v.cap,v.len)
  v
}

fn main(){

    let obj = vector{cap:12,len:23};
    let ret= set(obj);
     println!("{} {}",ret.cap,ret.len)

     
}