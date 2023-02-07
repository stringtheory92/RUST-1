// use rand::Rng;
use rand::prelude::*;
use std::collections::HashMap;
// pub mod rectangle;
// use core::num;
mod garden;
use garden::vegetables::asparagus::{Asparagus};

// use rectangle::{Rectangle, print_stuff};
// // pub mod ip;
// // use Message::{call};

// #[derive(Debug)]
// enum Message {
//     Quit, 
//     Move {x: i32, y: i32},
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// impl Message {
//     fn call(&self) {
//         println!("{:?}", &self);
//     }
// }
// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
// }
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }


fn main() {
    // let sq = Rectangle::square(100);

    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };
    // let rect2 = Rectangle {
    //     width: 10,
    //     height: 40,
    // };
    // let rect3 = Rectangle {
    //     width: 60,
    //     height: 45,
    // };

    // println!("sq can_hold rect1? {}", sq.can_hold(&rect1));
    // print_stuff();
    // let m = Message::Write(String::from("hello")).call();
    // println!("I have {} cents!", value_in_cents(Coin::Penny) + value_in_cents(Coin::Quarter(UsState::Alabama)));

//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);

//     println!("six: {:?}, none: {:?}", six, none);
// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i+1),
//     }
    let mut a = Asparagus::new(String::from("green"), 4);
    a.grow();
    // let mut a = garden::vegetables::asparagus::Asparagus::new(String::from("green"), 4);
    println!("{:#?}", a);

    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("Map: {:?}", map);

}