

// #[warn(dead_code)]
// enum Direction {
//     East,
//     West,
//     North,
//     South,
// }

// fn main() {
//     let dire = Direction::South;
//     match dire {
//         Direction::East => println!("East"),
//         Direction::North | Direction::South => {
//             println!("South or North");
//         },
//         _ => println!("West"),
//     };
// }


// enum IpAddr {
//    Ipv4,
//    Ipv6
// }

// fn main() {
//     let ip1 = IpAddr::Ipv6;
//     let ip_str = match ip1 {
//         IpAddr::Ipv4 => "127.0.0.1",
//         _ => "::1",
//     };

//     println!("{}", ip_str);
// }


// #![allow(unused)]
// fn main() {
//     let v = Some(3u8);
//     match v {
//         Some(3) => println!("three"),
//         _ => (),
//     }
// }

// 这两种匹配对于新手来说，可能有些难以抉择，但是只要记住一点就好：当你只要匹配一个条件，且忽略其他条件时就用 if let ，否则都用 match。

enum MyEnum {
    Foo,
    Bar
}

fn main() {
    let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
}
