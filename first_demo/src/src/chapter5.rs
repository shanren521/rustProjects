// Using Structs to Structure Related Data
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

pub fn chapter5_1() {
    // Defining and Instantiating Structs
    // structs are more flexible tuples

    let mut user = User{
        active: true,
        username: String::from("user123"),
        email: String::from("user123@gmail.com"),
        sign_in_count: 1
    };
    // 如果整个结构是可变的，可以通过.属性名改变某个字段的值
    user.email = String::from("user1234@gmail.com");

    let user1 = User{
        active: true,
        username: String::from("user123"),
        email: String::from("user123@gmail.com"),
        sign_in_count: 1
    };

    // 通过一个instance 更新另一个instance
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("test@gmail.com"),
        sign_in_count: user1.sign_in_count,
    };
    
    // 通过..来快速更新相同的数据到新的instance
    let user3 = User {
        email: String::from("hello@gmail.com"),
        ..user2
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwayEqual;

}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
    
}

// Using Tuple Structs Without Named Fields to Create Different Types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 定义没有任何字段
struct AlwayEqual;


pub fn chapter5_2() {
    // An Example Program Using
    let width1 = 30;
    let height1 = 50;
    println!("The area of the rectangle is {} square pixels", area(width1, height1));

    // Refactoring with Tuples
    let rect1 = (30 ,50);
    println!("The area of the rectangle is {} square pixels", area_refactor(rect1));

    let rect2 = Rectangle {
        width: 30,
        height: 50
    };
    println!("The area of the rectangle is {} square pixels", area_new(&rect2));

    // #[derive(Debug)]在struct上加上这个，可以实现debug
    let rect3 = Rectangle {
        width: 30,
        height: 50
    };
    println!("rect3 is {rect3:?}");  // 必须加上:?和在Rectangle结构体上加上#[derive(Debug)]，否则会报`Rectangle` doesn't implement `Debug`

    let scale = 2;
    let rect4 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect4);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_refactor(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn area_new(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

pub fn chapter5_3() {
    // Method Syntax
    let rect1 = Rectangle {width: 30, height: 50};
    println!("The area of the rectangle is {} square pixels", rect1.area());

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {width: 40, height: 40};
    let rect3 = Rectangle {width: 40, height: 60};
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

}

// Defining Methods
impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}




