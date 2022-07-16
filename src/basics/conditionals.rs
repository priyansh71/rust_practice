pub fn fun(){

    let age : u8 = 17;

    if age >= 18{
        println!("What would you like to drink?")
    }else {
        println!("No Kimi, you will not have the drink!")
    }

    //Short-hand
    let is_of_age = if age >= 18 {true} else {false};
    println!("{}", is_of_age)

}

