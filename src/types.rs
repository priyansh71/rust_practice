pub fn fun(){

    // x = "i32"
    let x = 1;

    //y = "f64"
    let y = 2.5;

    //add explicit types
    let z: i64 = 9249221389231;

    //find max size
    println!("Max i32 is {}", std::i32::MAX);
    println!("Max i64 is {}", std::i64::MAX);

    //boolean
    let is_true = true;
    let is_greater: bool = 10 < 5;

    //char
    let a1 = "A";
    let b1 = "\u{1F601}";
    println!("{:?}",(x,y,z,is_true,is_greater,a1,b1));


}  