pub fn fun(){
    // print
    println!("Hello from print.rs");

    //formatting
    println!("{} is from {}.","Priyansh","Bikaner");

    //positional arguments
    println!("{0} is from {1} and he likes {2}.",
    "Priyansh", "Bikaner", "Python");

    //named arguments
    println!("{name} likes to watch {game}.",
    name = "Priyansh",
    game = "Football");

    //placeholder traits
    println!("Binary {:b} , Hex {:x} , Octal {:o}",10,10,10);

    //placeholder for debugging
    println!("{:?}", (12, true, "Hello there"));

    //mathematics
    println!("10 + 10 = {}", 10 + 10);
}
