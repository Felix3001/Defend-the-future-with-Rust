


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


// fn main() {
//     let x = define_x();
//     println!("{}, world", x);
// }

// fn define_x() -> String {
//     let x = "hello".to_string();
//     x
// }

// fn main() {
//     // for i in 1..=100 {
//     //     println!("{}", i);
//     // }
//     for i in 'a'..='z' {
//         println!("{}", i);  
//     }
//     let heart_eyed_cat = '😻';
//     println!("{}", heart_eyed_cat);
//     let t = true;

//     let f: bool = false; 

// }

// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is: {}", y);
// }

// fn main() {
//     let s = sum(1 , 2);
//     assert_eq!(s, 3);
// }

// fn sum(x: i32, y: i32) -> i32 {
//     x + y
// }


// 函数名和变量名使用蛇形命名法(snake case)，例如 fn add_two() -> {}
// 函数的位置可以随便放，Rust 不关心我们在哪里定义了函数，只要有定义即可
// 每个函数参数都需要标注类型

// fn add(i: i32, j: i32) -> i32 {
//     i + j
// }

// fn main() {
//     // let result = add(1, 2);
//     // println!("{}", result);
//     // println!("hello,world");

//     let mut s = String::from("hello");
//     s.push_str(", world");
//     println!("{}", s);
// }


// fn main() {
//     let x: &str = "hello, world";
//     let y = x;
//     println!("{},{}",x,y);
// }

// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn main() {
//     let mut s = String::from("hello");

//     let a = change(&mut s);
//     println!("{}", a);
// }

// fn change(some_string: &mut String) -> &mut String {
//     some_string.push_str(", world");
//     some_string
// }


fn main() {
    let mut s = String::from("hello");
 
     let r1 = &s;
     let r2 = &s;
     println!("{} and {}", r1, r2);
     // 新编译器中，r1,r2作用域在这里结束
 
     let r3 = &mut s;
     println!("{}", r3);
 } // 老编译器中，r1、r2、r3作用域在这里结束
//    新编译器中，r3作用域在这里结束

// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &mut s;
//     let r2 = &mut s;

//     println!("{}, {}", r1, r2);
// }