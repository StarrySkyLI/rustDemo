fn main() {
    slice_one();
    slice_two();
}
fn slice_one() {
    let s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];
    let whole = &s[..];

    println!("{},{},{}", hello, world, whole)
}
fn slice_two() {
    let mut s = String::from("Hello World");
    let world_index = first_world(&s);
    println!("{}", world_index)
}
fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn slice_three(){
    let my_string =String::from("hello world");
    let world_index = first_world(&my_string[..]);

    let my_string_literal ="hello world";
    let world_index = first_world(my_string_literal);
}
fn slice_four(){
    let a =[1,2,3,4,5];
    let slice =&a[1..3];
}