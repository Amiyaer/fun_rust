mod string;
mod tunple;
mod structs;
fn main() {
    println!("Hello, world!?dddd");
    let a = ["1","2","3","4"];
    for (i, m) in a.iter().enumerate() {
        println!("index:{}----value:{}", i, m);
    }
    string::slice();
    tunple::tunple();
    structs::struct_test();
}
