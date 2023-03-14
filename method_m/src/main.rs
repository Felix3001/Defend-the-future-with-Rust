



// #![allow(unused)]
// fn main() {
// struct Circle {
//     x: f64,
//     y: f64,
//     radius: f64,
// }

// impl Circle {
//     // new是Circle的关联函数，因为它的第一个参数不是self，且new并不是关键字
//     // 这种方法往往用于初始化当前结构体的实例
//     fn new(x: f64, y: f64, radius: f64) -> Circle {
//         Circle {
//             x: x,
//             y: y,
//             radius: radius,
//         }
//     }

//     // Circle的方法，&self表示借用当前的Circle结构体
//     fn area(&self) -> f64 {
//         std::f64::consts::PI * (self.radius * self.radius)
//     }
// }
// }


#[derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32
}

impl Rectangle {
    
    fn area(&self) -> u32 {
        self.width * self.height
    }
}


fn main() {
    let rect1 = Rectangle {width:30, height:50};

    println!("The area of the rectangle is {} square pixels.",rect1.area());
}


