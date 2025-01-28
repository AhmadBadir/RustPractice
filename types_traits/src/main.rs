use aggregator::{Summary, Tweet, NewsArticle};
fn main() {
//     let number_list = vec![43, 50, 25, 100, 65];
//
//     let mut largest = &number_list[0];
//
//     for number in &number_list {
//         if number > largest {
//             largest = number;
//             }
//         }
//
//     println!("The largest number is {largest}");
//
//     let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
//
//     let mut largest = &number_list[0];
//
//     for number in &number_list {
//         if number > largest {
//             largest = number;
//             }
//         }
//     println!("The largest number is {largest}");

//extract the above code to a function
let number_list = vec![43, 50, 25, 100, 65];

let result = largest_i32(&number_list);
println!("The largest number is {result}");

let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
let result = largest_i32(&number_list);
println!("The largest number is {result}");

let char_list = vec!['y','m','a','q'];
let result = largest_char(&char_list);
println!("the largest char is {result}");

//generic type of struct
let integer = Point{x:5, y: 10};
let flot = Point{x: 1.0, y: 4.0};

//below code won't compile
//let mix_wont_work = Point { x: 4, y: 4.0};

//mix point
let mix = Point2{x:4, y: 5.0};
let mix2 = Point2{x:4, y: 5};
let mix3 = Point2{x:4.0, y: 5.0};

let p = Point{x: 5, y: 10};
println!("p.x = {}", p.x());


let p = Point{x: 5.7, y: 10.0};
println!("p.x = {}", p.distance_from_origin());

let p1 = Point4{x:5, y:10.0};
let p2 = Point4{x:"Hello", y:'c'};
let p3 = p1.mixup(p2);

println!("p3.x = {}, p3.y {}", p3.x, p3.y);

//monomorphized
let integer = Some(5);
let float = Some(5.0);
//compiler will monomorphized the above code to
let integer = Option_i32::Some(5);
let float = Option_f64::Some(5.0);


//traits

let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
    };
    println!("1 new tweet:{}", tweet.summarize());


//Trait with default implementation
let article = NewsArticle {
    headline: String::from("Penguis win the Stanley Cup Chammpionship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from(" The pittsburge Penguins once again are the best hockey team in the NHL."),
    };

    println!("New article available! {}", article.summarize());
}
//compiler perform monomorphized
enum Option_i32 {
    Some(i32),
    None,
    }
enum Option_f64 {
    Some(f64),
    None,

    }

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
            }
        }
    largest
    }

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
            }
        }
    largest
    }



//NOTE: CODE won't compile as we need to learn about TRAITS instead of defining two functions, we define one function with generic param
// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//             }
//         }
//     largest
//     }

//generic type in struct
struct Point<T> {
    x: T,
    y: T,
    }

// generic type in implementation
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
        }
    }

//implementation that applied only for specific type although struct is generic
// generic type in implementation
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }


struct Point2<T, U> {
    x: T,
    y: U,
    }
//generic type using enum
enum Option<T> {
    Some(T),
    None,
    }
//enum with multiple generic types
enum Option1<T, E> {
    Ok(T),
    Err(E),
    }

//Method using generic types different from its stuct definition
struct Point4<X1, Y1> {
    x:X1,
    y:Y1,
    }

impl<X1, Y1> Point4<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point4<X2, Y2>) -> Point4<X1, Y2> {
        Point4 {
            x: self.x,
            y: other.y,
            }
        }
    }
