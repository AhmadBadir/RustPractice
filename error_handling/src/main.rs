use std::fs::{self, File};
use std::io::{self, Read, ErrorKind};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    //panic - unrecoverable errors
    //panic!("Crash and burn");
    //println!("Hello, world!");

    let v = vec![1,2,3];
    //program will panice due to index out of bound
    //v[99];


    //recoverable errors using panic (program will crash)


//     let greeting_file_result = File::open("hello.txt");
//
//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => panic!("Problem opening the file: {error:?}"),
//         };

    //recoverable errors using non panic approach
//     let greeting_file_result = File::open("hello.txt");
//
//         let greeting_file = match greeting_file_result {
//             Ok(file) => file,
//             Err(error) => match error.kind() {
//                 ErrorKind::NotFound => match File::create("hello.txt") {
//                     Ok(fc) => fc,
//                     Err(e) => panic!("Problem creating the file: {e:?}"),
//                     },
//                     other_error => {
//                         panic!("Problem opening the file: {other_error:?}");
//                         }
//                 },
//             };

    //alternative way using match with Result<T, E>

//         let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
//             if error.kind() == ErrorKind::NotFound {
//                 File::create("hello.txt").unwrap_or_else(|error| {
//                     panic!("Problem creating the file: {error:?}");
//                     })
//                 } else {
//                     panic!("Problem opening the file: {error:}");
//                     }
//             });

    //short cut for panic on error
    //let greeting_file = File::open("hello.txt").unwrap();
   // let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");


// ? operator wont work in main function
let greeting_file = File::open("hello.txt")?;
Ok(())
}

//propagating errors
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
        };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
        }
    }

//propagating shortcut
fn read_user_from_file_file_shortcut() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut user = String::new();
    username_file.read_to_string(&mut user)?;
    Ok(user)
    }
//propagating shortcut- shorter version
fn read_user_from_file_file_shortcut_shorter() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
    }

//propagating shortcut- even shorter version
fn read_user_from_file_file_shortcut_even_shorter() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
    }

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}