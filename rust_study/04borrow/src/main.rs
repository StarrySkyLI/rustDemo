fn main() {
    let mut s1 = String::from("hello,world");
    let len =calculate_length(&mut s1);
    println!("the length of '{}' is {}",s1,len);
    //引用可以不涉及所有权
    //可变引用限制：在特定作用域内，对某一块数据，只能有一个可变引用。可以防止在编译时数据竞争
    {
        let s2 = &mut s1;
    }
    let s3= &mut s1;
    //另外一个限制
//     不可以同时拥有一个可变引用和一个不可变引用，多个不可变引用是可以的
}
fn calculate_length(string: &mut String)-> usize{
    string.push_str(",starry");
    string.len()
}