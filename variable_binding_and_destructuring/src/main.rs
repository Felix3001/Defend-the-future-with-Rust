


// fn main() {
//     // let x = 5;
//     // println!("{}", x);
//     // x = 6;
//     // println!("{}", x);

//     let mut x = 5;
//     println!("{}", x);
//     x = 6;
//     println!("{}", x);
// }


// fn main() {
//     let x = 5;
//     let _y = 6;
//     println!("{}", x)
// }

// fn main() {
//     let (a, mut b) = (true, false);
//     println!("a = {:?}, b = {:?}", a, b);

//     b = true;
//     assert_eq!(a, b);
// }


// struct Struct {
//     e: i32
// }

// fn main() {
//     let (a, b, c, d, e);
//     (a, b) = (1, 2);
//     [c, .., d, _] = [1, 2, 3, 4, 5];
//     Struct { e, .. } = Struct { e: 5 };
//     assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);


// }

// shadowing

// fn main() {
//     let x = 5;
//     // 在main函数的作用域内对之前的x进行遮蔽
//     let x = x + 1;

//     {
//         // 在当前的花括号作用域内，对之前的x进行遮蔽
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {}", x);
//     }

//     println!("The value of x is: {}", x);
// }

// fn main() {
//     let x: i32 = 10;
//     let y: i32 = 20;
//     {
//         let y: i32 = 5;
//         println!("The value of x is {} and value of y is {}", x, y);
//     }
//     println!("The value of x is {} and value of y is {}", x, y); 
// }


fn main() {
    let x = define_x();
    println!("{}, world", x);
}

fn define_x() -> String {
    let x = "hello".to_string();
    x
}