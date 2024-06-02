pub fn tunple(){
    let a: (i32, f32) = (3,2.0);
    let (x, y) = a;
    println!("{:?}-{}-{}",a.1,x,y);
}

#[warn(dead_code)]
fn t() {
    let tup = (1, 6.4, "hello");

    // 填空
    let (x,z,y) = tup;

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);
}