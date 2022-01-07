pub fn fun(){
    let mut count = 0;

    // regular loop
    loop{
        count += 1;
        println!("Number is {}", count);

        if count == 20{
            break;
        }
    }

    // while loop FizzBuzz
    while count <= 100{
        if count % 15 == 0{
            println!("FizzBuzz");
        }
        else if count % 3 == 0{
            println!("Fizz");
        }
        else if count % 5 == 0{
            println!("Buzz");
        }
        else{
            println!("{}", count);
        }

        count += 1;
    }

    // for range loop FizzBuzz
    for i in 0..100{
        if i % 15 == 0{
            println!("FizzBuzz");
        }
        else if i % 3 == 0{
            println!("Fizz");
        }
        else if i % 5 == 0{
            println!("Buzz");
        }
        else{
            println!("{}", i);
        }
    }
}