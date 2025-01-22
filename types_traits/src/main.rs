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


struct Point2<T, U> {
    x: T,
    y: U,
    }


