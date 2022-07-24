use std::fs::File;
use std::io;
use std::io::Read;

fn main(){
  let name = read_username_from_file();
  let name =  match name {
      Ok(n) => println!("{}",n),
      Err(e)=> panic!("{}",e),
    };
  }
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
//use std::fs::File;
//use std::io::{self, Read};
//fn main() {
//fn read_username_from_file() -> Result<String, io::Error> {
//let f = File::open("hello.txt");
//
//let mut f = match f {
//Ok(file) => file,
//Err(e) => return Err(e),
//};
//
//let mut s = String::new();
//
//match f.read_to_string(&mut s) {
//Ok(_) => Ok(s),
//Err(e) => Err(e),
//}
//}
//read_username_from_file();
//}
