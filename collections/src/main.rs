fn main() {
    let v : Vec<i32> = Vec::new();

//using vec! macro
    let v1 = vec![1,2,3];
//modifying vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    //Reading element from vector
    let v2 = vec![1,2,3,4,5];

    let third: &i32 = &v2[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v2.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
        }

    //accessing out of bound - program won't compile
    let v = vec![1,2,3,4,5];
    //let does_not_exists = &v[100];
    //this way it will compile as it returns an Option
    let does_not_exists = v.get(100);

    //adding element to a vector while holding a reference, the below code won't compile as we borrow
    let mut v = vec![1,2,3,4,5];
  //  let element = &v[1];
   // v.push(7);
    // println!("The first element is: {element}");

     //iterate over vector
     let mut v = vec![100,32,5,100];

     for i in &mut v {
         println!{"element i in v is {i}"};
         *i += 50;
         println!{"element i in v is {i}"};
         }
     for i in &mut v {
              println!{"element i in v is {i}"};
              }

     //enum vector
     enum SpeardsheetCell {
         Int(i32),
         Float(f64),
         Text(String),
         }

     let row = vec![SpeardsheetCell::Int(3),
     SpeardsheetCell::Text(String::from("blue")),
     SpeardsheetCell::Float(10.12),
     ];
    {
     //drop element in vector
     let v = vec![1,2,3,4,5];
     //do sstuff with v
     } // <- v goes out of scope and is freed here
}
