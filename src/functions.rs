pub fn fun(){
    greeting("Hello", "Ramesh");

    //Bind function values
    let sum = add(5,5);
    println!("{}",sum);

    //Closure
    let n3 = 10;
    let perform_on_nums = |n1 : i32, n2 : i32| n1 - n2 + n3;
    println!("Subtraction is {}.",perform_on_nums(6,3))
}

fn greeting(greet : &str, name : &str){
    println!("{} {}, nice to meet you.", greet, name);
}

fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}