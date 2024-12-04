fn main() {
    println!("Hello, world!");
    another_function(5,6);

    let x=6;//语句没有返回值，表达式有返回值
    let y = {
        let x =1;
        x+3//加了;就是语句了
    };//{}里面的就是表达式
    let z=five(y);
    println!("z {}",z)
}
fn another_function(x: i32,y: i32){
    println!("another one x {}",x);
    println!("another one y {}",y)
}
fn five(x:i32)->i32{
    x+5
}