

fn main() {
   let absent_number: Option<i32>=None;
   let five =Some(5);
   let six =plus_one(five);
   let none =plus_one(None);
   println!("{:#?}",six)

}
fn plus_one(x:Option<i32>)->Option<i32>{
   match x {
      None => None,
      Some(i) => Some(i+1)
   }
}