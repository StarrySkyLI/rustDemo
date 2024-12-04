enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(12);
    v.push(13);
    v.push(14);
    let mut w = vec![1, 2, 3]; //vector的另一种初始化方法,只能存同一种类型
    let t: &i32 = &v[2]; //获取vector的第一种方法，超过范围会panic
    println!("{}", t);
    match w.get(2) { //获取vector的第二种方法，超过范围会null
        None => {
            println!("no 3 element");
        }
        Some(t) => {
            println!("the 3 element is {}", t);
        }
    }
    for i in &w {
        println!("{}", i)
    }
    for i in &mut w {
        *i += 50;
    }
    for i in w {
        println!("{}", i)
    }
    //要存多种类型，使用vec+enum
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
