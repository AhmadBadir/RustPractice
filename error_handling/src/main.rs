fn main() {
    //panic - unrecoverable errors
    //panic!("Crash and burn");
    //println!("Hello, world!");

    let v = vec![1,2,3];
    //program will panice due to index out of bound
    //v[99];


    //recoverable errors
    use std::fs::File;
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
        };
}
