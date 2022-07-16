pub fn fun(){

    let args: Vec<String> = std::env::args().collect();
    let command = &args[1];
    let name = "Priyansh";
    let status = "100%";

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else{
        println!("That's not a valid command.")
    }
}