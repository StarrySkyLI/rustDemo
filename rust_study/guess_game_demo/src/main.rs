#[derive(Debug)]
pub struct Guess{
    value :i32,
}
impl Guess {
    pub fn new(value:i32)->Guess{
        if value<1||value>100{
            panic!("out of range {}",value)
        }
        Guess{value}
    }
    pub fn value(&self)->i32{
        self.value
    }
}
fn main() {
    let guess="10";
    let guess:i32=match guess.trim().parse() {
        Ok(num)=>num,
        Err(e)=>panic!("error {:?}",e),
    };
    let guess =Guess::new(guess);
    println!("{:?}",guess)
}
