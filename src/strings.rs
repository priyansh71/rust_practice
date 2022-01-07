pub fn fun(){

    //Mutable data structure String
    let mut hello = String::from("Hello ");

    //Length of string
    println!("Length is {}.", hello.len());
    
    //pushing characters
    hello.push('t');

    //pushing strings
    hello.push_str("here");

    //capacity in bytes
    println!("Capacity is {} bytes.", hello.capacity());

    //check if empty
    println!("{}", hello.is_empty());

    //check for substring
    println!("{}", hello.contains("there"));

    //replace
    println!("Replacing : {}", hello.replace("there", "world"));

    //loop through string by whitespace
    for word in hello.split_whitespace(){
        println!("{}", word);
    }

    //create string with whitespace
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    s.push('3');
    println!("{}",s);

    //assertion testing , only shows error if false
    assert_eq!(10,s.capacity());
    assert_eq!(3,s.len());

    println!("{:?}",( hello.len(), hello));

}