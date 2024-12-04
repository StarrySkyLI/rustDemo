pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
pub trait Iterator2<T>{
    fn next(&mut self) -> Option<T>;
}
struct Counter{}
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
// impl Iterator for Counter {
//     type Item = String;不行上面定义了U32就只能用u32
//
//     fn next(&mut self) -> Option<Self::Item> {
//         None
//     }
// }

impl Iterator2<String> for Counter{
    fn next(&mut self) -> Option<String> {
        None
    }
}