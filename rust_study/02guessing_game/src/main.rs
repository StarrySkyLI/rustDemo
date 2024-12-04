use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("猜数游戏");
    let sn = rand::thread_rng().gen_range(1..101);
    loop {
        println!("猜测一个数");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("cant read line");
        let guess:u32 = match guess.trim().parse() {
            Ok(num)=>num,
            Err(_)=>continue
        };
        println!("你猜的数是{}",guess);
        match guess.cmp(&sn) {
            Ordering::Less => println!("猜小了"),
            Ordering::Greater => println!("猜大了"),
            Ordering::Equal => {
                println!("猜对了");
                break;
                }
            }

        }
    }

