use std::ops::Deref;

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T; //定义关联类型

    fn deref(&self) -> &T {
        &self.0
    }
}
fn main() {
    // let x = 5;
    // let y = &x;
    // assert_eq!(5, x);
    // assert_eq!(5, *y);
    let x = 5;
    // let y = Box::new(x);
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m =MyBox::new(String::from("hello"));
    //deref coercion
    //&m  &MyBox<String>
    //deref解析为&String
    //再由String的deref解析为&str
    hello(&m);
    hello("rust")
}
fn hello(name: &str){
    println!("{}",name)
}