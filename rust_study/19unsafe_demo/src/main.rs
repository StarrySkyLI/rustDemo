fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1: {:?}", r1);
        println!("r2: {:?}",r2);
        dangerous();
    }
    let address = 0x012345usize;
    let r = address as *const i32;






        let mut v = vec![1, 2, 3, 4, 5, 6];

        let r = &mut v[..];

        let (a, b) = r.split_at_mut(3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);


}

unsafe fn dangerous(){
    println!("danger!!!")
}