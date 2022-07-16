pub fn fun(){

    let x = 1;
    match x {
        1 => println!("Value of x is {}", x),
        2 => println!("Value of x is {}", x),
        _ => println!("x is invalid"),
    }
    
    let y = true;
    let z = true;

    match (y,z) {
        (true,true) => println!("y : true , z : true."),
        (true,false) => println!("y : true , z : false."),
        (false,true) => println!("y : false , z : true."),
        (false,false) => println!("y : false , z : false."),
    }
}