enum Options<T> {
    Some(T),
    None,
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn if_let(x: Option<i32>) {
    if let Some(x) = Some(5) {
        println!("{}", x);
    }
}