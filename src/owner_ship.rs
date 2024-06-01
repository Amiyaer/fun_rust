fn owner(){
    let a = 1;
    let b = a;// 栈上分配的值可以直接拷贝，a不会失效

    let s1 = String::from("hello"); // 值的所有权
    let s2 = s1;// 所有权变成s2，s1不再生效
    // s1被丢弃了
    let s3 = s2.clone(); // s3深度克隆了s2，s2未失效

    let x: &str = "hello world"; // 仅仅拷贝引用
    let y = x; // xy都生效
}