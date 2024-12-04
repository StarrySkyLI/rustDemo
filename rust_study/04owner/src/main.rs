fn main() {
    let mut s = String::from("hello,world");
    take_ownership(s);//s被move了,所有权交给函数了
    // println!("{}",s);//报错consider changing this parameter type
    // in function `take_ownership` to 04borrow instead if owning the value isn't necessary
    let x = 5;
    make_copy(x);
    println!("x {}", x);

    let s1=give_ownership();
    println!("{}",s1);
    let s2=String::from("hello");
    let s3 =takes_and_gives_back(s2);
    println!("{}",  s3);
}
fn take_ownership(some_string: String) {
    println!("{}", some_string)
}
fn make_copy(some_num: i32) {
    println!("{}", some_num)
}
fn give_ownership()->String{
    let some_string =String::from("hello");
    some_string

}
fn takes_and_gives_back(a_string:String)->String{
    a_string
}