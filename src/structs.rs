#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
//初始化实例时，每个字段都需要进行初始化

pub fn struct_test(){
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("1993554403@qq.com");
    
    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // User {
    //     ..user1
            // 做了借用
    // };
    user2.email = String::from("wuhaotian.xx@bxxxxxxxe.com");

    // 打印结构体的方法
    println!("{:?}", user1);
    println!("{:#?}", user2);
    dbg!(user1);
}

// 元组结构体
struct Point(i32, i32, i32);

fn tunp_struct() {
    let origin = Point(0, 0, 0);
}