 use std::fs::File;
 use std::io::ErrorKind;
fn main() {
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
    let greeting_file_result = File::open("hello.txt");

        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {e:?}"),
                    },
                    other_error => {
                        panic!("Problem opening the file: {other_error:?}");
                        }
                },
            };
}
