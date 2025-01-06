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

    //adding element to a vector while holding a reference
    let mut v = vec![1,2,3,4,5];
    let element = &v[1];
    v.push(7);
     println!("The first element is: {element}");
}
