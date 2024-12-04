use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;

fn main()->Result<(),Box<dyn Error>> {
    let result = read_username_from_file();
    let f =File::open("hello.txt")?;
    Ok(())
}
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}