

// fn main() {
//     let my_name = String::from("Pascal");
//     greet(my_name);
//   }
  
//   fn greet(name: String) {
//     println!("Hello, {}!", name);
//   }


// #![allow(unused)]
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }



// fn main() {
//     let user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("someusername123"),
//         active: true,
//         sign_in_count: 1,
//     };

//     println!("{}", user1.email)
// }

// #[derive(Debug)]
//  struct File {
//    name: String,
//    data: Vec<u8>,
//  }

//  fn main() {
//    let f1 = File {
//      name: String::from("f1.txt"),
//      data: Vec::new(),
//    };

//    let f1_name = &f1.name;
//    let f1_length = &f1.data.len();

//    println!("{:?}", f1);
//    println!("{} is {} bytes long", f1_name, f1_length);
//  }


// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("rect1 is {:?}", rect1);
// }


// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };

//     dbg!(&rect1);
// }




// fn main() {
//  for i in 1..4 {
//      if i == 2 {
//          continue;
//      }
//      println!("{}", i);
//  }
// }


// fn main() {
//     for i in 1..4 {
//         if i == 2 {
//             continue;
//         }
//         println!("{}", i)
//     }
        
// }


// fn main() {
//     let mut i = 5;
//     while i >= 1 {
//         println!("{}", i);
//         i = i - 1;
//         println!("{}", i);
//     }
// }

// fn main() {
//     let condition = true;
//     let number = if condition {
//         5
//     } else {
//         6
//     };

//     println!("The value of number is: {}", number);
// }

fn main() {
    let n = 6;

    if n % 4 == 0 {
        println!("number is divisible by 4");
    } else if n % 3 == 0 {
        println!("number is divisible by 3");
    } else if n % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}



