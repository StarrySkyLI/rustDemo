fn main() {
    let condition = true;

    let number = if condition { 5 } else { 6 };

    println!("number {}", number);
    println!("{}", the_loop(0));
    the_while(3);
    the_for_one();
    the_for_two();

}
fn the_loop(mut counter: i32) -> i32 {
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    result
}
fn the_while(mut num: i32) {
    while num != 0 {
        println!("{}!", num);
        num = num - 1;
    }
    println!("LiftOFF!")
}
fn the_for_one(){
    let  a =[1,2,3,4,5];
    for num in a.iter(){
        println!("the num is {}",num)
    }
}
fn the_for_two(){
    for num in(1..4).rev(){
        println!("{}!",num)
    }
    println!("liftOFF")
}