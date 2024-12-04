use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // let f = File::open("hello.txt");
    // let f =match f {
    //    Ok(file) => file,
    //    Err(error) => match error.kind() {
    //          ErrorKind::NotFound =>match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Error Create file: {:?}",e),
    //          },
    //       other_error=>panic!("Error Create file: {:?}",other_error),
    //       },
    // };
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Error Create file: {:?}", error)
            })
        } else {
            panic!("Error Create file: {:?}", error)
        }
    });

    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("error{:?}", error)
        }
    };
    //unwrap:是match那一堆的简洁方法，信息不能自定义
    let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("cant open file");


}
