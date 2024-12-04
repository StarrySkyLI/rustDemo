struct Point<T,U> {
    x: T,
    y: U,
}

impl<T,U> Point<T,U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn mixup<V,W>(self,other: Point<V,W>)->Point<T,W>{
        Point{
            x:self.x,
            y:other.y
        }
    }
}
impl Point<i32,i32> {
    fn x1(&self) -> &i32 {
        &self.x
    }
}
fn main() {
    let p_1 = Point { x: 5, y: 10 };
    let p_2 = Point { x: 5.2, y: 10.1 };
    let p_3=Point{x:"a",y:"c"};
    // let p1=Point::x1(&p_1);
    // let p2 =Point::x(&p_2);
    //单态化：在编译的时候将泛型替换为具体类型的过程

    println!("p_1.x = {}", p_1.x1());
    println!("p_2.x = {}", p_2.x());
    let p_4=p_1.mixup(p_3);
    println!("p4.x={},p4.y={}",p_4.x,p_4.y);


}
