//use std::io;
//const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // let x = 5;
    // let x = x + 1;

    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }

    // println!("The value of x is: {x}");

    // let mut space = "   ";
    // let space = space.len();

    // let guess: u32 = "42".parse().expect("Not a number!");

    // let x = 2.0;
    // let y: f32 = 3.0;

    // let sum = 5 + 10;

    // let difference = 95.5 - 4.3;

    // let product = 4 * 30;

    // let quatient = 56.7 / 32.2;

    // let truncated = -5 / 3;

    // println!("The value of truncated is: {truncated}");

    // let remainder = 43 % 5;

    // let t = true;

    // let f: bool = false;

    // //tuple
    // let tup: (i32, f64, u8) = (500, 6.4, 1);

    // let (x, y, z) = tup;

    // println!("The value of y is {y}");

    // let five_hundred = tup.0;
    // println!("The value of five_hundred is {five_hundred}");

    // //array
    // let a = [1, 2, 3, 4, 5];

    // let months = [
    //     "January",
    //     "Febrauary",
    //     "March",
    //     "April",
    //     "May",
    //     "June",
    //     "July",
    //     "August",
    //     "September",
    //     "October",
    //     "November",
    //     "December",
    // ];

    // let x = months[0];
    // println!("The value of first month is {x}");

    // let a: [i32;5] = [1,2,3,4,-5];
    
   //  let b = [3;5];

   //  let first = b[0];

   //  let second = b[1];

   //  println!("The value of first  is {first}, value of second {second}");

    // let mut index = String::new();

    // io::stdin().read_line(&mut index).expect("Faild to read line");

    // let index: usize = index.trim().parse().expect("Index entered was not a number");

    // let element = a[index];

    // println!("Eleement at index {index} is equal to {element}");

//     another_function(1);
//
//     let x = {
//         let y = 3;
//         y + 1
//     };
//
//     println! (" x is {x}");
//
//     let z = number(5);
//     println! (" z is {z}");
//
//     let number = 1;
//     if number < 5 {
//         println!("true");
//     } else {
//         println!("false");
//     }
//
//     let number = 5;
//
//     if number != 0 {
//         println!("second true");
//
//     }

// let number = if true {1} else {5};
// let result = loop {
//     println!("number is {number}");
//     if number != 2 {
//         break 2;
//         }
//     };
//
//     println!("result is {result}");



// if number % 4 == 0 {
//     println!("number is divisible by 4");
//     } else if number % 3 == 0 && number % 4 == 0 {
//         println!("number is divisible by 3");
//         } else if number % 2 == 0 {
//             println!("number is divisible by 2")
//             } else {
//                 println!("number is not divisible b 4, 3, or 2");
//                 }


// let mut count = 0;
//
// 'counting_up: loop {
//     println!("count = {count}");
//     let mut remaining = 10;
//     loop {
//         println!("remaining = {remaining}");
//         if remaining == 9 {
//             break;
//             }
//         if count == 3 {
//             break 'counting_up;
//             }
//         remaining -= 1;
//         }
//     count += 1;
//     }
//     println!("End count = {count}");

// let a = [10,20,30,40,50];
// let mut index = 0;
//
// while index < 5 {
//     println!("the value is {}", a[index]);
//     index += 1;
//     }
//
// for element in a {
//     println!("the value is: {element}");
//     }
//
// for num in (-1..4) {
//         println!("{num}!");
//     }

// let mut test = "test!!";
// println!("String test value is {test}");
//
// let mut testString = String::from("testString");
// println!("String testString value is {testString}");
// testString.push_str(", I am test");
// println!("String testString value is {testString}");
// let mut testString2 = testString.clone();
// println!("String testString value is {testString}");
// println!("String testString2 value is {testString2}");
//
// let s = String::from("hello");
// let s1 = take_and_give_ownership(s);
// println! ("String passed after onwership is {s1}.");
//
// let x = 5;
// another_function(x);
// println! ("Number passed after onwership is {x}.");
//
//
// let s2 = String::from("TestSize");
// let (s3, len) = calculate_length(s2);
// println! ("String s3 is {s3} and size is {len}");
//
//
// let mut s4 = String::from("TestSize");
// let len = calculate_byReference_length(&mut s4);
// println! ("String s4 is {s4} and size is {len}");
//
// //pass by reference
//
// let mut s = String::from("hello");
//
//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     println!("{r1} and {r2}");
//     let r3 = &mut s; // BIG PROBLEM
//
//     println!("{}", r3);
//     println!("{r1} and {r2}");


//dangling pointer

//let reference_to_nothing = dangle();


//programming problem: write a function that takes a string of words separated by spaces and returns the first word it finds in that string. If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.


// let hello = &s[0..5];
// println!("hello is {hello}");
//
// let world = &s[6..11];
// println!("world is {world}");
//
// let s = String::from("hello");
//
// let slice = &s[0..2];
// println!("index for slice is {slice}");
//
// let slice = &s[2..];
// println!("index for slice iss {slice}");

// let mut s = String::from("hello world");
//
// let word = first_word(&s);
//
// s.clear();
// println!("first word with space is {word}");

// let s1 = String::from("hello");
// let s2 = &s1;
// let s3 = &s1;
//
// s1.push_str(", world");
// println!("s2 is {s2}, {s3}");
// println!("s2 is {s2}");

// let mut user1 = User {
//     active: true,
//     username: String::from("someusername123"),
//     email: String::from("someone@example.com"),
//     sign_in_count: 1,
//     };
//
// user1.email = String::from("anotheremail@example.com");
//
// println!("user1 email is {}",user1.email);
//
// let user2 = build_user(String::from("testUser"), String::from("testUser@example.com"));
// println!("user2 email is {}",user2.email);
// let user3 = User {
//     username: String::from("testuser3"),
//     email: String::from("testuser3@example.com"),
//     ..user2
//     };
// println!("user3 email is {}",user3.email);
//
// let user4 = User {
//     username: String::from("testuser4"),
//     ..user3
//     };
//
//
// let black = Color(0, 0, 0);
// println!("black Red value is {}",black.0);
//
// let subject = AlwaysEqual;
//
// let width = 30;
// let height = 50;
//
// println!("area of the rectangle is {}", area(width, height));
//
// let rectangle = Rectangle {
//     width: dbg!(30 * 2),
//     height: 50,
//     };
//
// let rectangle1 = Rectangle {
//         width: dbg!(20 * 2),
//         height: 40,
//         };
//
// println!("area of the rectangle using struct {}", areaRect(&rectangle));
// println!("rect1 {rectangle:?}");
// dbg!(&rectangle);
//
// println!("rect1 area is {}", {rectangle.area()});
// println!("rect1 length is {}", {rectangle.length()});
//
// println!("can rectangle hold rectangle1 {}", rectangle.can_hold(&rectangle1));
//
// let sqr = Rectangle::square(50);
// println!("square width and height are {}", sqr.width);
// let ip_four = IpAddrKind::V4(127, 0, 0, 1);
// let ip_six = IpAddrKind::V6(String::from("::1"));
//
// let m = Message::Write(String::from("test"));
//
// m.call();
//
// let some_number = Some(5);
// let some_char = Some('e');
//
// let absent_number : Option<i32> = None;
//
// let x: i8 = 5;
// let y: Option<i8> = Some(5);
//
// let sum = x + y;

let config_max : Option<u8> = None;
if let Some(max) = config_max {
    println!("The maximum is configured to be {max}");
    }
}



#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    }
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    }
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
            }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
            }
        }
    }
//options
// enum Option<T> {
//     None,
//     Some(T),
//     }

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
    }

impl Message {
    fn call(&self) {
        }
    }
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct Color(i32, i32, i32);
struct AlwaysEqual;
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
    }
impl Rectangle {
    fn square(size: u32) -> Self {
       Self {
           width: size,
           height: size,
           }
        }
    fn area(&self) -> u32 {
        self.width * self.height
        }
    fn length(&self) -> u32 {
        (self.width * 2) + (self.height * 2)
        }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
        }
    }
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,

    }
fn areaRect(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
    }
fn area(width: u32, height: u32) -> u32 {
    width * height
    }

fn build_user(username: String, email: String) -> User {
     User {
        active: true,
        username,
        email,
        sign_in_count: 1,
        }
    }

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
            }
        }
    &s[..]
    }

//fn dangle() -> &String {
//     let s = String::from("dangle pointer");
//     &s
//}

fn calculate_length(s: String) -> (String, usize) {
    let size = s.len();
    (s, size)
}

fn calculate_byReference_length(s: &mut String) -> usize {
    s.push_str("test");
    s.len()
}
fn number(x: i32) -> i32 {
    x + 1;
    x
}

fn take_ownership(s: String) {
    println! ("Another function, String passed is {s}.");
}

fn take_and_give_ownership(s: String) -> String {
    println! ("Another function, String passed is {s}.");
    return s;
   println! ("Another function2, String passed is {s}.");

}

fn another_function(x: u32) {
    println! ("Another function, number passed is {x}.");
}
