fn main() {
    let mut s1 = String::from("hello");
    s1.push_str("world");//加字符串
    println!("{}",s1);
    let mut s2 = String::new();
    s2.push('l');//加单个字符
    println!("{}",s2);
    let data = "hello";
    let s3 = data.to_string();
    let s4 = "hello".to_string();
    //字符串拼接
    //1
    let s5=s1+&s2;
    println!("{}",s5);
    //2
    let s =format!("{}-{}-{}",s4,s2,s3);
    println!("{}",s);
    //切割字符串
    let hello ="hello";
    let s=&hello[0..4];
    println!("{}",s);
    //遍历字符串
    let s =String::from("hello");
    for c in s.chars(){
        println!("c: {}",c);
    }
    for b in s.bytes(){
        println!("b: {}",b);
    }
}
