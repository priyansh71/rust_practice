pub fn fun(){
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    println!("{:?}", some_number);
    println!("{:?}", some_string);
    println!("{:?}", absent_number);

    let x = 5;
    let y = None;
    println!("{:?}", x+y.unwrap_or(0)); 
    // We cannot add a None to a number, so we unwrap if its possible to an i32, else consiser y as 0.

    let x = Some(5);
    let y : Option<i32> = None;
    println!("{:?}", plus_one(x));
    println!("{:?}", plus_one(y));
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _ => None, // This is the default if no match is found.
    }
    // We need to match explicitly on each value, otherwise we get a compile error.
}