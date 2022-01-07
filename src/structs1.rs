// Tradtional struct
struct RGB1{
    red : u8,
    green : u8,
    blue : u8
}

// Tuple struct
struct RGB2(u8,u8,u8);

pub fn fun(){
    // 1.
    let mut color1 = RGB1{
        red : 0,
        green : 0,
        blue : 0
    };

    println!("White : ({},{},{})", color1.red, color1.green, color1.blue);
    color1.red = 255;
    println!("Red : ({},{},{})", color1.red, color1.green, color1.blue);

    //2.
    let mut color2 = RGB2(255,255,255);

    println!("Black : ({},{},{})", color2.0, color2.1, color2.2);
    color2.0 = 0;
    println!("SkyBlue : ({},{},{})",color2.0, color2.1, color2.2 );

} 