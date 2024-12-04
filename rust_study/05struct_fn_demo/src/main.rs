#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }
}
impl Rectangle {
    //关联函数
    //可以在impl块里面定义不把self作为第一个参数的函数，他们叫关联函数（不是方法）
    // 例如：String::from
    //     关联函数常用于构造器（例子）
    //     ::符号
    //-关联函数
    //模块创建的命名空间
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            length: size,
        }
    }
}
fn main() {
    let s = Rectangle::square(20);
    let rect1 = Rectangle {
        width: 30,
        length: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        length: 40,
    };
    let rect3 = Rectangle {
        width: 30,
        length: 55,
    };
    println!("{}", rect1.area());
    println!("{:#?}", rect1);
    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect1.can_hold(&rect3));
}
