fn main() {
    let num_list = vec![34, 44, 54, 64, 4];
    let result = largest(&num_list);
    println!("largest {}", result);
    let char_list = vec!['a', 'b', 'c', 'q'];
    let result = largest(&char_list);
    println!("largest {}", result);
    let string_list = vec![String::from("hello"),String::from("world")];
    let result = largest(&string_list);
    println!("largest {}", result);
}
fn largest<T: PartialOrd + Clone>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}