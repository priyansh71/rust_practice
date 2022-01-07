pub fn fun(){

    // define types and length
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    
    //only possible if it is mutatable
    numbers[2] = 20;

    //get a single value
    println!("{}",numbers[0]);

    //add on an element
    numbers.push(6);

    //remove the last element
    numbers.pop();

    //length
    println!("{}",numbers.len());

    //print
    println!("The vector : {:?}",numbers);

    //get slice of an array
    let slice: &[i32] = &numbers[0..2];
    println!("{:?}",slice);

    // looping through
    for x in numbers.iter(){
        print!("{} ",x);
    }

    println!("");

    //loop and mutate the whole vector
    for x in numbers.iter_mut(){
        *x = *x * *x;
    }

    println!("{:?}",numbers);

}