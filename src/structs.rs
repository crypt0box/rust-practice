// debug traitをstructに実装しないと構造体はprintできない
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// struct内にメソッドを追加する
impl Rectangle {
    fn create(width: u32, height: u32) -> Self {
        // 構造体のインスタンスを生成s
        Self { width, height }
    }
    fn area(&self) {
        println!("{}", self.width * self.height);
    }
}

pub fn run() {
    let user1 = User {
        username: String::from("cryptobox"),
        email: String::from("example@example.com"),
        sign_in_count: 2,
        active: true,
    };
    let mut user1 = User {
        username: String::from("cryptobox"),
        email: String::from("example@example.com"),
        sign_in_count: 2,
        active: true,
    };
    user1.email = String::from("anotheremail@example.com");
    // 構造体でも所有権の移動がおこる
    // let user2 = user1;
    // println!("{}", user1);
    println!("{:#?}", user1);

    let user2 = buld_user(String::from("user2@xx.com"), String::from("user2"));
    println!("{:#?}", user2);

    let rect = Rectangle::create(20, 20);
    println!("{:#?}", rect);
    rect.area();
}

fn buld_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
