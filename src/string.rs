pub fn slice() {
    let a = String::from("hello,world");
    let b = &a[0..6];
    let c = &a[6..11];
    println!("{}",b);
    println!("{}",c);
    let hole = &a[0..];

    let x = [1, 2, 3, 4, 5];
    let slice = &x[1..3];
    assert_eq!(slice, &[2, 3]);
}